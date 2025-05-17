use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use dmn::entities::note::Note;

#[derive(Debug, Serialize, ToSchema)]
#[schema(title = "Note")]
pub(crate) struct NoteDto {
    pub(crate) id: Uuid,
    pub(crate) title: String,
    pub(crate) content: String,
    pub(crate) created_at: DateTime<Utc>,
    pub(crate) updated_at: DateTime<Utc>,
}

impl From<Note> for NoteDto {
    fn from(note: Note) -> Self {
        Self {
            id: note.id(),
            title: note.title().into(),
            content: note.content().into(),
            created_at: note.created_at(),
            updated_at: note.updated_at(),
        }
    }
}
