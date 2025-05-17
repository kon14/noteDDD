use sqlx::{PgExecutor, Postgres};

use app::types::auth_token::UniqueAccessTokenIdentifier;
use common::error::AppError;

pub(crate) async fn delete_token_pair<'a>(
    db: impl PgExecutor<'a, Database = Postgres>,
    access_token_id: &UniqueAccessTokenIdentifier,
) -> Result<(), AppError> {
    const INTERNAL_ERR_STR: &str = "Failed to delete auth token pair!";
    #[allow(non_snake_case)]
    let NOT_FOUND_ERR_STR =
        format!("Auth token pair (access_token_id: {access_token_id}) doesn't exist!");

    let (id, jwt) = match access_token_id {
        UniqueAccessTokenIdentifier::Id(id) => (Some(id), None),
        UniqueAccessTokenIdentifier::Jwt(ref jwt) => (None, Some(jwt)),
    };

    // Auto-cascades related refresh_tokens entries
    sqlx::query!(
        r#"
        DELETE FROM access_tokens
        WHERE
            (id = $1 OR jwt = $2)
        "#,
        id,
        jwt,
    )
    .execute(db)
    .await
    .map_err(|err| AppError::internal_with_private(INTERNAL_ERR_STR, err.to_string()))
    .and_then(|result| match result.rows_affected() {
        0 => Err(AppError::not_found(NOT_FOUND_ERR_STR)),
        _ => Ok(()),
    })
}
