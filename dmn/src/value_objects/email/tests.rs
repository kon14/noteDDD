use super::*;

#[test]
fn test_valid_email_creation() {
    let valid_emails = vec![
        "test@example.com",
        "user.name@domain.co.uk",
        "user+tag@example.org",
        "user123@test-domain.com",
        "a@b.co",
        "test.email.with+symbol@example.com",
        "user_name@example-domain.com",
    ];

    for email_str in valid_emails {
        let result = Email::try_from(email_str.to_string());
        assert!(result.is_ok(), "Failed to create email from: {}", email_str);

        let email = result.unwrap();
        assert_eq!(email.as_ref(), email_str);
    }
}

#[test]
fn test_invalid_email_creation() {
    let invalid_emails = vec![
        "",
        "invalid",
        "@domain.com",
        "user@",
        "user@domain",
        "user.domain.com",
        "user@domain.",
        "user@@domain.com",
        "user@domain@com",
        "user name@domain.com",
        "user@domain .com",
        "user@domain..com",
    ];

    for email_str in invalid_emails {
        let result = Email::try_from(email_str.to_string());
        assert!(
            result.is_err(),
            "Should have failed for invalid email: {}",
            email_str
        );

        if let Err(app_error) = result {
            assert!(app_error.to_string().contains("Invalid email address"));
        }
    }
}

#[test]
fn test_email_deref() {
    let email = Email::try_from("test@example.com".to_string()).unwrap();
    assert_eq!(&*email, "test@example.com");
}

#[test]
fn test_email_as_ref() {
    let email = Email::try_from("test@example.com".to_string()).unwrap();
    let email_str: &str = email.as_ref();
    assert_eq!(email_str, "test@example.com");
}

#[test]
fn test_email_display() {
    let email = Email::try_from("test@example.com".to_string()).unwrap();
    assert_eq!(format!("{}", email), "test@example.com");
}

#[test]
fn test_email_edge_cases() {
    // Test maximum reasonable email length
    let long_local = "a".repeat(64);
    let long_domain = "b".repeat(63);
    let long_email = format!("{}@{}.com", long_local, long_domain);
    let result = Email::try_from(long_email);
    assert!(result.is_ok());

    // Test special characters
    let special_email = Email::try_from("test.email+tag@sub-domain.example.com".to_string());
    assert!(special_email.is_ok());
}
