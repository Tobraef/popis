use crate::popis_error::PopisError;

mod services;

pub use services::serve;

impl actix_web::ResponseError for PopisError {
    fn status_code(&self) -> reqwest::StatusCode {
        match self {
            PopisError::WebRequest(_) => reqwest::StatusCode::FAILED_DEPENDENCY,
            PopisError::HtmlParsing(_) => reqwest::StatusCode::FAILED_DEPENDENCY,
            PopisError::DbConnectionError(_) => reqwest::StatusCode::SERVICE_UNAVAILABLE,
            PopisError::DbCommunicationError(_) => reqwest::StatusCode::SERVICE_UNAVAILABLE,
            PopisError::LogicError(_) => reqwest::StatusCode::INTERNAL_SERVER_ERROR,
            PopisError::ServerStart(_) => reqwest::StatusCode::SERVICE_UNAVAILABLE,
            PopisError::GameplayError(_) => reqwest::StatusCode::BAD_REQUEST,
        }
    }
}