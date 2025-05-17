use chrono::{DateTime, Duration, Utc};
use std::fmt;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct JsonWebTokenData {
    pub id: Uuid,
    pub user_id: Uuid,
    pub expires_at: DateTime<Utc>,
    pub variant: JsonWebTokenDataVariant,
}

#[derive(Debug, Clone, PartialEq)]
pub enum JsonWebTokenDataVariant {
    AccessToken,
    RefreshToken,
}

#[derive(Debug, Clone)]
pub enum UniqueAccessTokenIdentifier {
    Id(Uuid),
    Jwt(String),
}

#[derive(Debug, Clone)]
pub enum UniqueRefreshTokenIdentifier {
    Id(Uuid),
    Jwt(String),
    AccessTokenId(Uuid),
}

#[derive(Debug, Clone)]
pub struct AccessToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub jwt: String,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct RefreshToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub access_token_id: Uuid,
    pub jwt: String,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct AuthTokenPair {
    pub access_token: AccessToken,
    pub refresh_token: RefreshToken,
}

impl fmt::Display for UniqueAccessTokenIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UniqueAccessTokenIdentifier::Id(id) => write!(f, "{}", id),
            UniqueAccessTokenIdentifier::Jwt(jwt) => write!(f, "{}", jwt),
        }
    }
}

impl fmt::Display for UniqueRefreshTokenIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UniqueRefreshTokenIdentifier::Id(id) => write!(f, "{}", id),
            UniqueRefreshTokenIdentifier::Jwt(jwt) => write!(f, "{}", jwt),
            UniqueRefreshTokenIdentifier::AccessTokenId(access_token_id) => {
                write!(f, "{}", access_token_id)
            }
        }
    }
}

impl JsonWebTokenData {
    pub fn new_access(user_id: Uuid, duration: Duration) -> Self {
        let expiry = Utc::now() + duration;
        JsonWebTokenData {
            id: Uuid::new_v4(),
            user_id,
            expires_at: expiry,
            variant: JsonWebTokenDataVariant::AccessToken,
        }
    }

    pub fn new_refresh(user_id: Uuid, duration: Duration) -> Self {
        let expiry = Utc::now() + duration;
        JsonWebTokenData {
            id: Uuid::new_v4(),
            user_id,
            expires_at: expiry,
            variant: JsonWebTokenDataVariant::RefreshToken,
        }
    }
}
