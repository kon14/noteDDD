use std::ops::Sub;
use uuid::Uuid;

use super::*;

#[test]
fn test_invalid_note_with_empty_title() {
    let data = NoteData {
        id: Uuid::new_v4(),
        user_id: Uuid::new_v4(),
        title: String::new(),
        content: "Valid content".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    let result = Note::try_from(data);

    assert!(result.is_err());
    if let Err(AppError::BadRequest(base_error)) = result {
        assert_eq!(base_error.public_info, "Note title cannot be empty!");
    } else {
        panic!("Expected BadRequest error for empty title");
    }
}

#[test]
fn test_invalid_note_with_empty_content() {
    let data = NoteData {
        id: Uuid::new_v4(),
        user_id: Uuid::new_v4(),
        title: "Valid title".to_string(),
        content: String::new(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    let result = Note::try_from(data);

    assert!(result.is_err());
    if let Err(AppError::BadRequest(base_error)) = result {
        assert_eq!(base_error.public_info, "Note content cannot be empty!");
    } else {
        panic!("Expected BadRequest error for empty content");
    }
}

#[test]
fn test_valid_note_creation() {
    let id = Uuid::new_v4();
    let user_id = Uuid::new_v4();
    let title = "Valid title".to_string();
    let content = "Valid content".to_string();
    let created_at = Utc::now();
    let updated_at = created_at;
    let data = NoteData {
        id,
        user_id,
        title: title.clone(),
        content: content.clone(),
        created_at,
        updated_at,
    };
    let result = Note::try_from(data);

    assert!(result.is_ok());
    let note = result.unwrap();

    assert_eq!(note.id(), id);
    assert_eq!(note.user_id(), user_id);
    assert_eq!(note.title(), &title);
    assert_eq!(note.content(), &content);
    assert_eq!(note.user_id(), user_id);
    assert_eq!(note.created_at(), created_at);
    assert_eq!(note.updated_at(), updated_at);
}

#[test]
fn test_note_set_title() {
    let data = create_valid_note_data();
    let original_updated_at = data.updated_at;
    let mut note = Note::try_from(data).unwrap();

    // Small delay to ensure timestamp difference
    std::thread::sleep(std::time::Duration::from_millis(1));

    let new_title = "Updated title".to_string();
    let result = note.set_title(new_title.clone());

    assert!(result.is_ok());
    assert_eq!(note.title(), &new_title);
    assert!(note.updated_at() > original_updated_at);
}

#[test]
fn test_note_set_title_empty_fails() {
    let data = create_valid_note_data();
    let original_title = data.title.clone();
    let original_updated_at = data.updated_at;
    let mut note = Note::try_from(data).unwrap();

    // Small delay to ensure timestamp difference
    std::thread::sleep(std::time::Duration::from_millis(1));

    let result = note.set_title(String::new());

    assert!(result.is_err());
    if let Err(AppError::BadRequest(base_error)) = result {
        assert_eq!(base_error.public_info, "Note title cannot be empty!");
    } else {
        panic!("Expected BadRequest error for empty title");
    }

    // Content should remain unchanged
    assert_eq!(note.title(), original_title);
    assert_eq!(note.updated_at(), original_updated_at);
}

#[test]
fn test_note_set_content() {
    let data = create_valid_note_data();
    let original_updated_at = data.updated_at;
    let mut note = Note::try_from(data).unwrap();

    // Small delay to ensure timestamp difference
    std::thread::sleep(std::time::Duration::from_millis(1));

    let new_content = "Updated content".to_string();
    let result = note.set_title(new_content.clone());

    assert!(result.is_ok());
    assert_eq!(note.title(), &new_content);
    assert!(note.updated_at() > original_updated_at);
}

#[test]
fn test_note_set_content_empty_fails() {
    let data = create_valid_note_data();
    let original_content = data.content.clone();
    let original_updated_at = data.updated_at;
    let mut note = Note::try_from(data).unwrap();

    // Small delay to ensure timestamp difference
    std::thread::sleep(std::time::Duration::from_millis(1));

    let result = note.set_content(String::new());

    assert!(result.is_err());
    if let Err(AppError::BadRequest(base_error)) = result {
        assert_eq!(base_error.public_info, "Note content cannot be empty!");
    } else {
        panic!("Expected BadRequest error for empty content");
    }

    // Content should remain unchanged
    assert_eq!(note.content(), original_content);
    assert_eq!(note.updated_at(), original_updated_at);
}

fn create_valid_note_data() -> NoteData {
    NoteData {
        id: Uuid::new_v4(),
        user_id: Uuid::new_v4(),
        title: "Valid title".to_string(),
        content: "Valid content".to_string(),
        created_at: Utc::now().sub(chrono::Duration::days(1)),
        updated_at: Utc::now(),
    }
}
