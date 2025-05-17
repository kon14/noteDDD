use axum::extract::{Json, State};
use serde::Deserialize;
use utoipa::ToSchema;

use crate::{
    auth::extractors::AuthContextAccessExtractor, dtos::UserDto, types::error::PresentationError,
};
use app::{state::AppState, usecases::user::UpdateSelfUserInput};

#[derive(Deserialize, ToSchema)]
pub(crate) struct UpdateSelfUserHttpRequestBody {
    pub(crate) email: String,
}

/// Updates the authenticated User.
#[utoipa::path(
    tag = "Users",
    put,
    path = "/self",
    responses(
        (status = 200, description = "Success", body = UserDto),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Failure"),
    ),
    security(
        ("bearerAuth" = [])
    ),
)]
pub(crate) async fn update_self_user(
    State(state): State<AppState>,
    AuthContextAccessExtractor(auth_ctx): AuthContextAccessExtractor,
    Json(payload): Json<UpdateSelfUserHttpRequestBody>,
) -> Result<Json<UserDto>, PresentationError> {
    let AppState {
        update_self_user_use_case,
        ..
    } = state;

    let input = payload.into();
    let user = update_self_user_use_case.execute(auth_ctx, input).await?;

    let user_dto = user.into();
    Ok(Json(user_dto))
}

impl From<UpdateSelfUserHttpRequestBody> for UpdateSelfUserInput {
    fn from(payload: UpdateSelfUserHttpRequestBody) -> Self {
        Self {
            email: payload.email,
        }
    }
}
