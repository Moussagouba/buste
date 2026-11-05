use crate::api::AppState;
use crate::error::Result;
use crate::models::bankroll::PerformanceDashboardSummary;
use crate::risk::bankroll::BankrollManager;
use crate::simulation::backtest::BacktestEngine;
use axum::{extract::State, response::IntoResponse, Json};
use serde_json::json;

pub async fn get_dashboard_summary(State(state): State<AppState>) -> Result<impl IntoResponse> {
    let account = BankrollManager::get_primary_account(&state.pool).await?;

    let bet_stats = sqlx::query!(
        r#"
        SELECT 
            COUNT(*) FILTER (WHERE status = 'PENDING') as "active_count!",
            COUNT(*) FILTER (WHERE status IN ('WON', 'LOST', 'VOID')) as "settled_count!",
            COUNT(*) FILTER (WHERE status = 'WON') as "won_count!",
            COUNT(*) FILTER (WHERE status = 'LOST') as "lost_count!",
            COALESCE(AVG(odds_placed), 0.0) as "avg_odds!",
            COALESCE(AVG(closing_line_value), 0.0) as "avg_clv!"
        FROM paper_bets
        "#
    )
    .fetch_one(&state.pool)
    .await?;

    let value_bets_count = sqlx::query_scalar!(
        r#"
        SELECT COUNT(*)
        FROM predictions
        WHERE is_value_bet = TRUE
        "#
    )
    .fetch_one(&state.pool)
    .await?
    .unwrap_or(0);

    let net_profit = account.total_profit;
    let total_staked = account.total_staked;
    let roi_pct = if total_staked > 0.0 {
        (net_profit / total_staked) * 100.0
    } else {
        0.0
    };

    let total_decided = bet_stats.won_count + bet_stats.lost_count;
    let win_rate_pct = if total_decided > 0 {
        (bet_stats.won_count as f64 / total_decided as f64) * 100.0
    } else {
        0.0
    };

    let current_drawdown_pct = if account.peak_balance > 0.0 {
        ((account.peak_balance - account.current_balance) / account.peak_balance).max(0.0) * 100.0
    } else {
        0.0
    };

    let summary = PerformanceDashboardSummary {
        current_bankroll: account.current_balance,
        initial_bankroll: account.initial_balance,
        currency: account.currency,
        net_profit,
        total_staked,
        roi_pct,
        win_rate_pct,
        current_drawdown_pct,
        max_drawdown_pct: account.max_drawdown_pct * 100.0,
        active_bets_count: bet_stats.active_count,
        settled_bets_count: bet_stats.settled_count,
        won_bets_count: bet_stats.won_count,
        lost_bets_count: bet_stats.lost_count,
        average_odds: bet_stats.avg_odds,
        average_clv_pct: bet_stats.avg_clv * 100.0,
        active_value_bets_count: value_bets_count,
    };

    Ok(Json(json!({
        "status": "success",
        "data": summary,
    })))
}

pub async fn get_bankroll_history(State(state): State<AppState>) -> Result<impl IntoResponse> {
    let txs = sqlx::query!(
        r#"
        SELECT id, transaction_type, amount, balance_after, created_at
        FROM bankroll_transactions
        ORDER BY created_at ASC
        LIMIT 200
        "#
    )
    .fetch_all(&state.pool)
    .await?;

    let points: Vec<serde_json::Value> = txs
        .into_iter()
        .map(|t| {
            json!({
                "timestamp": t.created_at,
                "balance": t.balance_after,
                "amount": t.amount,
                "type": t.transaction_type,
            })
        })
        .collect();

    Ok(Json(json!({
        "status": "success",
        "data": points,
    })))
}

pub async fn run_backtest_handler(State(state): State<AppState>) -> Result<impl IntoResponse> {
    let account = BankrollManager::get_primary_account(&state.pool).await?;
    let result = BacktestEngine::run_historical_backtest(
        &state.pool,
        &state.strategy,
        account.initial_balance,
    )
    .await?;

    Ok(Json(json!({
        "status": "success",
        "data": result,
    })))
}

