use regex::Regex;
use std::ops::Deref;

use common::error::AppError;

#[derive(Debug, Clone)]
pub struct Email(String);

impl TryFrom<String> for Email {
    type Error = AppError;

    fn try_from(email_str: String) -> Result<Self, Self::Error> {
        if is_valid_email(&email_str) {
            Ok(Self(email_str))
        } else {
            Err(AppError::internal(format!(
                "Invalid email address ({email_str})!"
            )))
        }
    }
}

impl Deref for Email {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

fn is_valid_email(email_str: &str) -> bool {
    const EMAIL_PATTERN: &str =
        r"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9-]+(\.[a-zA-Z0-9-]+)+$";
    let regex = Regex::new(EMAIL_PATTERN).unwrap();
    regex.is_match(email_str)
}

#[cfg(test)]
mod tests;
