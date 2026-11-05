use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Prediction {
    pub id: Uuid,
    pub event_id: Uuid,
    pub market_id: Uuid,
    pub outcome_name: String,
    pub model_name: String,
    pub model_probability: f64,
    pub bookmaker_odds: f64,
    pub fair_odds: f64,
    pub expected_value: f64,
    pub edge: f64,
    pub confidence_score: f64,
    pub kelly_stake_pct: f64,
    pub is_value_bet: bool,
    pub features_used: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueBetSignal {
    pub prediction_id: Uuid,
    pub event_id: Uuid,
    pub market_id: Uuid,
    pub sport_name: String,
    pub league_name: String,
    pub match_name: String,
    pub start_time: DateTime<Utc>,
    pub market_type: String,
    pub outcome_name: String,
    pub bookmaker_odds: f64,
    pub fair_odds: f64,
    pub model_probability: f64,
    pub implied_probability: f64,
    pub expected_value: f64,
    pub edge: f64,
    pub confidence_score: f64,
    pub recommended_stake_pct: f64,
    pub recommended_stake_amount: f64,
}

