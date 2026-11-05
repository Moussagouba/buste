use crate::analysis::confidence::calculate_confidence_score;
use crate::analysis::features::FeatureEngine;
use crate::analysis::probability::ProbabilityEngine;
use crate::analysis::value::ValueEngine;
use crate::database::DbPool;
use crate::error::Result;
use crate::models::prediction::ValueBetSignal;
use crate::strategy::strategy::BettingStrategy;
use chrono::Utc;
use tracing::info;
use uuid::Uuid;

pub struct SignalGenerator;

impl SignalGenerator {
    pub async fn scan_and_generate(
        pool: &DbPool,
        strategy: &BettingStrategy,
        current_bankroll: f64,
    ) -> Result<Vec<ValueBetSignal>> {
        // Query active markets and latest odds
        let rows = sqlx::query!(
            r#"
            SELECT 
                e.id as event_id,
                e.sport_name,
                e.league_name,
                e.home_team,
                e.away_team,
                e.start_time,
                m.id as market_id,
                m.market_type,
                o.outcome_name,
                o.price as bookmaker_odds,
                o.bookmaker_margin
            FROM events e
            JOIN markets m ON m.event_id = e.id
            JOIN odds o ON o.market_id = m.id
            WHERE e.status = 'SCHEDULED' 
              AND m.status = 'ACTIVE'
              AND o.is_active = TRUE
            "#
        )
        .fetch_all(pool)
        .await?;

        let mut signals = Vec::new();

        for r in rows {
            let hours_until = (r.start_time - Utc::now()).num_minutes() as f64 / 60.0;
            if hours_until < 0.0 {
                continue;
            }

            let margin = r.bookmaker_margin.unwrap_or(0.05);
            let features = FeatureEngine::extract_features(
                &r.sport_name,
                &r.home_team,
                &r.away_team,
                margin,
                hours_until,
            );

            // Compute true probabilities from features
            let probs = ProbabilityEngine::compute_poisson_probabilities(
                features.expected_home_goals,
                features.expected_away_goals,
            );

            let model_prob = match r.outcome_name.as_str() {
                "1" => probs.home_win,
                "X" => probs.draw,
                "2" => probs.away_win,
                "Over" => probs.over_2_5,
                "Under" => probs.under_2_5,
                "Yes" => probs.btts_yes,
                "No" => probs.btts_no,
                _ => continue,
            };

            let eval = ValueEngine::evaluate_outcome(
                &r.outcome_name,
                r.bookmaker_odds,
                model_prob,
                strategy.config.filters.min_ev,
                strategy.config.kelly_fraction,
            );

            let confidence = calculate_confidence_score(
                &r.sport_name,
                &r.league_name,
                hours_until,
                margin,
                r.bookmaker_odds,
            );

            if strategy.config.filters.is_accepted(
                &r.sport_name,
                &r.market_type,
                eval.expected_value,
                eval.bookmaker_odds,
                confidence,
            ) {
                let stake_pct = eval.kelly_stake_pct.min(strategy.config.max_bet_pct);
                let stake_amount = (current_bankroll * stake_pct).max(1.0);

                // Upsert prediction into database
                let prediction_id: Uuid = sqlx::query_scalar!(
                    r#"
                    INSERT INTO predictions (
                        event_id, market_id, outcome_name, model_name, model_probability,
                        bookmaker_odds, fair_odds, expected_value, edge, confidence_score,
                        kelly_stake_pct, is_value_bet, features_used
                    ) VALUES ($1, $2, $3, 'poisson_ensemble_v1', $4, $5, $6, $7, $8, $9, $10, TRUE, $11)
                    RETURNING id
                    "#,
                    r.event_id,
                    r.market_id,
                    r.outcome_name,
                    eval.model_probability,
                    eval.bookmaker_odds,
                    eval.fair_odds,
                    eval.expected_value,
                    eval.edge,
                    confidence,
                    stake_pct,
                    serde_json::to_value(&features).ok()
                )
                .fetch_one(pool)
                .await?;

                signals.push(ValueBetSignal {
                    prediction_id,
                    event_id: r.event_id,
                    market_id: r.market_id,
                    sport_name: r.sport_name,
                    league_name: r.league_name,
                    match_name: format!("{} vs {}", r.home_team, r.away_team),
                    start_time: r.start_time,
                    market_type: r.market_type,
                    outcome_name: r.outcome_name,
                    bookmaker_odds: eval.bookmaker_odds,
                    fair_odds: eval.fair_odds,
                    model_probability: eval.model_probability,
                    implied_probability: eval.implied_probability,
                    expected_value: eval.expected_value,
                    edge: eval.edge,
                    confidence_score: confidence,
                    recommended_stake_pct: stake_pct,
                    recommended_stake_amount: stake_amount,
                });
            }
        }

        info!("Signal generator detected {} actionable value betting signals", signals.len());
        Ok(signals)
    }
}

