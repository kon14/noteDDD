use axum::{
    extract::{Query, State},
    Json,
};
use serde::Serialize;
use utoipa::ToSchema;

use crate::{
    auth::extractors::AuthContextAccessExtractor,
    dtos::NoteDto,
    types::{error::PresentationError, params::PaginationParams},
};
use app::{state::AppState, usecases::note::GetNotesInput};
use dmn::repos::note::GetNotesResponse;

#[derive(Serialize, ToSchema)]
pub(crate) struct GetNotesHttpResponseBody {
    pub(crate) notes: Vec<NoteDto>,
    pub(crate) count: u32,
}

/// Retrieves multiple Notes.
#[utoipa::path(
    tag = "Notes",
    get,
    path = "/",
    params(
        PaginationParams,
    ),
    responses(
        (status = 200, description = "Success", body = GetNotesHttpResponseBody),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Failure"),
    ),
    security(
        ("bearerAuth" = [])
    ),
)]
pub(crate) async fn get_notes(
    State(state): State<AppState>,
    Query(pagination): Query<PaginationParams>,
    AuthContextAccessExtractor(auth_ctx): AuthContextAccessExtractor,
) -> Result<Json<GetNotesHttpResponseBody>, PresentationError> {
    let AppState {
        get_notes_use_case, ..
    } = state;

    let input = get_use_case_input(pagination);
    let dmn_res = get_notes_use_case.execute(auth_ctx, input).await?;

    let note_dtos = dmn_res.notes.into_iter().map(|note| note.into()).collect();
    let http_res = GetNotesHttpResponseBody {
        notes: note_dtos,
        count: dmn_res.count,
    };
    Ok(Json(http_res))
}

fn get_use_case_input(pagination: PaginationParams) -> GetNotesInput {
    GetNotesInput {
        pagination: pagination.into(),
    }
}

impl From<GetNotesResponse> for GetNotesHttpResponseBody {
    fn from(dmn_res: GetNotesResponse) -> Self {
        let notes = dmn_res.notes.into_iter().map(|note| note.into()).collect();
        Self {
            notes,
            count: dmn_res.count,
        }
    }
}
