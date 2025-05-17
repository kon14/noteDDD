use chrono::Utc;
use std::ops::Sub;
use uuid::Uuid;

use super::*;

#[test]
fn test_valid_user_creation() {
    let id = Uuid::new_v4();
    let email = create_test_email();
    let created_at = Utc::now();
    let updated_at = created_at;
    let data = UserData {
        id,
        email: email.clone(),
        created_at,
        updated_at,
    };
    let result = User::try_from(data);

    assert!(result.is_ok());
    let user = result.unwrap();

    assert_eq!(user.id(), id);
    assert_eq!(user.email().as_ref(), email.as_ref());
    assert_eq!(user.created_at(), created_at);
    assert_eq!(user.updated_at(), updated_at);
}

#[test]
fn test_user_set_email() {
    let data = create_valid_user_data();
    let original_updated_at = data.updated_at;
    let mut user = User::try_from(data).unwrap();

    // Small delay to ensure timestamp difference
    std::thread::sleep(std::time::Duration::from_millis(1));

    let new_email = Email::try_from("newemail@example.com".to_string()).unwrap();
    let result = user.set_email(new_email.clone());

    assert!(result.is_ok());
    assert_eq!(user.email().as_ref(), "newemail@example.com");
    assert!(user.updated_at() > original_updated_at);
}

#[test]
fn test_unique_user_identifier_display() {
    let user_id = Uuid::new_v4();
    let id_identifier = UniqueUserIdentifier::Id(user_id);
    let email_identifier = UniqueUserIdentifier::Email(create_test_email());

    assert_eq!(id_identifier.to_string(), user_id.to_string());
    assert_eq!(email_identifier.to_string(), "test@example.com");
}

fn create_test_email() -> Email {
    Email::try_from("test@example.com".to_string()).unwrap()
}

fn create_valid_user_data() -> UserData {
    UserData {
        id: Uuid::new_v4(),
        email: create_test_email(),
        created_at: Utc::now().sub(chrono::Duration::days(1)),
        updated_at: Utc::now(),
    }
}
