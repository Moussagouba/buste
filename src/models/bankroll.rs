use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct BankrollAccount {
    pub id: Uuid,
    pub name: String,
    pub currency: String,
    pub initial_balance: f64,
    pub current_balance: f64,
    pub peak_balance: f64,
    pub total_staked: f64,
    pub total_profit: f64,
    pub max_drawdown_pct: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct BankrollTransaction {
    pub id: Uuid,
    pub account_id: Uuid,
    pub paper_bet_id: Option<Uuid>,
    pub transaction_type: String,
    pub amount: f64,
    pub balance_after: f64,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct BankrollSnapshot {
    pub id: Uuid,
    pub account_id: Uuid,
    pub balance: f64,
    pub equity: f64,
    pub total_staked: f64,
    pub net_profit: f64,
    pub roi_pct: f64,
    pub win_rate_pct: f64,
    pub drawdown_pct: f64,
    pub active_bets_count: i32,
    pub settled_bets_count: i32,
    pub recorded_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceDashboardSummary {
    pub current_bankroll: f64,
    pub initial_bankroll: f64,
    pub currency: String,
    pub net_profit: f64,
    pub total_staked: f64,
    pub roi_pct: f64,
    pub win_rate_pct: f64,
    pub current_drawdown_pct: f64,
    pub max_drawdown_pct: f64,
    pub active_bets_count: i64,
    pub settled_bets_count: i64,
    pub won_bets_count: i64,
    pub lost_bets_count: i64,
    pub average_odds: f64,
    pub average_clv_pct: f64,
    pub active_value_bets_count: i64,
}

