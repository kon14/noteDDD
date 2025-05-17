use std::sync::Arc;

use crate::{
    auth::{authenticator::Authenticator, context::AuthRefreshContext},
    types::auth_token::{AuthTokenPair, UniqueAccessTokenIdentifier},
};
use common::error::AppError;

#[derive(Clone)]
pub struct AuthRefreshUseCase {
    authenticator: Arc<dyn Authenticator + Send + Sync>,
}

impl AuthRefreshUseCase {
    pub fn new(authenticator: Arc<dyn Authenticator + Send + Sync>) -> Self {
        Self { authenticator }
    }

    pub async fn execute(
        &self,
        auth_ref_ctx: AuthRefreshContext,
    ) -> Result<AuthTokenPair, AppError> {
        // Refresh Authentication Tokens
        let revoke_token_pair_id =
            UniqueAccessTokenIdentifier::Id(auth_ref_ctx.refresh_token.access_token_id);
        let token_pair = self
            .authenticator
            .generate_auth_tokens(auth_ref_ctx.user.id(), Some(revoke_token_pair_id))
            .await?;
        Ok(token_pair)
    }
}
