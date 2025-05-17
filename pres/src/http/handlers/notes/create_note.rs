use axum::{extract::State, Json};
use serde::Deserialize;
use utoipa::ToSchema;

use crate::{
    auth::extractors::AuthContextAccessExtractor, dtos::NoteDto, types::error::PresentationError,
};
use app::{state::AppState, usecases::note::CreateNoteInput};

#[derive(Deserialize, ToSchema)]
pub(crate) struct CreateNoteHttpRequestBody {
    pub(crate) title: String,
    pub(crate) content: String,
}

/// Creates a new Note.
#[utoipa::path(
    tag = "Notes",
    post,
    path = "/",
    responses(
        (status = 200, description = "Success", body = NoteDto),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Failure"),
    ),
    security(
        ("bearerAuth" = [])
    ),
)]
pub(crate) async fn create_note(
    State(state): State<AppState>,
    AuthContextAccessExtractor(auth_ctx): AuthContextAccessExtractor,
    Json(payload): Json<CreateNoteHttpRequestBody>,
) -> Result<Json<NoteDto>, PresentationError> {
    let AppState {
        create_note_use_case,
        ..
    } = state;

    let input = payload.into();
    let note = create_note_use_case.execute(auth_ctx, input).await?;

    let note_dto = note.into();
    Ok(Json(note_dto))
}

impl From<CreateNoteHttpRequestBody> for CreateNoteInput {
    fn from(payload: CreateNoteHttpRequestBody) -> Self {
        Self {
            title: payload.title,
            content: payload.content,
        }
    }
}
