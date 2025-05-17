use std::sync::Arc;
use uuid::Uuid;

use crate::auth::context::AuthAccessContext;
use common::error::AppError;
use dmn::repos::note::NoteRepository;

#[derive(Clone)]
pub struct DeleteNoteUseCase {
    note_repo: Arc<dyn NoteRepository + Send + Sync>,
}

impl DeleteNoteUseCase {
    pub fn new(note_repo: Arc<dyn NoteRepository + Send + Sync>) -> Self {
        Self { note_repo }
    }

    pub async fn execute(
        &self,
        auth_ctx: AuthAccessContext,
        input: DeleteNoteInput,
    ) -> Result<(), AppError> {
        self.note_repo
            .delete_note(None, input.note_id, Some(auth_ctx.user.id()))
            .await
    }
}

#[derive(Debug)]
pub struct DeleteNoteInput {
    pub note_id: Uuid,
}
