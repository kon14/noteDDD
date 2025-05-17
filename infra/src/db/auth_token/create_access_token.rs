use chrono::{DateTime, Utc};
use sqlx::{PgExecutor, Postgres};
use uuid::Uuid;

use crate::models::token::AccessTokenPg;
use common::error::AppError;

pub(crate) async fn create_access_token<'a>(
    db: impl PgExecutor<'a, Database = Postgres>,
    token_data: CreateAccessTokenDataPg,
) -> Result<AccessTokenPg, AppError> {
    const INTERNAL_ERR_STR: &str = "Failed to create access token!";

    sqlx::query_as!(
        AccessTokenPg,
        r#"
        INSERT INTO access_tokens (
            id,
            user_id,
            jwt,
            expires_at
        )
        VALUES ($1, $2, $3, $4)
        RETURNING
            id,
            user_id,
            jwt,
            expires_at
        "#,
        token_data.id,
        token_data.user_id,
        token_data.jwt,
        token_data.expires_at,
    )
    .fetch_one(db)
    .await
    .map_err(|err| AppError::internal_with_private(INTERNAL_ERR_STR, err.to_string()))
}

#[derive(Debug)]
pub(crate) struct CreateAccessTokenDataPg {
    pub(crate) id: Uuid,
    pub(crate) user_id: Uuid,
    pub(crate) jwt: String,
    pub(crate) expires_at: DateTime<Utc>,
}
