use axum::extract::{Json, State};
use serde::Deserialize;
use utoipa::ToSchema;

use crate::{auth::extractors::AuthContextAccessExtractor, types::error::PresentationError};
use app::{state::AppState, usecases::auth::ChangeUserPasswordInput};

#[derive(Deserialize, ToSchema)]
pub(crate) struct ChangeUserPasswordHttpRequestBody {
    pub(crate) current_password: String,
    pub(crate) new_password: String,
}

/// Updates the authenticated User's password.
#[utoipa::path(
    tag = "Authentication",
    put,
    path = "/password",
    responses(
        (status = 200, description = "Success", body = String),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Failure"),
    ),
    security(
        ("bearerAuth" = [])
    ),
)]
pub(crate) async fn change_user_pass(
    State(state): State<AppState>,
    AuthContextAccessExtractor(auth_ctx): AuthContextAccessExtractor,
    Json(payload): Json<ChangeUserPasswordHttpRequestBody>,
) -> Result<String, PresentationError> {
    let AppState {
        change_user_pass_use_case,
        ..
    } = state;

    let input = payload.into();
    change_user_pass_use_case.execute(auth_ctx, input).await?;

    Ok("User password updated successfully.".to_string())
}

impl From<ChangeUserPasswordHttpRequestBody> for ChangeUserPasswordInput {
    fn from(payload: ChangeUserPasswordHttpRequestBody) -> Self {
        Self {
            current_password: payload.current_password,
            new_password: payload.new_password,
        }
    }
}
