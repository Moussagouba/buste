use crate::error::{AppError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskLimits {
    pub max_bet_pct: f64,              // Max stake as % of bankroll (e.g. 5%)
    pub max_active_exposure_pct: f64,  // Total pending bets as % of bankroll (e.g. 25%)
    pub max_drawdown_stop_loss_pct: f64,// Halt paper trading if drawdown reaches e.g. 25%
    pub max_odds: f64,
}

impl Default for RiskLimits {
    fn default() -> Self {
        Self {
            max_bet_pct: 0.05,
            max_active_exposure_pct: 0.25,
            max_drawdown_stop_loss_pct: 0.25,
            max_odds: 5.0,
        }
    }
}

pub struct RiskValidator;

impl RiskValidator {
    pub fn validate_order(
        limits: &RiskLimits,
        current_bankroll: f64,
        pending_exposure: f64,
        stake: f64,
        odds: f64,
        current_drawdown_pct: f64,
    ) -> Result<()> {
        if current_drawdown_pct >= limits.max_drawdown_stop_loss_pct {
            return Err(AppError::RiskViolation(format!(
                "Trading halted by Stop-Loss: Current drawdown {:.2}% exceeds threshold {:.2}%",
                current_drawdown_pct * 100.0,
                limits.max_drawdown_stop_loss_pct * 100.0
            )));
        }

        if stake > current_bankroll {
            return Err(AppError::InsufficientFunds {
                required: stake,
                available: current_bankroll,
            });
        }

        let max_allowed_stake = current_bankroll * limits.max_bet_pct;
        if stake > max_allowed_stake {
            return Err(AppError::RiskViolation(format!(
                "Stake {:.2} exceeds maximum single bet limit of {:.2} ({:.1}% of bankroll)",
                stake,
                max_allowed_stake,
                limits.max_bet_pct * 100.0
            )));
        }

        let total_exposure_after = pending_exposure + stake;
        let max_exposure = current_bankroll * limits.max_active_exposure_pct;
        if total_exposure_after > max_exposure {
            return Err(AppError::RiskViolation(format!(
                "Total active exposure {:.2} would exceed maximum portfolio risk limit of {:.2}",
                total_exposure_after,
                max_exposure
            )));
        }

        if odds > limits.max_odds {
            return Err(AppError::RiskViolation(format!(
                "Odds {:.2} exceed safety cap of {:.2}",
                odds, limits.max_odds
            )));
        }

        Ok(())
    }
}

