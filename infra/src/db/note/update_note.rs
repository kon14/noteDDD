use sqlx::PgExecutor;
use uuid::Uuid;

use crate::models::note::NotePg;
use common::error::AppError;

pub(crate) async fn update_note<'a>(
    db: impl PgExecutor<'a>,
    note_id: Uuid,
    note_data: UpdateNoteDataPg,
    user_id: Option<Uuid>,
) -> Result<NotePg, AppError> {
    #[allow(non_snake_case)]
    let INTERNAL_ERR_STR = format!("Failed to update note ({note_id})!");
    #[allow(non_snake_case)]
    let NOT_FOUND_ERR_STR = format!("Note ({note_id}) doesn't exist!");

    sqlx::query_as!(
        NotePg,
        r#"
        UPDATE notes
        SET
            title = $3,
            content = $4
        WHERE
            id = $1 AND
            ($2::uuid IS NULL OR user_id = $2)
        RETURNING
            id,
            user_id,
            title,
            content,
            created_at,
            updated_at
        "#,
        note_id,
        user_id,
        note_data.title,
        note_data.content,
    )
    .fetch_one(db)
    .await
    .map_err(|err| match err {
        sqlx::Error::RowNotFound => AppError::not_found(NOT_FOUND_ERR_STR),
        _ => AppError::internal_with_private(INTERNAL_ERR_STR, err.to_string()),
    })
}

#[derive(Debug)]
pub(crate) struct UpdateNoteDataPg {
    pub(crate) title: String,
    pub(crate) content: String,
}
