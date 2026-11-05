use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventFeatures {
    pub home_attack_strength: f64,
    pub home_defense_strength: f64,
    pub away_attack_strength: f64,
    pub away_defense_strength: f64,
    pub league_avg_home_goals: f64,
    pub league_avg_away_goals: f64,
    pub expected_home_goals: f64, // lambda
    pub expected_away_goals: f64, // mu
    pub bookmaker_overround: f64,
    pub days_until_event: f64,
}

pub struct FeatureEngine;

impl FeatureEngine {
    pub fn extract_features(
        sport: &str,
        home_team: &str,
        away_team: &str,
        bookmaker_overround: f64,
        hours_until: f64,
    ) -> EventFeatures {
        // Base league averages (e.g. Football ~1.50 home, 1.15 away)
        let (base_home, base_away) = match sport.to_lowercase().as_str() {
            "football" => (1.52, 1.18),
            "basketball" => (108.5, 104.2),
            _ => (1.40, 1.20),
        };

        // Team strength heuristics based on top tier teams
        let home_att = Self::estimate_team_strength(home_team, true);
        let home_def = Self::estimate_team_strength(home_team, false);
        let away_att = Self::estimate_team_strength(away_team, true);
        let away_def = Self::estimate_team_strength(away_team, false);

        // Expected goals (Poisson intensity parameters)
        // lambda = Home Attack * Away Defense * League Avg Home
        // mu = Away Attack * Home Defense * League Avg Away
        let lambda = (home_att * away_def * base_home).max(0.2);
        let mu = (away_att * home_def * base_away).max(0.2);

        EventFeatures {
            home_attack_strength: home_att,
            home_defense_strength: home_def,
            away_attack_strength: away_att,
            away_defense_strength: away_def,
            league_avg_home_goals: base_home,
            league_avg_away_goals: base_away,
            expected_home_goals: lambda,
            expected_away_goals: mu,
            bookmaker_overround,
            days_until_event: hours_until / 24.0,
        }
    }

    fn estimate_team_strength(name: &str, is_attack: bool) -> f64 {
        let n = name.to_lowercase();
        if n.contains("city") || n.contains("real madrid") || n.contains("bayern") {
            if is_attack { 1.35 } else { 0.75 }
        } else if n.contains("arsenal") || n.contains("liverpool") || n.contains("barcelona") || n.contains("celtics") {
            if is_attack { 1.25 } else { 0.82 }
        } else if n.contains("chelsea") || n.contains("juventus") || n.contains("inter") || n.contains("atletico") {
            if is_attack { 1.05 } else { 0.88 }
        } else {
            1.00
        }
    }
}

