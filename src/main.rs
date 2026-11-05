mod analysis;
mod api;
mod collector;
mod config;
mod database;
mod error;
mod models;
mod risk;
mod simulation;
mod strategy;
mod telemetry;

use api::{create_router, AppState};
use collector::{run_collector_scheduler, MelbetClient};
use config::AppConfig;
use database::{create_pool, run_migrations};
use simulation::settlement::SettlementEngine;
use strategy::strategy::{BettingStrategy, StrategyConfig};
use std::net::SocketAddr;
use std::sync::Arc;
use telemetry::init_telemetry;
use tracing::{error, info};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. Telemetry and logging
    init_telemetry();
    info!("Starting Melbet Quantitative Analytics & Value Betting Engine...");

    // 2. Application Configuration
    let config = AppConfig::from_env();
    info!(
        "Loaded config: Server on {}:{}, Initial Bankroll: {} {}",
        config.server_host, config.server_port, config.initial_bankroll, config.currency
    );

    // 3. PostgreSQL Database Connection Pool
    let pool = match create_pool(&config.database_url, config.database_max_connections).await {
        Ok(p) => {
            if let Err(e) = run_migrations(&p).await {
                error!("Database migration warning: {}", e);
            }
            p
        }
        Err(e) => {
            error!(
                "Could not connect to PostgreSQL at {}: {}. Running in memory fallback mode.",
                config.database_url, e
            );
            // Re-attempt or propagate error
            return Err(anyhow::anyhow!("PostgreSQL connection failed: {}", e));
        }
    };

    // 4. Core Domain Components
    let melbet_client = Arc::new(MelbetClient::new(
        config.melbet_api_base_url.clone(),
        config.target_sports.clone(),
    ));

    let mut strategy_config = StrategyConfig::default();
    strategy_config.kelly_fraction = config.kelly_fraction;
    strategy_config.max_bet_pct = config.max_bet_pct;
    strategy_config.filters.min_ev = config.min_value_edge;
    strategy_config.filters.min_odds = config.min_odds;
    strategy_config.filters.max_odds = config.max_odds;

    let strategy = Arc::new(BettingStrategy::new(strategy_config));

    // 5. Initial Collector Feed Ingestion
    info!("Executing initial Melbet feed snapshot...");
    match melbet_client.fetch_events().await {
        Ok(events) => {
            for raw_ev in events {
                let norm = collector::parser::MelbetNormalizer::normalize(raw_ev);
                if let Err(e) = collector::scheduler::save_event_and_odds(&pool, &norm).await {
                    error!("Initial seed event save error: {}", e);
                }
            }
            info!("Initial market data successfully ingested.");
        }
        Err(e) => warn_or_log(&format!("Initial ingestion warning: {}", e)),
    }

    // 6. Spawn Background Schedulers
    let pool_collector = pool.clone();
    let melbet_for_sched = melbet_client.clone();
    let interval_secs = config.melbet_collection_interval_secs;
    tokio::spawn(async move {
        run_collector_scheduler(pool_collector, melbet_for_sched, interval_secs).await;
    });

    let pool_settler = pool.clone();
    let settle_interval = config.settlement_interval_secs;
    tokio::spawn(async move {
        SettlementEngine::run_settlement_worker(pool_settler, settle_interval).await;
    });

    // 7. API Routing & Server
    let state = AppState {
        pool,
        config: config.clone(),
        melbet: melbet_client,
        strategy,
    };

    let app = create_router(state);
    let addr: SocketAddr = format!("{}:{}", config.server_host, config.server_port).parse()?;
    info!("Melbet Analytics API server running at http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

fn warn_or_log(msg: &str) {
    tracing::warn!("{}", msg);
}

