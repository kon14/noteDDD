use axum::{extract::State, Json};

use crate::{
    auth::extractors::AuthContextRefreshExtractor, dtos::AuthTokenPairDto,
    types::error::PresentationError,
};
use app::state::AppState;

/// Rotates a User's authentication token pair.
#[utoipa::path(
    tag = "Authentication",
    post,
    path = "/refresh",
    description = "Expects a <strong>refresh token</strong> in the <em>Authorization</em> header.",
    responses(
        (status = 200, description = "Success", body = AuthTokenPairDto),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Failure"),
    ),
    security(
        ("bearerAuth" = [])
    ),
)]
pub(crate) async fn auth_refresh(
    State(state): State<AppState>,
    AuthContextRefreshExtractor(auth_ref_ctx): AuthContextRefreshExtractor,
) -> Result<Json<AuthTokenPairDto>, PresentationError> {
    let AppState {
        auth_refresh_use_case,
        ..
    } = state;

    let token_pair = auth_refresh_use_case.execute(auth_ref_ctx).await?;

    let token_pair_dto = token_pair.into();
    Ok(Json(token_pair_dto))
}
