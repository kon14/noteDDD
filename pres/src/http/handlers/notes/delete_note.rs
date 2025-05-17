use axum::extract::{Path, State};
use uuid::Uuid;

use crate::{auth::extractors::AuthContextAccessExtractor, types::error::PresentationError};
use app::{state::AppState, usecases::note::DeleteNoteInput};

/// Deletes a Note.
#[utoipa::path(
    tag = "Notes",
    delete,
    path = "/{note_id}",
    params(
        ("note_id" = Uuid, Path),
    ),
    responses(
        (status = 200, description = "Success", body = String),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Failure"),
    ),
    security(
        ("bearerAuth" = [])
    ),
)]
pub(crate) async fn delete_note(
    State(state): State<AppState>,
    Path(note_id): Path<Uuid>,
    AuthContextAccessExtractor(auth_ctx): AuthContextAccessExtractor,
) -> Result<String, PresentationError> {
    let AppState {
        delete_note_use_case,
        ..
    } = state;

    let input = DeleteNoteInput { note_id };
    delete_note_use_case.execute(auth_ctx, input).await?;

    Ok(format!("Note ({}) deleted successfully.", note_id))
}
