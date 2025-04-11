use actix_web::{HttpResponse, ResponseError};
use mysql::Error as MysqlError;
use serde_json::Error as JsonError;
use std::fmt;
use tracing::error;

#[derive(Debug)]
pub enum AppError {
    Database(MysqlError),
    Config(String),
    NotFound,
    Validation(String),
    Internal(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Database(e) => write!(f, "Database error: {}", e),
            Self::Config(e) => write!(f, "Configuration error: {}", e),
            Self::NotFound => write!(f, "Resource not found"),
            Self::Validation(e) => write!(f, "Validation error: {}", e),
            Self::Internal(e) => write!(f, "Internal error: {}", e),
        }
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        error!("{}", self);
        
        match self {
            Self::Database(_) => HttpResponse::InternalServerError().json(
                serde_json::json!({"error": "Database operation failed"})
            ),
            Self::Config(_) => HttpResponse::InternalServerError().json(
                serde_json::json!({"error": "Configuration error"})
            ),
            Self::NotFound => HttpResponse::NotFound().json(
                serde_json::json!({"error": "Not found"})
            ),
            Self::Validation(msg) => HttpResponse::BadRequest().json(
                serde_json::json!({"error": msg})
            ),
            Self::Internal(_) => HttpResponse::InternalServerError().json(
                serde_json::json!({"error": "Internal server error"})
            ),
        }
    }
}

impl From<MysqlError> for AppError {
    fn from(value: MysqlError) -> Self {
        Self::Database(value)
    }
}

impl From<env::VarError> for AppError {
    fn from(value: env::VarError) -> Self {
        Self::Config(value.to_string())
    }
}

impl From<JsonError> for AppError {
    fn from(value: JsonError) -> Self {
        Self::Validation(value.to_string())
    }
}