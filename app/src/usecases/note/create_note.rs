use std::sync::Arc;

use crate::auth::context::AuthAccessContext;
use common::error::AppError;
use dmn::{
    entities::note::{CreateNoteData, Note},
    repos::note::NoteRepository,
};

#[derive(Clone)]
pub struct CreateNoteUseCase {
    note_repo: Arc<dyn NoteRepository + Send + Sync>,
}

impl CreateNoteUseCase {
    pub fn new(note_repo: Arc<dyn NoteRepository + Send + Sync>) -> Self {
        Self { note_repo }
    }

    pub async fn execute(
        &self,
        auth_ctx: AuthAccessContext,
        input: CreateNoteInput,
    ) -> Result<Note, AppError> {
        let note_data = input.try_into_dmn(auth_ctx)?;
        let note = self.note_repo.create_note(None, note_data).await?;
        Ok(note)
    }
}

#[derive(Debug)]
pub struct CreateNoteInput {
    pub title: String,
    pub content: String,
}

impl CreateNoteInput {
    pub fn try_into_dmn(self, auth_ctx: AuthAccessContext) -> Result<CreateNoteData, AppError> {
        let data = CreateNoteData {
            user_id: auth_ctx.user.id(),
            title: self.title,
            content: self.content,
        };
        let valid_data = data.validate()?;
        Ok(valid_data)
    }
}
