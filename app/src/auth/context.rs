use crate::types::auth_token::{AccessToken, RefreshToken};
use dmn::entities::user::User;

#[derive(Debug, Clone)]
pub struct AuthAccessContext {
    pub user: User,
    pub access_token: AccessToken,
}

#[derive(Debug, Clone)]
pub struct AuthRefreshContext {
    pub user: User,
    pub refresh_token: RefreshToken,
}
