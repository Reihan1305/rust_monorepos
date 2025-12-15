use actix_web::{HttpResponse, http::StatusCode};
use serde_json::json;

use std::{collections::HashMap, env, fmt, fs, sync::OnceLock};

#[derive(Debug, Clone)]
pub struct AppError {
    code: u16,
    message: String,
    http_code: StatusCode,
    errors: Option<String>,
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
            panic!("Failed to read global error file: {}", file_path)
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
            panic!("Failed to read service error file: {}", file_path)
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

impl AppError {
    pub fn new(code: u16, http_code: Option<StatusCode>) -> Self {
        let message = get_error_message(code, &format!("Error code: {}", code));
        if let Some(code_http) = http_code {
            return Self {
                code,
                message,
                http_code: code_http,
                errors: None,
            };
        };
        return Self {
            code,
            message,
            http_code: StatusCode::INTERNAL_SERVER_ERROR,
            errors: None,
        };
    }

    pub fn map_db_error(err: sqlx::Error) -> Self {
        if let sqlx::Error::Database(db_err) = &err {
            let code = db_err.code().unwrap_or_default();
            let constraint = db_err.constraint().unwrap_or_default();
            if code == "23505" {
                let parts: Vec<&str> = constraint.split('_').collect();
                let field: String;
                if parts.len() < 3 {
                    field = constraint.to_string();
                } else {
                    let field_parts = &parts[1..parts.len() - 1];

                    field = field_parts.join(" ");
                }

                let message = get_error_message(3011, "a database error occurred");
                return Self {
                    code: 3005,
                    message: message,
                    http_code: StatusCode::INTERNAL_SERVER_ERROR,
                    errors: Some(format!("{} already exists", field)),
                };
            }
        }
        let message = get_error_message(3005, "a database error occurred");
        return Self {
            code: 3005,
            message: message,
            http_code: StatusCode::INTERNAL_SERVER_ERROR,
            errors: None,
        };
    }

    pub fn http_response_builder(&self) -> HttpResponse {
        let error_response = json!({
            "code": self.code,
            "message": self.message,
            "errors": self.errors,
        });
        HttpResponse::build(self.http_code).json(error_response)
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

pub type AppResult<T> = Result<T, AppError>;
