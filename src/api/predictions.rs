use crate::api::AppState;
use crate::error::Result;
use crate::models::prediction::Prediction;
use crate::risk::bankroll::BankrollManager;
use crate::strategy::signals::SignalGenerator;
use axum::{extract::State, response::IntoResponse, Json};
use serde_json::json;

pub async fn list_predictions(State(state): State<AppState>) -> Result<impl IntoResponse> {
    let predictions = sqlx::query_as!(
        Prediction,
        r#"
        SELECT 
            id, event_id, market_id, outcome_name, model_name, model_probability,
            bookmaker_odds, fair_odds, expected_value, edge, confidence_score,
            kelly_stake_pct, is_value_bet, features_used, created_at
        FROM predictions
        ORDER BY created_at DESC
        LIMIT 100
        "#
    )
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(json!({
        "status": "success",
        "data": predictions,
    })))
}

pub async fn list_value_bets(State(state): State<AppState>) -> Result<impl IntoResponse> {
    let account = BankrollManager::get_primary_account(&state.pool).await?;
    let signals = SignalGenerator::scan_and_generate(&state.pool, &state.strategy, account.current_balance).await?;

    Ok(Json(json!({
        "status": "success",
        "count": signals.len(),
        "data": signals,
    })))
}

