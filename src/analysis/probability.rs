use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct OutcomeProbabilities {
    pub home_win: f64,
    pub draw: f64,
    pub away_win: f64,
    pub over_2_5: f64,
    pub under_2_5: f64,
    pub btts_yes: f64,
    pub btts_no: f64,
}

pub struct ProbabilityEngine;

impl ProbabilityEngine {
    /// Calculate exact score matrix using Poisson distribution
    /// P(X=k) = (lambda^k * e^-lambda) / k!
    pub fn compute_poisson_probabilities(lambda: f64, mu: f64) -> OutcomeProbabilities {
        let max_goals = 10;
        let mut home_probs = vec![0.0; max_goals + 1];
        let mut away_probs = vec![0.0; max_goals + 1];

        for k in 0..=max_goals {
            home_probs[k] = Self::poisson_pmf(lambda, k);
            away_probs[k] = Self::poisson_pmf(mu, k);
        }

        let mut home_win = 0.0;
        let mut draw = 0.0;
        let mut away_win = 0.0;
        let mut over_2_5 = 0.0;
        let mut under_2_5 = 0.0;
        let mut btts_yes = 0.0;
        let mut btts_no = 0.0;

        for h in 0..=max_goals {
            for a in 0..=max_goals {
                let prob = home_probs[h] * away_probs[a];

                // 1X2
                if h > a {
                    home_win += prob;
                } else if h == a {
                    draw += prob;
                } else {
                    away_win += prob;
                }

                // Over / Under 2.5
                if (h + a) > 2 {
                    over_2_5 += prob;
                } else {
                    under_2_5 += prob;
                }

                // BTTS
                if h > 0 && a > 0 {
                    btts_yes += prob;
                } else {
                    btts_no += prob;
                }
            }
        }

        // Normalize sum to 1.0
        let total_1x2 = home_win + draw + away_win;
        if total_1x2 > 0.0 {
            home_win /= total_1x2;
            draw /= total_1x2;
            away_win /= total_1x2;
        }

        OutcomeProbabilities {
            home_win,
            draw,
            away_win,
            over_2_5,
            under_2_5,
            btts_yes,
            btts_no,
        }
    }

    /// De-vig bookmaker odds using Shin's / Proportional method to extract implied probabilities
    pub fn remove_vig_proportional(odds: &[f64]) -> Vec<f64> {
        let raw_implied: Vec<f64> = odds.iter().map(|o| 1.0 / o).collect();
        let overround: f64 = raw_implied.iter().sum();
        if overround <= 0.0 {
            return raw_implied;
        }
        raw_implied.iter().map(|p| p / overround).collect()
    }

    fn poisson_pmf(lambda: f64, k: usize) -> f64 {
        let k_f = k as f64;
        (-lambda).exp() * lambda.powf(k_f) / Self::factorial(k)
    }

    fn factorial(n: usize) -> f64 {
        let mut res = 1.0;
        for i in 2..=n {
            res *= i as f64;
        }
        res
    }
}

