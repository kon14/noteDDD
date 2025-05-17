use axum::{extract::State, Json};

use crate::{
    auth::extractors::AuthContextAccessExtractor, dtos::UserDto, types::error::PresentationError,
};
use app::state::AppState;

/// Retrieves the authenticated User.
#[utoipa::path(
    tag = "Users",
    get,
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
pub(crate) async fn get_self_user(
    State(state): State<AppState>,
    AuthContextAccessExtractor(auth_ctx): AuthContextAccessExtractor,
) -> Result<Json<UserDto>, PresentationError> {
    let AppState {
        get_self_user_use_case,
        ..
    } = state;

    let user = get_self_user_use_case.execute(auth_ctx).await?;

    let user_dto = user.into();
    Ok(Json(user_dto))
}
