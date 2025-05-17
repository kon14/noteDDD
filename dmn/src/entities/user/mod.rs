mod validation;

use chrono::{DateTime, Utc};
use common::error::AppError;
use uuid::Uuid;

use crate::value_objects::email::Email;

#[derive(Debug, Clone)]
pub struct User {
    id: Uuid,
    email: Email,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl TryFrom<UserData> for User {
    type Error = AppError;

    fn try_from(data: UserData) -> Result<Self, Self::Error> {
        // Add any necessary validation here if needed in the future...
        Ok(Self {
            id: data.id,
            email: data.email,
            created_at: data.created_at,
            updated_at: data.updated_at,
        })
    }
}

impl User {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn email(&self) -> &Email {
        &self.email
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

impl User {
    pub fn set_email(&mut self, new_email: Email) -> Result<(), AppError> {
        self.email = new_email;
        self.updated_at = Utc::now();
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum UniqueUserIdentifier {
    Id(Uuid),
    Email(Email),
}

impl std::fmt::Display for UniqueUserIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UniqueUserIdentifier::Id(id) => id.fmt(f),
            UniqueUserIdentifier::Email(e) => e.fmt(f),
        }
    }
}

/// [`User`] entity pre-validation data struct.
pub struct UserData {
    pub id: Uuid,
    pub email: Email,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// [`User`] entity creation pre-validation utility struct.
#[derive(Debug)]
pub struct CreateUserData {
    pub email: Email,
    pub password_hash: String,
}

/// [`User`] entity update pre-validation utility struct.
#[derive(Debug)]
pub struct UpdateUserData {
    pub email: Option<Email>,
    pub password_hash: Option<String>,
}

#[cfg(test)]
mod tests;
