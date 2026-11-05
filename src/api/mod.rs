pub mod bets;
pub mod dashboard;
pub mod events;
pub mod predictions;
pub mod routes;
pub mod sources;

use crate::collector::MelbetClient;
use crate::config::AppConfig;
use crate::database::DbPool;
use crate::strategy::strategy::BettingStrategy;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub pool: DbPool,
    pub config: AppConfig,
    pub melbet: Arc<MelbetClient>,
    pub strategy: Arc<BettingStrategy>,
}
