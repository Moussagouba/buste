use crate::strategy::filters::StrategyFilters;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyConfig {
    pub name: String,
    pub description: String,
    pub kelly_fraction: f64,
    pub max_bet_pct: f64,
    pub filters: StrategyFilters,
}

impl Default for StrategyConfig {
    fn default() -> Self {
        Self {
            name: "Quarter Kelly +EV Standard".to_string(),
            description: "Fractional 0.25 Kelly betting on verified positive expectation sports markets".to_string(),
            kelly_fraction: 0.25,
            max_bet_pct: 0.05,
            filters: StrategyFilters::default(),
        }
    }
}

pub struct BettingStrategy {
    pub config: StrategyConfig,
}

impl BettingStrategy {
    pub fn new(config: StrategyConfig) -> Self {
        Self { config }
    }
}

