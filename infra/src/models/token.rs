use app::types::auth_token::{AccessToken, AuthTokenPair, RefreshToken};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug)]
pub(crate) struct AccessTokenPg {
    pub(crate) id: Uuid,
    pub(crate) user_id: Uuid,
    pub(crate) jwt: String,
    pub(crate) expires_at: DateTime<Utc>,
}

impl From<AccessTokenPg> for AccessToken {
    fn from(pg_access_token: AccessTokenPg) -> Self {
        Self {
            id: pg_access_token.id,
            user_id: pg_access_token.user_id,
            jwt: pg_access_token.jwt,
            expires_at: pg_access_token.expires_at,
        }
    }
}

#[derive(Debug)]
pub(crate) struct RefreshTokenPg {
    pub(crate) id: Uuid,
    pub(crate) user_id: Uuid,
    pub(crate) access_token_id: Uuid,
    pub(crate) jwt: String,
    pub(crate) expires_at: DateTime<Utc>,
}

impl From<RefreshTokenPg> for RefreshToken {
    fn from(pg_refresh_token: RefreshTokenPg) -> Self {
        Self {
            id: pg_refresh_token.id,
            user_id: pg_refresh_token.user_id,
            access_token_id: pg_refresh_token.access_token_id,
            jwt: pg_refresh_token.jwt,
            expires_at: pg_refresh_token.expires_at,
        }
    }
}

#[derive(Debug)]
pub struct AuthTokenPairPg {
    pub(crate) access_token: AccessTokenPg,
    pub(crate) refresh_token: RefreshTokenPg,
}

impl From<AuthTokenPairPg> for AuthTokenPair {
    fn from(pg_token_pair: AuthTokenPairPg) -> Self {
        Self {
            access_token: pg_token_pair.access_token.into(),
            refresh_token: pg_token_pair.refresh_token.into(),
        }
    }
}
