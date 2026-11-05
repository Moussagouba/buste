use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Event {
    pub id: Uuid,
    pub external_id: String,
    pub sport_name: String,
    pub league_name: String,
    pub home_team: String,
    pub away_team: String,
    pub start_time: DateTime<Utc>,
    pub status: String,
    pub home_score: Option<i32>,
    pub away_score: Option<i32>,
    pub period_scores: Option<serde_json::Value>,
    pub raw_metadata: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateEventDto {
    pub external_id: String,
    pub sport_name: String,
    pub league_name: String,
    pub home_team: String,
    pub away_team: String,
    pub start_time: DateTime<Utc>,
    pub status: String,
    pub raw_metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventWithMarkets {
    #[serde(flatten)]
    pub event: Event,
    pub markets: Vec<super::MarketWithOptions>,
}

