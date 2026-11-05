use crate::api::AppState;
use crate::error::Result;
use crate::models::bet::{PaperBetWithEventDetails, PlaceBetRequest};
use crate::risk::limits::RiskLimits;
use crate::simulation::paper_betting::PaperBettingEngine;
use crate::simulation::settlement::SettlementEngine;
use axum::{extract::State, response::IntoResponse, Json};
use serde_json::json;

pub async fn list_bets(State(state): State<AppState>) -> Result<impl IntoResponse> {
    let bets = sqlx::query!(
        r#"
        SELECT 
            b.id, b.prediction_id, b.event_id, b.market_id, b.outcome_name,
            b.odds_placed, b.stake_amount, b.potential_return, b.status,
            b.profit_loss, b.closing_odds, b.closing_line_value, b.strategy_tag,
            b.placed_at, b.settled_at,
            e.sport_name, e.league_name, e.home_team, e.away_team, e.status as match_status,
            e.home_score, e.away_score
        FROM paper_bets b
        JOIN events e ON e.id = b.event_id
        ORDER BY b.placed_at DESC
        LIMIT 100
        "#
    )
    .fetch_all(&state.pool)
    .await?;

    let list: Vec<serde_json::Value> = bets
        .into_iter()
        .map(|b| {
            let final_score = match (b.home_score, b.away_score) {
                (Some(h), Some(a)) => Some(format!("{} - {}", h, a)),
                _ => None,
            };

            json!({
                "id": b.id,
                "prediction_id": b.prediction_id,
                "event_id": b.event_id,
                "market_id": b.market_id,
                "outcome_name": b.outcome_name,
                "odds_placed": b.odds_placed,
                "stake_amount": b.stake_amount,
                "potential_return": b.potential_return,
                "status": b.status,
                "profit_loss": b.profit_loss,
                "closing_odds": b.closing_odds,
                "closing_line_value": b.closing_line_value,
                "strategy_tag": b.strategy_tag,
                "placed_at": b.placed_at,
                "settled_at": b.settled_at,
                "sport_name": b.sport_name,
                "league_name": b.league_name,
                "home_team": b.home_team,
                "away_team": b.away_team,
                "match_status": b.match_status,
                "final_score": final_score,
            })
        })
        .collect();

    Ok(Json(json!({
        "status": "success",
        "data": list,
    })))
}

pub async fn place_bet(
    State(state): State<AppState>,
    Json(payload): Json<PlaceBetRequest>,
) -> Result<impl IntoResponse> {
    let limits = RiskLimits {
        max_bet_pct: state.config.max_bet_pct,
        max_odds: state.config.max_odds,
        ..Default::default()
    };

    let bet = PaperBettingEngine::execute_order(&state.pool, payload, &limits).await?;

    Ok(Json(json!({
        "status": "success",
        "message": "Paper bet successfully placed",
        "data": bet,
    })))
}

pub async fn trigger_settlement(State(state): State<AppState>) -> Result<impl IntoResponse> {
    let count = SettlementEngine::settle_pending_bets(&state.pool).await?;

    Ok(Json(json!({
        "status": "success",
        "message": format!("Settlement cycle completed. Settled {} bets.", count),
        "settled_count": count
    })))
}

