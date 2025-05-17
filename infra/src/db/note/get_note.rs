use sqlx::PgExecutor;

use crate::models::note::NotePg;
use common::error::AppError;
use uuid::Uuid;

pub(crate) async fn get_note<'a>(
    db: impl PgExecutor<'a>,
    note_id: Uuid,
    user_id: Option<Uuid>,
) -> Result<NotePg, AppError> {
    #[allow(non_snake_case)]
    let INTERNAL_ERR_STR = format!("Failed to retrieve note ({note_id})!");
    #[allow(non_snake_case)]
    let NOT_FOUND_ERR_STR = format!("Note ({note_id}) doesn't exist!");

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
        WHERE
            id = $1 AND
            ($2::uuid IS NULL OR user_id = $2)
        "#,
        note_id,
        user_id,
    )
    .fetch_one(db)
    .await
    .map_err(|err| match err {
        sqlx::Error::RowNotFound => AppError::not_found(NOT_FOUND_ERR_STR),
        _ => AppError::internal_with_private(INTERNAL_ERR_STR, err.to_string()),
    })
}
