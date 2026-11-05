use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationResult {
    pub outcome_name: String,
    pub bookmaker_odds: f64,
    pub model_probability: f64,
    pub implied_probability: f64,
    pub fair_odds: f64,
    pub expected_value: f64, // (p * odds) - 1.0
    pub edge: f64,           // p - implied_probability
    pub kelly_stake_pct: f64,
    pub is_value_bet: bool,
}

pub struct ValueEngine;

impl ValueEngine {
    pub fn evaluate_outcome(
        outcome_name: &str,
        bookmaker_odds: f64,
        model_probability: f64,
        min_ev_threshold: f64,
        kelly_fraction: f64,
    ) -> EvaluationResult {
        let implied_probability = if bookmaker_odds > 0.0 { 1.0 / bookmaker_odds } else { 0.0 };
        let fair_odds = if model_probability > 0.0 { 1.0 / model_probability } else { 999.0 };

        // Expected Value = (Probability * Decimal Odds) - 1.0
        let expected_value = (model_probability * bookmaker_odds) - 1.0;

        // Edge over market = true prob - implied prob
        let edge = model_probability - implied_probability;

        // Full Kelly Criterion: f* = (bp - q) / b
        // where b = odds - 1, p = model prob, q = 1 - p
        let b = bookmaker_odds - 1.0;
        let q = 1.0 - model_probability;
        let full_kelly = if b > 0.0 {
            ((b * model_probability) - q) / b
        } else {
            0.0
        };

        // Fractional Kelly applied only if EV > 0
        let fractional_kelly = if full_kelly > 0.0 && expected_value > 0.0 {
            (full_kelly * kelly_fraction).clamp(0.0, 0.10) // Cap at 10% for risk mitigation
        } else {
            0.0
        };

        let is_value_bet = expected_value >= min_ev_threshold && fractional_kelly > 0.005;

        EvaluationResult {
            outcome_name: outcome_name.to_string(),
            bookmaker_odds,
            model_probability,
            implied_probability,
            fair_odds,
            expected_value,
            edge,
            kelly_stake_pct: fractional_kelly,
            is_value_bet,
        }
    }
}

