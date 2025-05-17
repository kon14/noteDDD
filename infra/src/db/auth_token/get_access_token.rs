use sqlx::PgExecutor;

use crate::models::token::AccessTokenPg;
use app::types::auth_token::UniqueAccessTokenIdentifier;
use common::error::AppError;

pub(crate) async fn get_access_token<'a>(
    db: impl PgExecutor<'a>,
    access_token_id: &UniqueAccessTokenIdentifier,
) -> Result<AccessTokenPg, AppError> {
    const INTERNAL_ERR_STR: &str = "Failed to retrieve access token!";
    #[allow(non_snake_case)]
    let NOT_FOUND_ERR_STR = format!("Access token ({access_token_id}) doesn't exist!");

    let (id, jwt) = match access_token_id {
        UniqueAccessTokenIdentifier::Id(id) => (Some(id), None),
        UniqueAccessTokenIdentifier::Jwt(ref jwt) => (None, Some(jwt)),
    };

    sqlx::query_as!(
        AccessTokenPg,
        r#"
        SELECT
            id,
            user_id,
            jwt,
            expires_at
        FROM access_tokens
        WHERE id = $1 OR jwt = $2
        "#,
        id,
        jwt,
    )
    .fetch_one(db)
    .await
    .map_err(|err| match err {
        sqlx::Error::RowNotFound => AppError::not_found(NOT_FOUND_ERR_STR),
        _ => AppError::internal_with_private(INTERNAL_ERR_STR, err.to_string()),
    })
}
