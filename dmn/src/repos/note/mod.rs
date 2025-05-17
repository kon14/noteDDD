use async_trait::async_trait;
use uuid::Uuid;

use crate::entities::note::{CreateNoteData, Note, UpdateNoteData};
use common::{error::AppError, params::PaginationParams, tx::ctx::TransactionContext};

#[async_trait]
pub trait NoteRepository: Send + Sync {
    async fn get_note(
        &self,
        ctx: Option<&mut dyn TransactionContext>,
        note_id: Uuid,
        user_id: Option<Uuid>,
    ) -> Result<Note, AppError>;

    async fn get_notes(
        &self,
        ctx: Option<&mut dyn TransactionContext>,
        pagination: &PaginationParams,
        user_id: Option<Uuid>,
    ) -> Result<GetNotesResponse, AppError>;

    async fn create_note(
        &self,
        ctx: Option<&mut dyn TransactionContext>,
        note_data: CreateNoteData,
    ) -> Result<Note, AppError>;

    async fn update_note(
        &self,
        ctx: Option<&mut dyn TransactionContext>,
        note_id: Uuid,
        note_data: UpdateNoteData,
        user_id: Option<Uuid>,
    ) -> Result<Note, AppError>;

    async fn delete_note(
        &self,
        ctx: Option<&mut dyn TransactionContext>,
        note_id: Uuid,
        user_id: Option<Uuid>,
    ) -> Result<(), AppError>;
}

#[derive(Debug)]
pub struct GetNotesResponse {
    pub notes: Vec<Note>,
    pub count: u32,
}
