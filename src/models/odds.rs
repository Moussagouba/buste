use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Odds {
    pub id: Uuid,
    pub market_id: Uuid,
    pub outcome_name: String, // 1, X, 2, Over, Under, Yes, No
    pub price: f64,
    pub implied_probability: f64,
    pub bookmaker_margin: Option<f64>,
    pub is_active: bool,
    pub recorded_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOddsDto {
    pub market_id: Uuid,
    pub outcome_name: String,
    pub price: f64,
    pub implied_probability: f64,
    pub bookmaker_margin: Option<f64>,
}

