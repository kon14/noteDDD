use std::sync::Arc;
use uuid::Uuid;

use crate::auth::context::AuthAccessContext;
use common::error::AppError;
use dmn::{entities::note::Note, repos::note::NoteRepository};

#[derive(Clone)]
pub struct GetNoteUseCase {
    note_repo: Arc<dyn NoteRepository + Send + Sync>,
}

impl GetNoteUseCase {
    pub fn new(note_repo: Arc<dyn NoteRepository + Send + Sync>) -> Self {
        Self { note_repo }
    }

    pub async fn execute(
        &self,
        auth_ctx: AuthAccessContext,
        input: GetNoteInput,
    ) -> Result<Note, AppError> {
        let note = self
            .note_repo
            .get_note(None, input.note_id, Some(auth_ctx.user.id()))
            .await?;
        Ok(note)
    }
}

#[derive(Debug)]
pub struct GetNoteInput {
    pub note_id: Uuid,
}
