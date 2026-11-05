use crate::database::DbPool;
use crate::error::Result;
use crate::risk::bankroll::BankrollManager;
use chrono::Utc;
use rand::Rng;
use std::time::Duration;
use tracing::{info, warn};
use uuid::Uuid;

pub struct SettlementEngine;

impl SettlementEngine {
    pub async fn run_settlement_worker(pool: DbPool, interval_secs: u64) {
        info!("Starting Paper Betting settlement worker (interval: {}s)...", interval_secs);
        let mut interval = tokio::time::interval(Duration::from_secs(interval_secs));

        loop {
            interval.tick().await;
            if let Err(e) = Self::settle_pending_bets(&pool).await {
                warn!("Error during bet settlement cycle: {}", e);
            }
        }
    }

    pub async fn settle_pending_bets(pool: &DbPool) -> Result<usize> {
        let account = BankrollManager::get_primary_account(pool).await?;

        // Find pending bets where event start_time is in the past or status is FINISHED
        let pending = sqlx::query!(
            r#"
            SELECT 
                b.id as bet_id,
                b.outcome_name,
                b.odds_placed,
                b.stake_amount,
                b.potential_return,
                e.id as event_id,
                e.home_team,
                e.away_team,
                e.home_score,
                e.away_score,
                e.status as event_status,
                e.start_time,
                m.market_type,
                m.parameter as market_param
            FROM paper_bets b
            JOIN events e ON e.id = b.event_id
            JOIN markets m ON m.id = b.market_id
            WHERE b.status = 'PENDING'
            "#
        )
        .fetch_all(pool)
        .await?;

        if pending.is_empty() {
            return Ok(0);
        }

        let mut settled_count = 0;
        let mut rng = rand::thread_rng();

        for b in pending {
            // For testing & simulation: if event is scheduled in the past or > 10s old, settle it!
            let is_ready = b.event_status == "FINISHED" || (Utc::now() - b.start_time).num_seconds() > 0;

            if !is_ready {
                continue;
            }

            // Resolve score (if not yet in DB, simulate realistic football/basketball outcome)
            let (home_score, away_score) = match (b.home_score, b.away_score) {
                (Some(h), Some(a)) => (h, a),
                _ => {
                    let h = rng.gen_range(0..=3);
                    let a = rng.gen_range(0..=3);
                    // Update event record
                    sqlx::query!(
                        "UPDATE events SET home_score = $1, away_score = $2, status = 'FINISHED', updated_at = NOW() WHERE id = $3",
                        h, a, b.event_id
                    )
                    .execute(pool)
                    .await?;
                    (h, a)
                }
            };

            // Evaluate market outcome
            let won = match b.market_type.as_str() {
                "1X2" => match b.outcome_name.as_str() {
                    "1" => home_score > away_score,
                    "X" => home_score == away_score,
                    "2" => away_score > home_score,
                    _ => false,
                },
                "OVER_UNDER" => {
                    let total = (home_score + away_score) as f64;
                    let param = b.market_param.unwrap_or(2.5);
                    match b.outcome_name.as_str() {
                        "Over" => total > param,
                        "Under" => total < param,
                        _ => false,
                    }
                }
                "BOTH_TEAMS_SCORE" => match b.outcome_name.as_str() {
                    "Yes" => home_score > 0 && away_score > 0,
                    "No" => home_score == 0 || away_score == 0,
                    _ => false,
                },
                _ => false,
            };

            let status = if won { "WON" } else { "LOST" };
            let profit_loss = if won {
                b.potential_return - b.stake_amount
            } else {
                -b.stake_amount
            };

            // Simulated closing odds to measure CLV (Closing Line Value)
            let closing_odds = b.odds_placed * rng.gen_range(0.95..=1.05);
            let clv = (b.odds_placed / closing_odds) - 1.0;

            sqlx::query!(
                r#"
                UPDATE paper_bets
                SET status = $1,
                    profit_loss = $2,
                    closing_odds = $3,
                    closing_line_value = $4,
                    settled_at = NOW()
                WHERE id = $5
                "#,
                status,
                profit_loss,
                closing_odds,
                clv,
                b.bet_id
            )
            .execute(pool)
            .await?;

            BankrollManager::record_bet_settled(
                pool,
                account.id,
                b.bet_id,
                status,
                b.stake_amount,
                b.potential_return,
                profit_loss,
            )
            .await?;

            info!(
                "Settled paper bet {} on {} vs {}: {} ({}-{}) -> Profit: €{:.2}",
                b.bet_id, b.home_team, b.away_team, status, home_score, away_score, profit_loss
            );

            settled_count += 1;
        }

        Ok(settled_count)
    }
}

