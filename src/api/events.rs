use crate::api::AppState;
use crate::collector::parser::MelbetNormalizer;
use crate::collector::scheduler::save_event_and_odds;
use crate::error::Result;
use crate::models::event::Event;
use crate::models::market::{Market, MarketWithOptions};
use crate::models::odds::Odds;
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use serde_json::json;
use uuid::Uuid;

pub async fn list_events(State(state): State<AppState>) -> Result<impl IntoResponse> {
    let events = sqlx::query_as!(
        Event,
        r#"
        SELECT 
            id, external_id, sport_name, league_name, home_team, away_team, 
            start_time, status, home_score, away_score, period_scores, 
            raw_metadata, created_at, updated_at
        FROM events
        ORDER BY start_time ASC
        LIMIT 50
        "#
    )
    .fetch_all(&state.pool)
    .await?;

    let mut result = Vec::new();

    for ev in events {
        let markets = sqlx::query_as!(
            Market,
            r#"
            SELECT id, event_id, external_market_id, market_type, name, parameter, status, created_at, updated_at
            FROM markets
            WHERE event_id = $1
            "#,
            ev.id
        )
        .fetch_all(&state.pool)
        .await?;

        let mut market_with_odds = Vec::new();
        for m in markets {
            let odds = sqlx::query_as!(
                Odds,
                r#"
                SELECT id, market_id, outcome_name, price, implied_probability, bookmaker_margin, is_active, recorded_at
                FROM odds
                WHERE market_id = $1
                ORDER BY outcome_name ASC
                "#,
                m.id
            )
            .fetch_all(&state.pool)
            .await?;

            market_with_odds.push(MarketWithOptions {
                market: m,
                odds,
            });
        }

        result.push(json!({
            "event": ev,
            "markets": market_with_odds,
        }));
    }

    Ok(Json(json!({
        "status": "success",
        "data": result,
    })))
}

pub async fn get_event(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse> {
    let event = sqlx::query_as!(
        Event,
        r#"
        SELECT 
            id, external_id, sport_name, league_name, home_team, away_team, 
            start_time, status, home_score, away_score, period_scores, 
            raw_metadata, created_at, updated_at
        FROM events
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(&state.pool)
    .await?;

    match event {
        Some(ev) => Ok(Json(json!({ "status": "success", "data": ev }))),
        None => Err(crate::error::AppError::NotFound("Event not found".into())),
    }
}

pub async fn refresh_events(State(state): State<AppState>) -> Result<impl IntoResponse> {
    let raw = state.melbet.fetch_events().await?;
    let mut imported = 0;

    for item in raw {
        let norm = MelbetNormalizer::normalize(item);
        if save_event_and_odds(&state.pool, &norm).await.is_ok() {
            imported += 1;
        }
    }

    Ok(Json(json!({
        "status": "success",
        "message": format!("Refreshed and imported {} events from Melbet", imported),
        "imported_count": imported
    })))
}

