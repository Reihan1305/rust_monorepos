use actix_web::{HttpResponse, error::ResponseError, http::StatusCode};
use serde::Serialize;
use std::{collections::HashMap, fmt, sync::OnceLock};

#[derive(Debug)]
pub enum AppError {
    InternalError(String),
    BadRequest(String),
    NotFound(String),
    Unauthorized(String),
    ValidationError(String),
    DatabaseError(String),
    ServiceUnavailable,
    ConfigurationError,
    ExternalServiceError,
    RateLimitExceeded,
    InvalidApiKey,
    MaintenanceMode,
}

static ERROR_MESSAGES: OnceLock<HashMap<String, String>> = OnceLock::new();

fn load_error_messages() -> &'static HashMap<String, String> {
    ERROR_MESSAGES.get_or_init(|| {
        let error_json = include_str!("../../error.json");
        serde_json::from_str(error_json).unwrap_or_else(|_| HashMap::new())
    })
}

fn get_error_message(code: u16, fallback: &str) -> String {
    let messages = load_error_messages();
    messages
        .get(&code.to_string())
        .cloned()
        .unwrap_or_else(|| fallback.to_string())
}

#[derive(Serialize)]
struct ErrorResponse {
    code: u16,
    message: String,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            AppError::InternalError(msg) => format!("Internal Error: {}", msg),
            AppError::BadRequest(msg) => format!("Bad Request: {}", msg),
            AppError::NotFound(msg) => format!("Not Found: {}", msg),
            AppError::Unauthorized(msg) => format!("Unauthorized: {}", msg),
            AppError::ValidationError(msg) => format!("Validation Error: {}", msg),
            AppError::DatabaseError(msg) => format!("Database Error: {}", msg),
            AppError::ServiceUnavailable => get_error_message(3100, "Service unavailable"),
            AppError::ConfigurationError => get_error_message(3101, "Configuration error"),
            AppError::ExternalServiceError => get_error_message(3102, "External service error"),
            AppError::RateLimitExceeded => get_error_message(3103, "Rate limit exceeded"),
            AppError::InvalidApiKey => get_error_message(3104, "Invalid API key"),
            AppError::MaintenanceMode => get_error_message(3105, "Service maintenance mode"),
        };
        write!(f, "{}", message)
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            AppError::ValidationError(_) => StatusCode::UNPROCESSABLE_ENTITY,
            AppError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::ServiceUnavailable => StatusCode::SERVICE_UNAVAILABLE,
            AppError::ConfigurationError => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::ExternalServiceError => StatusCode::BAD_GATEWAY,
            AppError::RateLimitExceeded => StatusCode::TOO_MANY_REQUESTS,
            AppError::InvalidApiKey => StatusCode::UNAUTHORIZED,
            AppError::MaintenanceMode => StatusCode::SERVICE_UNAVAILABLE,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let (code, message) = match self {
            AppError::InternalError(msg) => (3000, msg.clone()),
            AppError::BadRequest(msg) => (3001, msg.clone()),
            AppError::NotFound(msg) => (3002, msg.clone()),
            AppError::Unauthorized(msg) => (3003, msg.clone()),
            AppError::ValidationError(msg) => (3004, msg.clone()),
            AppError::DatabaseError(msg) => (3005, msg.clone()),
            AppError::ServiceUnavailable => (3100, get_error_message(3100, "Service unavailable")),
            AppError::ConfigurationError => (3101, get_error_message(3101, "Configuration error")),
            AppError::ExternalServiceError => {
                (3102, get_error_message(3102, "External service error"))
            }
            AppError::RateLimitExceeded => (3103, get_error_message(3103, "Rate limit exceeded")),
            AppError::InvalidApiKey => (3104, get_error_message(3104, "Invalid API key")),
            AppError::MaintenanceMode => {
                (3105, get_error_message(3105, "Service maintenance mode"))
            }
        };

        let error_response = ErrorResponse { code, message };
        HttpResponse::build(self.status_code()).json(error_response)
    }
}

impl AppError {
    // Convenience constructors for service-specific errors
    pub fn service_unavailable() -> Self {
        AppError::ServiceUnavailable
    }

    pub fn configuration_error() -> Self {
        AppError::ConfigurationError
    }

    pub fn external_service_error() -> Self {
        AppError::ExternalServiceError
    }

    pub fn rate_limit_exceeded() -> Self {
        AppError::RateLimitExceeded
    }

    pub fn invalid_api_key() -> Self {
        AppError::InvalidApiKey
    }

    pub fn maintenance_mode() -> Self {
        AppError::MaintenanceMode
    }
}

pub type AppResult<T> = Result<T, AppError>;
