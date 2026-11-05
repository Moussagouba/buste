use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Market {
    pub id: Uuid,
    pub event_id: Uuid,
    pub external_market_id: Option<String>,
    pub market_type: String, // 1X2, OVER_UNDER, BOTH_TEAMS_SCORE, HANDICAP
    pub name: String,
    pub parameter: Option<f64>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketWithOptions {
    #[serde(flatten)]
    pub market: Market,
    pub odds: Vec<super::Odds>,
}

