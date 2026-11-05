use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, AppError>;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("HTTP client error: {0}")]
    HttpClient(#[from] reqwest::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Risk limit violation: {0}")]
    RiskViolation(String),

    #[error("Insufficient funds in bankroll: required {required:.2}, available {available:.2}")]
    InsufficientFunds { required: f64, available: f64 },

    #[error("Internal server error: {0}")]
    Internal(String),

    #[error("Unknown error: {0}")]
    Other(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match &self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::RiskViolation(msg) => (StatusCode::FORBIDDEN, msg.clone()),
            AppError::InsufficientFunds { required, available } => (
                StatusCode::BAD_REQUEST,
                format!("Insufficient bankroll: required {:.2}, available {:.2}", required, available),
            ),
            AppError::Database(err) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", err)),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };

        let body = Json(json!({
            "status": "error",
            "message": error_message,
        }));

        (status, body).into_response()
    }
}

