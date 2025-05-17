use sqlx::PgExecutor;
use uuid::Uuid;

use crate::models::note::NotePg;
use common::error::AppError;

pub(crate) async fn create_note<'a>(
    db: impl PgExecutor<'a>,
    note_data: CreateNoteDataPg,
) -> Result<NotePg, AppError> {
    const INTERNAL_ERR_STR: &str = "Failed to create note!";

    sqlx::query_as!(
        NotePg,
        r#"
        INSERT INTO notes (
            user_id,
            title,
            content
        )
        VALUES ($1, $2, $3)
        RETURNING
            id,
            user_id,
            title,
            content,
            created_at,
            updated_at
        "#,
        note_data.user_id,
        note_data.title,
        note_data.content,
    )
    .fetch_one(db)
    .await
    .map_err(|err| AppError::internal_with_private(INTERNAL_ERR_STR, err.to_string()))
}

#[derive(Debug)]
pub(crate) struct CreateNoteDataPg {
    pub(crate) user_id: Uuid,
    pub(crate) title: String,
    pub(crate) content: String,
}
