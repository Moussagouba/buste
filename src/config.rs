use std::env;

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub server_host: String,
    pub server_port: u16,
    pub database_url: String,
    pub database_max_connections: u32,

    // Data Sources
    pub melbet_api_base_url: String,
    pub melbet_collection_interval_secs: u64,
    pub target_sports: Vec<u32>,

    pub api_football_key: Option<String>,
    pub api_football_base_url: String,

    pub football_data_org_key: Option<String>,
    pub football_data_org_base_url: String,

    pub openligadb_base_url: String,
    pub statsbomb_base_url: String,
    pub football_data_uk_base_url: String,

    pub thesportsdb_key: String,
    pub thesportsdb_base_url: String,

    // Risk & Strategy
    pub initial_bankroll: f64,
    pub currency: String,
    pub kelly_fraction: f64,
    pub max_bet_pct: f64,
    pub min_value_edge: f64,
    pub min_odds: f64,
    pub max_odds: f64,
    pub paper_betting_auto_execute: bool,
    pub settlement_interval_secs: u64,
}

impl AppConfig {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        let server_host = env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
        let server_port = env::var("SERVER_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(8080);

        let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgres://melbet_user:melbet_secure_pass@localhost:5432/melbet_analytics".to_string()
        });

        let database_max_connections = env::var("DATABASE_MAX_CONNECTIONS")
            .ok()
            .and_then(|c| c.parse().ok())
            .unwrap_or(20);

        // Sources
        let melbet_api_base_url = env::var("MELBET_API_BASE_URL")
            .unwrap_or_else(|_| "https://melbet.com/LineFeed".to_string());

        let melbet_collection_interval_secs = env::var("MELBET_COLLECTION_INTERVAL_SECS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(60);

        let target_sports = env::var("MELBET_TARGET_SPORTS")
            .unwrap_or_else(|_| "1,2,3,4".to_string())
            .split(',')
            .filter_map(|s| s.trim().parse::<u32>().ok())
            .collect();

        let api_football_key = env::var("API_FOOTBALL_KEY").ok().filter(|k| !k.is_empty() && k != "your_api_football_key_here");
        let api_football_base_url = env::var("API_FOOTBALL_BASE_URL")
            .unwrap_or_else(|_| "https://v3.football.api-sports.io".to_string());

        let football_data_org_key = env::var("FOOTBALL_DATA_ORG_KEY").ok().filter(|k| !k.is_empty() && k != "your_football_data_org_key_here");
        let football_data_org_base_url = env::var("FOOTBALL_DATA_ORG_BASE_URL")
            .unwrap_or_else(|_| "https://api.football-data.org/v4".to_string());

        let openligadb_base_url = env::var("OPENLIGADB_BASE_URL")
            .unwrap_or_else(|_| "https://api.openligadb.de".to_string());

        let statsbomb_base_url = env::var("STATSBOMB_BASE_URL")
            .unwrap_or_else(|_| "https://raw.githubusercontent.com/statsbomb/open-data/master/data".to_string());

        let football_data_uk_base_url = env::var("FOOTBALL_DATA_UK_BASE_URL")
            .unwrap_or_else(|_| "https://www.football-data.co.uk/mmz4281".to_string());

        let thesportsdb_key = env::var("THESPORTSDB_KEY").unwrap_or_else(|_| "3".to_string());
        let thesportsdb_base_url = env::var("THESPORTSDB_BASE_URL")
            .unwrap_or_else(|_| format!("https://www.thesportsdb.com/api/v1/json/{}", thesportsdb_key));

        let initial_bankroll = env::var("INITIAL_BANKROLL")
            .ok()
            .and_then(|b| b.parse().ok())
            .unwrap_or(1000.0);

        let currency = env::var("CURRENCY").unwrap_or_else(|_| "EUR".to_string());

        let kelly_fraction = env::var("KELLY_FRACTION")
            .ok()
            .and_then(|k| k.parse().ok())
            .unwrap_or(0.25);

        let max_bet_pct = env::var("MAX_BET_PERCENTAGE")
            .ok()
            .and_then(|m| m.parse().ok())
            .unwrap_or(0.05);

        let min_value_edge = env::var("MIN_VALUE_EDGE")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(0.03);

        let min_odds = env::var("MIN_ODDS")
            .ok()
            .and_then(|o| o.parse().ok())
            .unwrap_or(1.30);

        let max_odds = env::var("MAX_ODDS")
            .ok()
            .and_then(|o| o.parse().ok())
            .unwrap_or(4.50);

        let paper_betting_auto_execute = env::var("PAPER_BETTING_AUTO_EXECUTE")
            .map(|v| v.to_lowercase() == "true" || v == "1")
            .unwrap_or(true);

        let settlement_interval_secs = env::var("SIMULATION_SETTLEMENT_INTERVAL_SECS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(30);

        Self {
            server_host,
            server_port,
            database_url,
            database_max_connections,
            melbet_api_base_url,
            melbet_collection_interval_secs,
            target_sports,
            api_football_key,
            api_football_base_url,
            football_data_org_key,
            football_data_org_base_url,
            openligadb_base_url,
            statsbomb_base_url,
            football_data_uk_base_url,
            thesportsdb_key,
            thesportsdb_base_url,
            initial_bankroll,
            currency,
            kelly_fraction,
            max_bet_pct,
            min_value_edge,
            min_odds,
            max_odds,
            paper_betting_auto_execute,
            settlement_interval_secs,
        }
    }
}
