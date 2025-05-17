use axum::extract::{Json, Path, State};
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    auth::extractors::AuthContextAccessExtractor, dtos::NoteDto, types::error::PresentationError,
};
use app::{state::AppState, usecases::note::UpdateNoteInput};

#[derive(Deserialize, ToSchema)]
pub(crate) struct UpdateNoteHttpRequestBody {
    pub(crate) title: String,
    pub(crate) content: String,
}

/// Updates a Note.
#[utoipa::path(
    tag = "Notes",
    put,
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
pub(crate) async fn update_note(
    State(state): State<AppState>,
    Path(note_id): Path<Uuid>,
    AuthContextAccessExtractor(auth_ctx): AuthContextAccessExtractor,
    Json(payload): Json<UpdateNoteHttpRequestBody>,
) -> Result<Json<NoteDto>, PresentationError> {
    let AppState {
        update_note_use_case,
        ..
    } = state;

    let input = get_use_case_input(note_id, payload);
    let note = update_note_use_case.execute(auth_ctx, input.into()).await?;

    let note_dto = note.into();
    Ok(Json(note_dto))
}

fn get_use_case_input(note_id: Uuid, payload: UpdateNoteHttpRequestBody) -> UpdateNoteInput {
    UpdateNoteInput {
        note_id,
        title: payload.title,
        content: payload.content,
    }
}
