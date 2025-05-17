mod validation;

use chrono::{DateTime, Utc};
use uuid::Uuid;

use common::error::AppError;

#[derive(Debug, Clone)]
pub struct Note {
    id: Uuid,
    user_id: Uuid,
    title: String,
    content: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl TryFrom<NoteData> for Note {
    type Error = AppError;

    fn try_from(data: NoteData) -> Result<Self, Self::Error> {
        let title = validation::validate_title(&data.title)?;
        let content = validation::validate_content(&data.content)?;

        Ok(Self {
            id: data.id,
            user_id: data.user_id,
            title,
            content,
            created_at: data.created_at,
            updated_at: data.updated_at,
        })
    }
}

impl Note {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn user_id(&self) -> Uuid {
        self.user_id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

impl Note {
    pub fn set_title(&mut self, title: String) -> Result<(), AppError> {
        let title = validation::validate_title(&title)?;
        self.title = title;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn set_content(&mut self, content: String) -> Result<(), AppError> {
        let content = validation::validate_content(&content)?;
        self.content = content;
        self.updated_at = Utc::now();
        Ok(())
    }
}

/// [`Note`] entity pre-validation data struct.
pub struct NoteData {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// [`Note`] entity creation pre-validation utility data struct.
#[derive(Debug)]
pub struct CreateNoteData {
    pub user_id: Uuid,
    pub title: String,
    pub content: String,
}

/// [`Note`] entity update pre-validation utility data struct.
#[derive(Debug)]
pub struct UpdateNoteData {
    pub title: String,
    pub content: String,
}

#[cfg(test)]
mod tests;
