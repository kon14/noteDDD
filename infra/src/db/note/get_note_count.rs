use sqlx::PgExecutor;
use uuid::Uuid;

use common::error::AppError;

pub(crate) async fn get_note_count<'a>(
    db: impl PgExecutor<'a>,
    user_id: Option<Uuid>,
) -> Result<u32, AppError> {
    const INTERNAL_ERR_STR: &str = "Failed to retrieve note count!";

    sqlx::query_scalar!(
        r#"
        SELECT COUNT(*) as "total_count!"
        FROM notes
        WHERE ($1::uuid IS NULL OR user_id = $1)
        "#,
        user_id,
    )
    .fetch_one(db)
    .await
    .map(|count| count as u32)
    .map_err(|err| AppError::internal_with_private(INTERNAL_ERR_STR, err.to_string()))
}
