use actix_web::{HttpResponse, error::ResponseError, http::StatusCode};
use serde::Serialize;
use std::{collections::HashMap, env, fmt, fs, process::exit, sync::OnceLock};

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
    CustomError { code: u16, message: String },
}

static GLOBAL_ERROR_MESSAGES: OnceLock<HashMap<String, String>> = OnceLock::new();
static SERVICE_ERROR_MESSAGES: OnceLock<HashMap<String, String>> = OnceLock::new();

fn load_global_error_messages() -> &'static HashMap<String, String> {
    GLOBAL_ERROR_MESSAGES.get_or_init(|| {
        let default_path = "error.json";
        let file_path =
            env::var("GLOBAL_ERROR_FILE_PATH").unwrap_or_else(|_| default_path.to_string());

        let error_json = if let Ok(content) = fs::read_to_string(&file_path) {
            content
        } else {
            tracing::error!("Failed to read global error file: {}", file_path);
            exit(1)
        };

        serde_json::from_str(&error_json).unwrap_or_else(|e| {
            eprintln!("Failed to parse global error JSON: {}", e);
            HashMap::new()
        })
    })
}

fn load_service_error_messages() -> &'static HashMap<String, String> {
    SERVICE_ERROR_MESSAGES.get_or_init(|| {
        let default_path = "apps/rust_app_template/error.json";
        let file_path =
            env::var("SERVICE_ERROR_FILE_PATH").unwrap_or_else(|_| default_path.to_string());

        let error_json = if let Ok(content) = fs::read_to_string(&file_path) {
            content
        } else {
            tracing::error!("Failed to read service error file: {}", file_path);
            exit(1)
        };

        serde_json::from_str(&error_json).unwrap_or_else(|e| {
            eprintln!("Failed to parse service error JSON: {}", e);
            HashMap::new()
        })
    })
}

fn get_error_message(code: u16, fallback: &str) -> String {
    let message = if code >= 1000 && code < 2000 {
        let global_messages = load_global_error_messages();
        global_messages.get(&code.to_string()).cloned()
    } else {
        let service_messages = load_service_error_messages();
        service_messages.get(&code.to_string()).cloned()
    };

    message.unwrap_or_else(|| fallback.to_string())
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
            AppError::ServiceUnavailable => get_error_message(1102, "Service unavailable"),
            AppError::ConfigurationError => get_error_message(1100, "Configuration error"),
            AppError::ExternalServiceError => get_error_message(1101, "External service error"),
            AppError::RateLimitExceeded => get_error_message(1111, "Rate limit exceeded"),
            AppError::InvalidApiKey => get_error_message(1106, "Invalid API key"),
            AppError::MaintenanceMode => get_error_message(1112, "Service maintenance mode"),
            AppError::CustomError { message, .. } => message.clone(),
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
            AppError::CustomError { code, .. } => match *code {
                1001 | 2001..=2099 => StatusCode::BAD_REQUEST,
                1002 | 2100..=2199 => StatusCode::NOT_FOUND,
                1003 | 2200..=2299 => StatusCode::UNAUTHORIZED,
                1004 | 2300..=2399 => StatusCode::UNPROCESSABLE_ENTITY,
                1111 | 2400..=2499 => StatusCode::TOO_MANY_REQUESTS,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            },
        }
    }

    fn error_response(&self) -> HttpResponse {
        let (code, message) = match self {
            AppError::InternalError(msg) => (1000, msg.clone()),
            AppError::BadRequest(msg) => (1001, msg.clone()),
            AppError::NotFound(msg) => (1002, msg.clone()),
            AppError::Unauthorized(msg) => (1003, msg.clone()),
            AppError::ValidationError(msg) => (1004, msg.clone()),
            AppError::DatabaseError(msg) => (1005, msg.clone()),
            AppError::ServiceUnavailable => (1102, get_error_message(1102, "Service unavailable")),
            AppError::ConfigurationError => (1100, get_error_message(1100, "Configuration error")),
            AppError::ExternalServiceError => {
                (1101, get_error_message(1101, "External service error"))
            }
            AppError::RateLimitExceeded => (1111, get_error_message(1111, "Rate limit exceeded")),
            AppError::InvalidApiKey => (1106, get_error_message(1106, "Invalid API key")),
            AppError::MaintenanceMode => {
                (1112, get_error_message(1112, "Service maintenance mode"))
            }
            AppError::CustomError { code, message } => (*code, message.clone()),
        };

        let error_response = ErrorResponse { code, message };
        HttpResponse::build(self.status_code()).json(error_response)
    }
}

impl AppError {
    pub fn service_error(code: u16) -> Self {
        let message = get_error_message(code, &format!("Error code: {}", code));
        AppError::CustomError { code, message }
    }
}

pub type AppResult<T> = Result<T, AppError>;
