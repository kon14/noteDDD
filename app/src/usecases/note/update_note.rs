use std::sync::Arc;
use uuid::Uuid;

use crate::auth::context::AuthAccessContext;
use common::error::AppError;
use dmn::{
    entities::note::{Note, UpdateNoteData},
    repos::note::NoteRepository,
};

#[derive(Clone)]
pub struct UpdateNoteUseCase {
    note_repo: Arc<dyn NoteRepository + Send + Sync>,
}

impl UpdateNoteUseCase {
    pub fn new(note_repo: Arc<dyn NoteRepository + Send + Sync>) -> Self {
        Self { note_repo }
    }

    pub async fn execute(
        &self,
        auth_ctx: AuthAccessContext,
        input: UpdateNoteInput,
    ) -> Result<Note, AppError> {
        let note_id = input.note_id;
        let note_data = input.try_into()?;
        let note = self
            .note_repo
            .update_note(None, note_id, note_data, Some(auth_ctx.user.id()))
            .await?;
        Ok(note)
    }
}

#[derive(Debug)]
pub struct UpdateNoteInput {
    pub note_id: Uuid,
    pub title: String,
    pub content: String,
}

impl TryFrom<UpdateNoteInput> for UpdateNoteData {
    type Error = AppError;

    fn try_from(input: UpdateNoteInput) -> Result<Self, Self::Error> {
        let data = Self {
            title: input.title,
            content: input.content,
        };
        let valid_data = data.validate()?;
        Ok(valid_data)
    }
}
