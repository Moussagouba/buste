use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawMelbetOutcome {
    pub type_id: u32,       // e.g. 1 = W1, 2 = W2, 3 = X, 9 = Total Over, 10 = Total Under
    pub name: Option<String>,
    pub coef: f64,          // Decimal odds
    pub param: Option<f64>, // e.g. 2.5 for totals
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawMelbetEvent {
    pub id: u64,
    pub sport_id: u32,
    pub sport_name: String,
    pub league_id: Option<u64>,
    pub league_name: String,
    pub home_team: String,
    pub away_team: String,
    pub start_timestamp: i64,
    pub is_live: bool,
    pub outcomes: Vec<RawMelbetOutcome>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NormalizedOutcome {
    pub outcome_name: String, // "1", "X", "2", "Over", "Under", "Yes", "No"
    pub price: f64,
    pub implied_probability: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NormalizedMarket {
    pub market_type: String, // "1X2", "OVER_UNDER", "BOTH_TEAMS_SCORE"
    pub name: String,
    pub parameter: Option<f64>,
    pub outcomes: Vec<NormalizedOutcome>,
    pub bookmaker_margin: f64, // Overround
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NormalizedEvent {
    pub external_id: String,
    pub sport_name: String,
    pub league_name: String,
    pub home_team: String,
    pub away_team: String,
    pub start_time: DateTime<Utc>,
    pub status: String,
    pub markets: Vec<NormalizedMarket>,
}

pub struct MelbetNormalizer;

impl MelbetNormalizer {
    pub fn normalize(raw: RawMelbetEvent) -> NormalizedEvent {
        let start_time = Utc.timestamp_opt(raw.start_timestamp, 0)
            .single()
            .unwrap_or_else(Utc::now);

        let status = if raw.is_live {
            "LIVE".to_string()
        } else {
            "SCHEDULED".to_string()
        };

        let mut markets = Vec::new();

        // 1. Group 1X2 market (Type 1 = 1, Type 2 = 2, Type 3 = X)
        let w1 = raw.outcomes.iter().find(|o| o.type_id == 1);
        let draw = raw.outcomes.iter().find(|o| o.type_id == 3 || o.type_id == 2 && raw.sport_name.to_lowercase().contains("tennis"));
        let w2 = raw.outcomes.iter().find(|o| o.type_id == 2 || (o.type_id == 3 && raw.sport_name.to_lowercase().contains("tennis")));

        if let (Some(w1_o), Some(w2_o)) = (w1, w2) {
            let mut outcomes = vec![
                NormalizedOutcome {
                    outcome_name: "1".to_string(),
                    price: w1_o.coef,
                    implied_probability: 1.0 / w1_o.coef,
                },
            ];

            let is_two_way = raw.sport_name.to_lowercase().contains("tennis") || raw.sport_name.to_lowercase().contains("basketball");

            if !is_two_way {
                if let Some(draw_o) = draw {
                    outcomes.push(NormalizedOutcome {
                        outcome_name: "X".to_string(),
                        price: draw_o.coef,
                        implied_probability: 1.0 / draw_o.coef,
                    });
                }
            }

            outcomes.push(NormalizedOutcome {
                outcome_name: "2".to_string(),
                price: w2_o.coef,
                implied_probability: 1.0 / w2_o.coef,
            });

            let sum_implied: f64 = outcomes.iter().map(|o| o.implied_probability).sum();
            let margin = (sum_implied - 1.0).max(0.0);

            markets.push(NormalizedMarket {
                market_type: "1X2".to_string(),
                name: if is_two_way { "Moneyline (Match Winner)".to_string() } else { "Match Result (1X2)".to_string() },
                parameter: None,
                outcomes,
                bookmaker_margin: margin,
            });
        }

        // 2. Over / Under (Total Goals / Points)
        let over = raw.outcomes.iter().find(|o| o.type_id == 9 || o.name.as_deref() == Some("Over"));
        let under = raw.outcomes.iter().find(|o| o.type_id == 10 || o.name.as_deref() == Some("Under"));

        if let (Some(ov), Some(un)) = (over, under) {
            let param = ov.param.or(un.param).unwrap_or(2.5);
            let outcomes = vec![
                NormalizedOutcome {
                    outcome_name: "Over".to_string(),
                    price: ov.coef,
                    implied_probability: 1.0 / ov.coef,
                },
                NormalizedOutcome {
                    outcome_name: "Under".to_string(),
                    price: un.coef,
                    implied_probability: 1.0 / un.coef,
                },
            ];
            let sum_implied: f64 = outcomes.iter().map(|o| o.implied_probability).sum();
            markets.push(NormalizedMarket {
                market_type: "OVER_UNDER".to_string(),
                name: format!("Total Over/Under {:.1}", param),
                parameter: Some(param),
                outcomes,
                bookmaker_margin: (sum_implied - 1.0).max(0.0),
            });
        }

        // 3. Both Teams To Score (BTTS)
        let btts_yes = raw.outcomes.iter().find(|o| o.type_id == 180 || o.name.as_deref() == Some("BTTS_Yes"));
        let btts_no = raw.outcomes.iter().find(|o| o.type_id == 181 || o.name.as_deref() == Some("BTTS_No"));

        if let (Some(y), Some(n)) = (btts_yes, btts_no) {
            let outcomes = vec![
                NormalizedOutcome {
                    outcome_name: "Yes".to_string(),
                    price: y.coef,
                    implied_probability: 1.0 / y.coef,
                },
                NormalizedOutcome {
                    outcome_name: "No".to_string(),
                    price: n.coef,
                    implied_probability: 1.0 / n.coef,
                },
            ];
            let sum_implied: f64 = outcomes.iter().map(|o| o.implied_probability).sum();
            markets.push(NormalizedMarket {
                market_type: "BOTH_TEAMS_SCORE".to_string(),
                name: "Both Teams To Score".to_string(),
                parameter: None,
                outcomes,
                bookmaker_margin: (sum_implied - 1.0).max(0.0),
            });
        }

        NormalizedEvent {
            external_id: raw.id.to_string(),
            sport_name: raw.sport_name,
            league_name: raw.league_name,
            home_team: raw.home_team,
            away_team: raw.away_team,
            start_time,
            status,
            markets,
        }
    }
}

