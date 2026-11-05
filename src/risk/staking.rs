use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StakingMethod {
    FixedAmount(f64),
    FixedPercentage(f64),
    FractionalKelly { fraction: f64, max_pct: f64 },
}

pub struct StakingPlan;

impl StakingPlan {
    pub fn calculate_stake(
        method: &StakingMethod,
        bankroll: f64,
        odds: f64,
        model_prob: f64,
    ) -> f64 {
        if bankroll <= 0.0 || odds <= 1.0 {
            return 0.0;
        }

        match method {
            StakingMethod::FixedAmount(amount) => amount.min(bankroll),
            StakingMethod::FixedPercentage(pct) => (bankroll * pct).min(bankroll),
            StakingMethod::FractionalKelly { fraction, max_pct } => {
                let b = odds - 1.0;
                let q = 1.0 - model_prob;
                let full_kelly = ((b * model_prob) - q) / b;

                if full_kelly <= 0.0 {
                    0.0
                } else {
                    let kelly_pct = (full_kelly * fraction).min(*max_pct);
                    (bankroll * kelly_pct).clamp(1.0, bankroll)
                }
            }
        }
    }
}

