use axum::{extract::FromRequestParts, http::request::Parts, RequestPartsExt};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};

use crate::types::error::PresentationError;
use app::{
    auth::context::{AuthAccessContext, AuthRefreshContext},
    state::AppState,
};
use common::error::AppError;

pub(crate) struct AuthContextAccessExtractor(pub(crate) AuthAccessContext);
pub(crate) struct AuthContextRefreshExtractor(pub(crate) AuthRefreshContext);

async fn extract_jwt_from_headers(parts: &mut Parts) -> Result<String, AppError> {
    const UNAUTHORIZED_ERR_STR: &str = "Failed to extract authentication token!";

    let TypedHeader(Authorization(bearer)) = parts
        .extract::<TypedHeader<Authorization<Bearer>>>()
        .await
        .map_err(|err| {
            AppError::unauthorized_with_private(UNAUTHORIZED_ERR_STR, err.to_string())
        })?;
    let token = bearer.token().to_string();
    Ok(token)
}

impl FromRequestParts<AppState> for AuthContextAccessExtractor {
    type Rejection = PresentationError;

    async fn from_request_parts(
        parts: &mut Parts,
        app_state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        const UNAUTHORIZED_ERR_STR: &str = "Failed to authenticate user!";

        let token = extract_jwt_from_headers(parts).await?;
        let ctx = app_state
            .authenticator
            .authenticate_access_token(token)
            .await
            .map_err(|err| err.reword(UNAUTHORIZED_ERR_STR.to_string()))?;
        Ok(Self(ctx))
    }
}

impl FromRequestParts<AppState> for AuthContextRefreshExtractor {
    type Rejection = PresentationError;

    async fn from_request_parts(
        parts: &mut Parts,
        app_state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        const UNAUTHORIZED_ERR_STR: &str = "Failed to authenticate user!";

        let token = extract_jwt_from_headers(parts).await?;
        let ctx = app_state
            .authenticator
            .authenticate_refresh_token(token)
            .await
            .map_err(|err| err.reword(UNAUTHORIZED_ERR_STR.to_string()))?;
        Ok(Self(ctx))
    }
}
