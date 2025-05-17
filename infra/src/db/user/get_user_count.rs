use sqlx::PgExecutor;

use common::error::AppError;

pub(crate) async fn get_user_count<'a>(db: impl PgExecutor<'a>) -> Result<u32, AppError> {
    const INTERNAL_ERR_STR: &str = "Failed to retrieve user count!";

    sqlx::query_scalar!(
        r#"
        SELECT COUNT(*) as "total_count!"
        FROM users
        "#,
    )
    .fetch_one(db)
    .await
    .map(|count| count as u32)
    .map_err(|err| AppError::internal_with_private(INTERNAL_ERR_STR, err.to_string()))
}
