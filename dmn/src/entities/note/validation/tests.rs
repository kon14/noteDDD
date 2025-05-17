use chrono::Utc;
use uuid::Uuid;

use super::*;
use crate::entities::note::{Note, NoteData};

#[test]
fn test_validate_title_rejects_empty_title() {
    let result = validate_title("");
    assert!(result.is_err());
    if let Err(AppError::BadRequest(base_error)) = result {
        assert_eq!(base_error.public_info, "Note title cannot be empty!");
    } else {
        panic!("Expected BadRequest error for empty title");
    }
}

#[test]
fn test_validate_title_accepts_non_empty_title() {
    let result = validate_title("Title");
    assert!(result.is_ok());
}

#[test]
fn test_validate_content_rejects_empty_content() {
    let result = validate_content("");
    assert!(result.is_err());
    if let Err(AppError::BadRequest(base_error)) = result {
        assert_eq!(base_error.public_info, "Note content cannot be empty!");
    } else {
        panic!("Expected BadRequest error for empty content");
    }
}

#[test]
fn test_validate_content_accepts_non_empty_content() {
    let result = validate_content("Content");
    assert!(result.is_ok());
}

#[test]
fn test_try_from_valid_data_creates_note() {
    let data = NoteData {
        id: Uuid::new_v4(),
        user_id: Uuid::new_v4(),
        title: "Valid Title".to_string(),
        content: "Valid content".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    let note = Note::try_from(data);
    assert!(note.is_ok());
    let note = note.unwrap();
    assert_eq!(note.title, "Valid Title");
    assert_eq!(note.content, "Valid content");
}

#[test]
fn test_try_from_invalid_title_returns_error() {
    let data = NoteData {
        id: Uuid::new_v4(),
        user_id: Uuid::new_v4(),
        title: "".to_string(),
        content: "Valid content".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    let err = Note::try_from(data).unwrap_err();
    match err {
        AppError::BadRequest(base_error) => {
            assert_eq!(base_error.public_info, "Note title cannot be empty!");
        }
        _ => panic!("Expected BadRequest error"),
    }
}

#[test]
fn test_try_from_invalid_content_returns_error() {
    let data = NoteData {
        id: Uuid::new_v4(),
        user_id: Uuid::new_v4(),
        title: "Valid Title".to_string(),
        content: "".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    let err = Note::try_from(data).unwrap_err();
    match err {
        AppError::BadRequest(base_error) => {
            assert_eq!(base_error.public_info, "Note content cannot be empty!");
        }
        _ => panic!("Expected BadRequest error"),
    }
}
