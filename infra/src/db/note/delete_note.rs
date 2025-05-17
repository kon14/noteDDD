use sqlx::PgExecutor;

use common::error::AppError;
use uuid::Uuid;

pub(crate) async fn delete_note<'a>(
    db: impl PgExecutor<'a>,
    note_id: Uuid,
    user_id: Option<Uuid>,
) -> Result<(), AppError> {
    #[allow(non_snake_case)]
    let INTERNAL_ERR_STR = format!("Failed to delete note ({note_id})!");
    #[allow(non_snake_case)]
    let NOT_FOUND_ERR_STR = format!("Note ({note_id}) doesn't exist!");

    let res = sqlx::query!(
        r#"
        DELETE FROM notes
        WHERE
            id = $1 AND
            ($2::uuid IS NULL OR user_id = $2)
        "#,
        note_id,
        user_id,
    )
    .execute(db)
    .await
    .map_err(|err| AppError::internal_with_private(INTERNAL_ERR_STR, err.to_string()))?;

    if res.rows_affected() == 0 {
        Err(AppError::not_found(NOT_FOUND_ERR_STR))
    } else {
        Ok(())
    }
}
