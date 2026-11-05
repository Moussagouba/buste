use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PaperBet {
    pub id: Uuid,
    pub prediction_id: Option<Uuid>,
    pub event_id: Uuid,
    pub market_id: Uuid,
    pub outcome_name: String,
    pub odds_placed: f64,
    pub stake_amount: f64,
    pub potential_return: f64,
    pub status: String, // PENDING, WON, LOST, VOID, CANCELLED
    pub profit_loss: Option<f64>,
    pub closing_odds: Option<f64>,
    pub closing_line_value: Option<f64>,
    pub strategy_tag: Option<String>,
    pub placed_at: DateTime<Utc>,
    pub settled_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaceBetRequest {
    pub prediction_id: Option<Uuid>,
    pub event_id: Uuid,
    pub market_id: Uuid,
    pub outcome_name: String,
    pub odds: f64,
    pub stake: f64,
    pub strategy_tag: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaperBetWithEventDetails {
    #[serde(flatten)]
    pub bet: PaperBet,
    pub sport_name: String,
    pub league_name: String,
    pub home_team: String,
    pub away_team: String,
    pub match_status: String,
    pub final_score: Option<String>,
}

