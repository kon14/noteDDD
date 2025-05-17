use chrono::{DateTime, Utc};
use uuid::Uuid;

use dmn::entities::note::{Note, NoteData};

#[derive(Debug)]
pub(crate) struct NotePg {
    pub(crate) id: Uuid,
    pub(crate) user_id: Uuid,
    pub(crate) title: String,
    pub(crate) content: String,
    pub(crate) created_at: DateTime<Utc>,
    pub(crate) updated_at: DateTime<Utc>,
}

impl From<Note> for NotePg {
    fn from(dmn_note: Note) -> Self {
        Self {
            id: dmn_note.id(),
            user_id: dmn_note.user_id(),
            title: dmn_note.title().into(),
            content: dmn_note.content().into(),
            created_at: dmn_note.created_at(),
            updated_at: dmn_note.updated_at(),
        }
    }
}

impl From<NotePg> for NoteData {
    fn from(pg_note: NotePg) -> Self {
        NoteData {
            id: pg_note.id,
            user_id: pg_note.user_id,
            title: pg_note.title,
            content: pg_note.content,
            created_at: pg_note.created_at,
            updated_at: pg_note.updated_at,
        }
    }
}
