use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyFilters {
    pub min_ev: f64,
    pub max_ev: f64,
    pub min_odds: f64,
    pub max_odds: f64,
    pub min_confidence: f64,
    pub allowed_sports: Vec<String>,
    pub allowed_market_types: Vec<String>,
}

impl Default for StrategyFilters {
    fn default() -> Self {
        Self {
            min_ev: 0.03, // At least +3% EV
            max_ev: 0.35, // Filter out erroneous/suspicious odds outliers
            min_odds: 1.30,
            max_odds: 4.50,
            min_confidence: 0.65,
            allowed_sports: vec![
                "Football".to_string(),
                "Basketball".to_string(),
                "Tennis".to_string(),
            ],
            allowed_market_types: vec![
                "1X2".to_string(),
                "OVER_UNDER".to_string(),
                "BOTH_TEAMS_SCORE".to_string(),
            ],
        }
    }
}

impl StrategyFilters {
    pub fn is_accepted(
        &self,
        sport: &str,
        market_type: &str,
        ev: f64,
        odds: f64,
        confidence: f64,
    ) -> bool {
        if ev < self.min_ev || ev > self.max_ev {
            return false;
        }

        if odds < self.min_odds || odds > self.max_odds {
            return false;
        }

        if confidence < self.min_confidence {
            return false;
        }

        if !self.allowed_sports.is_empty()
            && !self.allowed_sports.iter().any(|s| s.eq_ignore_ascii_case(sport))
        {
            return false;
        }

        if !self.allowed_market_types.is_empty()
            && !self.allowed_market_types.iter().any(|m| m.eq_ignore_ascii_case(market_type))
        {
            return false;
        }

        true
    }
}

