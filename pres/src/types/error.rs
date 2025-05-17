use axum::http::StatusCode;
use serde_json::json;

use common::error::AppError;

/// A wrapper type extending `common::AppError` with presentation layer semantics.<br />
/// Used to implement `axum::response::IntoResponse` for `common::AppError`.<br />
pub(crate) struct PresentationError(AppError);

impl From<AppError> for PresentationError {
    fn from(err: AppError) -> Self {
        PresentationError(err)
    }
}

impl PresentationError {
    fn status_code(&self) -> StatusCode {
        match self.0 {
            AppError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::Conflict(_) => StatusCode::CONFLICT,
            AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            AppError::Forbidden(_) => StatusCode::FORBIDDEN,
        }
    }
}

impl axum::response::IntoResponse for PresentationError {
    fn into_response(self) -> axum::response::Response {
        self.0.log();

        let body = axum::Json(json!({
            "error": {
                "type": self.0.error_type(),
                "message": self.0.public_info,
            }
        }));
        (self.status_code(), body).into_response()
    }
}
