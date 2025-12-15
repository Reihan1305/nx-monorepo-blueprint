use actix_web::{HttpResponse, error::ResponseError, http::StatusCode};
use serde::Serialize;
use std::{collections::HashMap, env, fmt, fs, sync::OnceLock};

#[derive(Debug, Clone)]
pub struct AppError {
    code: u16,
    message: String,
    http_code: Option<u16>,
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
        let default_path = "apps/rust_forge_boilerplate/error.json";
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

#[derive(Serialize)]
struct ErrorResponse {
    code: u16,
    message: String,
}

impl AppError {
    pub fn new(code: u16, message: Option<String>, http_code: Option<u16>) -> Self {
        let message = message.unwrap_or_else(|| get_error_message(code, &format!("Error code: {}", code)));
        Self {
            code,
            message,
            http_code,
        }
    }

    pub fn with_code(code: u16) -> Self {
        Self::new(code, None, None)
    }

    pub fn with_message(code: u16, message: String) -> Self {
        Self::new(code, Some(message), None)
    }

    pub fn with_http_code(code: u16, message: Option<String>, http_code: u16) -> Self {
        Self::new(code, message, Some(http_code))
    }

    pub fn get_message(&self) -> &str {
        &self.message
    }

    pub fn get_error_code(&self) -> u16 {
        self.code
    }

    pub fn get_http_code(&self) -> u16 {
        self.http_code.unwrap_or(400)
    }

    // Convenience constructors for common errors
    pub fn internal_error(message: Option<String>) -> Self {
        Self::with_http_code(1000, message, 500)
    }

    pub fn bad_request(message: Option<String>) -> Self {
        Self::with_http_code(1001, message, 400)
    }

    pub fn not_found(message: Option<String>) -> Self {
        Self::with_http_code(1002, message, 404)
    }

    pub fn unauthorized(message: Option<String>) -> Self {
        Self::with_http_code(1003, message, 401)
    }

    pub fn validation_error(message: Option<String>) -> Self {
        Self::with_http_code(1004, message, 422)
    }

    pub fn database_error(message: Option<String>) -> Self {
        Self::with_http_code(1005, message, 500)
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self.get_http_code() {
            400 => StatusCode::BAD_REQUEST,
            401 => StatusCode::UNAUTHORIZED,
            403 => StatusCode::FORBIDDEN,
            404 => StatusCode::NOT_FOUND,
            422 => StatusCode::UNPROCESSABLE_ENTITY,
            429 => StatusCode::TOO_MANY_REQUESTS,
            500 => StatusCode::INTERNAL_SERVER_ERROR,
            502 => StatusCode::BAD_GATEWAY,
            503 => StatusCode::SERVICE_UNAVAILABLE,
            _ => StatusCode::BAD_REQUEST, // Default fallback
        }
    }

    fn error_response(&self) -> HttpResponse {
        let error_response = ErrorResponse {
            code: self.code,
            message: self.message.clone(),
        };
        HttpResponse::build(self.status_code()).json(error_response)
    }
}



pub type AppResult<T> = Result<T, AppError>;
