use sqlx::PgExecutor;
use uuid::Uuid;

use crate::models::note::NotePg;
use common::{error::AppError, params::PaginationParams};

pub(crate) async fn get_notes<'a>(
    db: impl PgExecutor<'a>,
    pagination: &PaginationParams,
    user_id: Option<Uuid>,
) -> Result<Vec<NotePg>, AppError> {
    const INTERNAL_ERR_STR: &str = "Failed to retrieve notes!";

    sqlx::query_as!(
        NotePg,
        r#"
        SELECT
            id,
            user_id,
            title,
            content,
            created_at,
            updated_at
        FROM notes
        WHERE ($1::uuid IS NULL OR user_id = $1)
        ORDER BY created_at DESC
        OFFSET $2
        LIMIT $3
        "#,
        user_id,
        pagination.skip as i64,
        pagination.limit as i64,
    )
    .fetch_all(db)
    .await
    .map_err(|err| AppError::internal_with_private(INTERNAL_ERR_STR, err.to_string()))
}
