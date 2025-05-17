use axum::{extract::State, Json};
use serde::Deserialize;
use utoipa::ToSchema;

use crate::{dtos::AuthTokenPairDto, types::error::PresentationError};
use app::{state::AppState, usecases::auth::AuthLoginInput};

#[derive(Deserialize, ToSchema)]
pub(crate) struct AuthLoginHttpRequestBody {
    pub(crate) email: String,
    pub(crate) password: String,
}

/// Authenticates a User.
#[utoipa::path(
    tag = "Authentication",
    post,
    path = "/login",
    responses(
        (status = 200, description = "Success", body = AuthTokenPairDto),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Failure"),
    ),
)]
pub(crate) async fn auth_login(
    State(state): State<AppState>,
    Json(payload): Json<AuthLoginHttpRequestBody>,
) -> Result<Json<AuthTokenPairDto>, PresentationError> {
    let AppState {
        auth_login_use_case,
        ..
    } = state;

    let input = payload.into();
    let token_pair = auth_login_use_case.execute(input).await?;

    let token_pair_dto = token_pair.into();
    Ok(Json(token_pair_dto))
}

impl From<AuthLoginHttpRequestBody> for AuthLoginInput {
    fn from(payload: AuthLoginHttpRequestBody) -> Self {
        Self {
            email: payload.email,
            password: payload.password,
        }
    }
}
