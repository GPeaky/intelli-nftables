use axum::response::IntoResponse;
use hyper::StatusCode;

use crate::response::error::AppErrorResponse;

pub type AppResult<T> = Result<T, AppError>;

pub enum AppError {
    NotFound,
    InternalServerError,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let status = match self {
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        };

        AppErrorResponse::send(status, None)
    }
}
