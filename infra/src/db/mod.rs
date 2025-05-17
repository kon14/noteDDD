use sqlx::{postgres::PgPoolOptions, PgPool};

use common::error::AppError;

pub(crate) mod auth_token;
pub(crate) mod note;
pub(crate) mod user;

pub async fn get_pg_pool(db_url: &str) -> Result<PgPool, AppError> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await
        .map_err(|err| {
            AppError::internal_with_private(
                format!("Failed to connect to Postgres database @ {db_url}"),
                err.to_string(),
            )
        })
}
