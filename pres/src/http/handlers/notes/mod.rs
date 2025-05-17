mod create_note;
mod delete_note;
mod get_note;
mod get_notes;
mod update_note;

use create_note::*;
use delete_note::*;
use get_note::*;
use get_notes::*;
use update_note::*;

use utoipa::OpenApi;

use app::state::AppState;

#[derive(OpenApi)]
#[openapi(
    paths(
        // Notes
        create_note,
        get_note,
        get_notes,
        update_note,
        delete_note,
    ),
    tags(
        (name = "Notes"),
    )
)]
pub struct NotesApiDoc;

pub fn declare_routes(base_path: &str) -> axum::Router<AppState> {
    axum::Router::new()
        .route(&format!("{base_path}"), axum::routing::post(create_note))
        .route(&format!("{base_path}"), axum::routing::get(get_notes))
        .route(
            &format!("{base_path}/{{note_id}}"),
            axum::routing::get(get_note),
        )
        .route(
            &format!("{base_path}/{{note_id}}"),
            axum::routing::put(update_note),
        )
        .route(
            &format!("{base_path}/{{note_id}}"),
            axum::routing::delete(delete_note),
        )
}
