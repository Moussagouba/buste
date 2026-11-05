use crate::api::{
    bets::{list_bets, place_bet, trigger_settlement},
    dashboard::{get_bankroll_history, get_dashboard_summary, run_backtest_handler},
    events::{get_event, list_events, refresh_events},
    predictions::{list_predictions, list_value_bets},
    sources::list_sources_status,
    AppState,
};
use axum::{
    routing::{get, post},
    Router,
};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

pub fn create_router(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let api_routes = Router::new()
        // Dashboard KPIs & Analytics
        .route("/dashboard/summary", get(get_dashboard_summary))
        .route("/dashboard/bankroll-history", get(get_bankroll_history))
        .route("/dashboard/backtest", post(run_backtest_handler))
        // Multi-Source Data Feeds Status
        .route("/sources", get(list_sources_status))
        // Events & Melbet line
        .route("/events", get(list_events))
        .route("/events/:id", get(get_event))
        .route("/events/refresh", post(refresh_events))
        // Predictions & Value Betting Signals
        .route("/predictions", get(list_predictions))
        .route("/predictions/value-bets", get(list_value_bets))
        // Paper Bets & Virtual Portfolio
        .route("/bets", get(list_bets).post(place_bet))
        .route("/bets/settle", post(trigger_settlement));

    Router::new()
        .nest("/api", api_routes)
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
