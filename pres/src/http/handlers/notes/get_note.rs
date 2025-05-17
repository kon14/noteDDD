use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;

use crate::{
    auth::extractors::AuthContextAccessExtractor, dtos::NoteDto, types::error::PresentationError,
};
use app::{state::AppState, usecases::note::GetNoteInput};

/// Retrieves a Note.
#[utoipa::path(
    tag = "Notes",
    get,
    path = "/{note_id}",
    params(
        ("note_id" = Uuid, Path),
    ),
    responses(
        (status = 200, description = "Success", body = NoteDto),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Failure"),
    ),
    security(
        ("bearerAuth" = [])
    ),
)]
pub(crate) async fn get_note(
    State(state): State<AppState>,
    Path(note_id): Path<Uuid>,
    AuthContextAccessExtractor(auth_ctx): AuthContextAccessExtractor,
) -> Result<Json<NoteDto>, PresentationError> {
    let AppState {
        get_note_use_case, ..
    } = state;

    let input = GetNoteInput { note_id };
    let note = get_note_use_case.execute(auth_ctx, input).await?;

    let note_dto = note.into();
    Ok(Json(note_dto))
}
