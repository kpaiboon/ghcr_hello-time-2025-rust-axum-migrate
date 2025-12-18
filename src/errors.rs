use std::sync::{PoisonError, RwLockReadGuard, RwLockWriteGuard};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum HttpAppError {
    #[error("An element with the same ID already exists")]
    Conflict,
    #[error("Not found")]
    NotFound,
    #[error("Poison error {0}")]
    LockError(String),
}

impl IntoResponse for HttpAppError {
    fn into_response(self) -> Response {
        let status = match self {
            HttpAppError::Conflict => StatusCode::CONFLICT,
            HttpAppError::NotFound => StatusCode::NOT_FOUND,
            HttpAppError::LockError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status, Json(self.to_string())).into_response()
    }
}

impl<T> From<PoisonError<RwLockReadGuard<'_, T>>> for HttpAppError {
    fn from(_: PoisonError<RwLockReadGuard<'_, T>>) -> Self {
        HttpAppError::LockError("Read Lock was poisoned".to_string())
    }
}

impl<T> From<PoisonError<RwLockWriteGuard<'_, T>>> for HttpAppError {
    fn from(_: PoisonError<RwLockWriteGuard<'_, T>>) -> Self {
        HttpAppError::LockError("Write Lock was poisoned".to_string())
    }
}
