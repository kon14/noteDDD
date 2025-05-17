use sqlx::PgExecutor;

use crate::models::user::UserPg;
use common::{error::AppError, params::PaginationParams};

pub(crate) async fn get_users<'a>(
    db: impl PgExecutor<'a>,
    pagination: &PaginationParams,
) -> Result<Vec<UserPg>, AppError> {
    const INTERNAL_ERR_STR: &str = "Failed to retrieve users!";

    sqlx::query_as!(
        UserPg,
        r#"
        SELECT
            id,
            email,
            created_at,
            updated_at
        FROM users
        ORDER BY id ASC
        OFFSET $1
        LIMIT $2
        "#,
        pagination.skip as i64,
        pagination.limit as i64,
    )
    .fetch_all(db)
    .await
    .map_err(|err| AppError::internal_with_private(INTERNAL_ERR_STR, err.to_string()))
}
