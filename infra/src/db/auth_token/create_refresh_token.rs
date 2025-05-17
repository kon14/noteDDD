use chrono::{DateTime, Utc};
use sqlx::{PgExecutor, Postgres};
use uuid::Uuid;

use crate::models::token::RefreshTokenPg;
use common::error::AppError;

pub(crate) async fn create_refresh_token<'a>(
    db: impl PgExecutor<'a, Database = Postgres>,
    token_data: CreateRefreshTokenDataPg,
) -> Result<RefreshTokenPg, AppError> {
    const INTERNAL_ERR_STR: &str = "Failed to create refresh token!";

    sqlx::query_as!(
        RefreshTokenPg,
        r#"
        INSERT INTO refresh_tokens (
            id,
            user_id,
            access_token_id,
            jwt,
            expires_at
        )
        VALUES ($1, $2, $3, $4, $5)
        RETURNING
            id,
            user_id,
            access_token_id,
            jwt,
            expires_at
        "#,
        token_data.id,
        token_data.user_id,
        token_data.access_token_id,
        token_data.jwt,
        token_data.expires_at,
    )
    .fetch_one(db)
    .await
    .map_err(|err| AppError::internal_with_private(INTERNAL_ERR_STR, err.to_string()))
}

#[derive(Debug)]
pub(crate) struct CreateRefreshTokenDataPg {
    pub(crate) id: Uuid,
    pub(crate) user_id: Uuid,
    pub(crate) access_token_id: Uuid,
    pub(crate) jwt: String,
    pub(crate) expires_at: DateTime<Utc>,
}
