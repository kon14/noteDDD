use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    auth::context::{AuthAccessContext, AuthRefreshContext},
    types::auth_token::{AuthTokenPair, UniqueAccessTokenIdentifier},
};
use common::error::AppError;

#[async_trait]
pub trait Authenticator: Send + Sync {
    async fn generate_auth_tokens(
        &self,
        user_id: Uuid,
        revoke_token_pair_id: Option<UniqueAccessTokenIdentifier>,
    ) -> Result<AuthTokenPair, AppError>;

    async fn authenticate_access_token(&self, token: String)
        -> Result<AuthAccessContext, AppError>;

    async fn authenticate_refresh_token(
        &self,
        token: String,
    ) -> Result<AuthRefreshContext, AppError>;
}
