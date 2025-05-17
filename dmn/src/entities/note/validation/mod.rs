use super::{CreateNoteData, UpdateNoteData};
use common::error::AppError;

impl CreateNoteData {
    pub fn validate(self) -> Result<Self, AppError> {
        validate_title(&self.title)?;
        validate_content(&self.content)?;
        Ok(self)
    }
}
impl UpdateNoteData {
    pub fn validate(self) -> Result<Self, AppError> {
        validate_title(&self.title)?;
        validate_content(&self.content)?;
        Ok(self)
    }
}

pub(crate) fn validate_title(title: &str) -> Result<String, AppError> {
    let title = title.trim().to_string();
    if title.is_empty() {
        return Err(AppError::BadRequest(common::error::BaseError::new(
            "Note title cannot be empty!".to_string(),
            None,
        )));
    }
    Ok(title)
}

pub(crate) fn validate_content(content: &str) -> Result<String, AppError> {
    let content = content.trim().to_string();
    if content.is_empty() {
        return Err(AppError::BadRequest(common::error::BaseError::new(
            "Note content cannot be empty!".to_string(),
            None,
        )));
    }
    Ok(content)
}

#[cfg(test)]
mod tests;
