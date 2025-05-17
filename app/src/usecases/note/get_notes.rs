use std::sync::Arc;

use crate::auth::context::AuthAccessContext;
use common::{error::AppError, params::PaginationParams};
use dmn::repos::note::{GetNotesResponse, NoteRepository};

#[derive(Clone)]
pub struct GetNotesUseCase {
    note_repo: Arc<dyn NoteRepository + Send + Sync>,
}

impl GetNotesUseCase {
    pub fn new(note_repo: Arc<dyn NoteRepository + Send + Sync>) -> Self {
        Self { note_repo }
    }

    pub async fn execute(
        &self,
        auth_ctx: AuthAccessContext,
        input: GetNotesInput,
    ) -> Result<GetNotesResponse, AppError> {
        let notes = self
            .note_repo
            .get_notes(None, &input.pagination, Some(auth_ctx.user.id()))
            .await?;
        Ok(notes)
    }
}

#[derive(Debug)]
pub struct GetNotesInput {
    pub pagination: PaginationParams,
}
