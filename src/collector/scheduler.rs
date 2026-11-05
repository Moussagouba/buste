use crate::collector::melbet::MelbetClient;
use crate::collector::parser::{MelbetNormalizer, NormalizedEvent};
use crate::database::DbPool;
use crate::error::Result;
use std::sync::Arc;
use std::time::Duration;
use tracing::{error, info};
use uuid::Uuid;

pub async fn run_collector_scheduler(
    pool: DbPool,
    melbet_client: Arc<MelbetClient>,
    interval_secs: u64,
) {
    info!("Starting Melbet Data Collector scheduler (polling every {}s)...", interval_secs);
    let mut interval = tokio::time::interval(Duration::from_secs(interval_secs));

    loop {
        interval.tick().await;

        info!("Collector tick: fetching latest market odds...");
        match melbet_client.fetch_events().await {
            Ok(raw_events) => {
                let normalized: Vec<NormalizedEvent> = raw_events
                    .into_iter()
                    .map(MelbetNormalizer::normalize)
                    .collect();

                info!("Normalized {} events. Storing in database...", normalized.len());

                for event in normalized {
                    if let Err(e) = save_event_and_odds(&pool, &event).await {
                        error!("Failed to store event {}: {}", event.external_id, e);
                    }
                }
            }
            Err(e) => {
                error!("Melbet collector error during fetch: {}", e);
            }
        }
    }
}

pub async fn save_event_and_odds(pool: &DbPool, event: &NormalizedEvent) -> Result<Uuid> {
    // 1. Upsert event
    let event_id: Uuid = sqlx::query_scalar!(
        r#"
        INSERT INTO events (external_id, sport_name, league_name, home_team, away_team, start_time, status)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        ON CONFLICT (external_id) DO UPDATE SET
            status = EXCLUDED.status,
            updated_at = NOW()
        RETURNING id
        "#,
        event.external_id,
        event.sport_name,
        event.league_name,
        event.home_team,
        event.away_team,
        event.start_time,
        event.status
    )
    .fetch_one(pool)
    .await?;

    // 2. Upsert markets and insert odds
    for market in &event.markets {
        let market_id: Uuid = sqlx::query_scalar!(
            r#"
            INSERT INTO markets (event_id, market_type, name, parameter, status)
            VALUES ($1, $2, $3, $4, 'ACTIVE')
            ON CONFLICT (event_id, market_type, parameter) DO UPDATE SET
                status = 'ACTIVE',
                updated_at = NOW()
            RETURNING id
            "#,
            event_id,
            market.market_type,
            market.name,
            market.parameter
        )
        .fetch_one(pool)
        .await?;

        for outcome in &market.outcomes {
            sqlx::query!(
                r#"
                INSERT INTO odds (market_id, outcome_name, price, implied_probability, bookmaker_margin)
                VALUES ($1, $2, $3, $4, $5)
                "#,
                market_id,
                outcome.outcome_name,
                outcome.price,
                outcome.implied_probability,
                market.bookmaker_margin
            )
            .execute(pool)
            .await?;
        }
    }

    Ok(event_id)
}

