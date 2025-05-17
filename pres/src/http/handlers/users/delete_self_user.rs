use axum::{extract::State, Json};
use serde::Deserialize;
use utoipa::ToSchema;

use crate::{auth::extractors::AuthContextAccessExtractor, types::error::PresentationError};
use app::{state::AppState, usecases::user::DeleteSelfUserInput};

#[derive(Deserialize, ToSchema)]
pub(crate) struct DeleteSelfUserHttpRequestBody {
    pub(crate) password: String,
}

/// Deletes the authenticated User.
#[utoipa::path(
    tag = "Users",
    delete,
    path = "/self",
    responses(
        (status = 200, description = "Success", body = String),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Failure"),
    ),
    security(
        ("bearerAuth" = [])
    ),
)]
pub(crate) async fn delete_self_user(
    State(state): State<AppState>,
    AuthContextAccessExtractor(auth_ctx): AuthContextAccessExtractor,
    Json(payload): Json<DeleteSelfUserHttpRequestBody>,
) -> Result<String, PresentationError> {
    let AppState {
        delete_self_user_use_case,
        ..
    } = state;

    let input = payload.into();
    delete_self_user_use_case
        .execute(auth_ctx.clone(), input)
        .await?;

    Ok(format!(
        "User ({}) deleted successfully.",
        auth_ctx.user.id()
    ))
}

impl From<DeleteSelfUserHttpRequestBody> for DeleteSelfUserInput {
    fn from(payload: DeleteSelfUserHttpRequestBody) -> Self {
        Self {
            password: payload.password,
        }
    }
}
