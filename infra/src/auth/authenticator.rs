use async_trait::async_trait;
use chrono::{Duration, Timelike, Utc};
use std::sync::Arc;
use uuid::Uuid;

use app::{
    auth::{
        authenticator::Authenticator,
        context::{AuthAccessContext, AuthRefreshContext},
        token_adapter::TokenAdapter,
    },
    repos::auth_token::AuthTokenRepository,
    types::auth_token::{
        AccessToken, AuthTokenPair, JsonWebTokenData, JsonWebTokenDataVariant, RefreshToken,
        UniqueAccessTokenIdentifier, UniqueRefreshTokenIdentifier,
    },
};
use common::{
    error::AppError,
    tx::{TransactionResult, UnitOfWork},
};
use dmn::{
    entities::user::{UniqueUserIdentifier, User},
    repos::user::UserRepository,
};

pub struct JwtAuthenticator {
    access_token_duration: Duration,
    refresh_token_duration: Duration,
    token_adapter: Arc<dyn TokenAdapter + Send + Sync>,
    unit_of_work: Arc<dyn UnitOfWork>,
    token_repo: Arc<dyn AuthTokenRepository + Send + Sync>,
    user_repo: Arc<dyn UserRepository + Send + Sync>,
}

impl JwtAuthenticator {
    pub fn new(
        access_token_duration_secs: u32,
        refresh_token_duration_secs: u32,
        token_adapter: Arc<dyn TokenAdapter + Send + Sync>,
        unit_of_work: Arc<dyn UnitOfWork>,
        token_repo: Arc<dyn AuthTokenRepository + Send + Sync>,
        user_repo: Arc<dyn UserRepository + Send + Sync>,
    ) -> Self {
        Self {
            access_token_duration: Duration::seconds(access_token_duration_secs as i64),
            refresh_token_duration: Duration::seconds(refresh_token_duration_secs as i64),
            token_adapter,
            unit_of_work,
            token_repo,
            user_repo,
        }
    }

    async fn authenticate_token(
        &self,
        token: String,
        variant: JsonWebTokenDataVariant,
    ) -> Result<(User, Option<AccessToken>, Option<RefreshToken>), AppError> {
        const UNAUTHORIZED_ERR_STR: &str = "Failed to authenticate user!";

        // Verify JWT data.
        let token_data = self.token_adapter.decode(&token).map_err(|err| {
            AppError::unauthorized_with_private(UNAUTHORIZED_ERR_STR, err.to_string())
        })?;

        if token_data.variant != variant {
            return Err(AppError::unauthorized_with_private(
                UNAUTHORIZED_ERR_STR,
                "Auth token variant mismatch!",
            ));
        }

        // Retrieve non-revoked token from persistent storage.
        // Fetch implicitly guarantees user existence (db record cascade).
        let (db_token_user_id, db_token_expires_at, db_access_token, db_refresh_token) =
            match token_data.variant {
                JsonWebTokenDataVariant::AccessToken => {
                    let token_id = UniqueAccessTokenIdentifier::Jwt(token);
                    let token = self
                        .token_repo
                        .get_access_token(None, &token_id)
                        .await
                        .map_err(|err| err.reword(UNAUTHORIZED_ERR_STR.to_string()))?;
                    (token.user_id, token.expires_at, Some(token), None)
                }
                JsonWebTokenDataVariant::RefreshToken => {
                    let token_id = UniqueRefreshTokenIdentifier::Jwt(token);
                    let token = self
                        .token_repo
                        .get_refresh_token(None, &token_id)
                        .await
                        .map_err(|err| err.reword(UNAUTHORIZED_ERR_STR.to_string()))?;
                    (token.user_id, token.expires_at, None, Some(token))
                }
            };

        // Perform additional validations.
        if db_token_user_id != token_data.user_id {
            return Err(AppError::unauthorized_with_private(
                UNAUTHORIZED_ERR_STR,
                "Token user id mismatch!",
            ));
        }
        let expires_at = db_token_expires_at.with_nanosecond(0).unwrap();
        if expires_at != token_data.expires_at {
            return Err(AppError::unauthorized_with_private(
                UNAUTHORIZED_ERR_STR,
                "Token expiry mismatch!",
            ));
        }
        // Technically already checked during decoding...
        if expires_at < Utc::now() {
            return Err(AppError::unauthorized_with_private(
                UNAUTHORIZED_ERR_STR,
                "Token has expired!",
            ));
        }

        // Fetch User data.
        let user_id = UniqueUserIdentifier::Id(token_data.user_id);
        let user = self
            .user_repo
            .get_user(None, &user_id)
            .await
            .map_err(|err| {
                AppError::unauthorized_with_private(UNAUTHORIZED_ERR_STR, err.to_string())
            })?;

        Ok((user, db_access_token, db_refresh_token))
    }
}

#[async_trait]
impl Authenticator for JwtAuthenticator {
    async fn generate_auth_tokens(
        &self,
        user_id: Uuid,
        revoke_token_pair_id: Option<UniqueAccessTokenIdentifier>,
    ) -> Result<AuthTokenPair, AppError> {
        const INTERNAL_ERR_STR: &str = "Failed to generate auth tokens!";

        // Generate JWTs
        let access_token = JsonWebTokenData::new_access(user_id, self.access_token_duration);
        let refresh_token = JsonWebTokenData::new_refresh(user_id, self.refresh_token_duration);
        let access_token_jwt = self
            .token_adapter
            .encode(access_token.clone())
            .map_err(|err| AppError::internal_with_private(INTERNAL_ERR_STR, err.to_string()))?;
        let refresh_token_jwt = self
            .token_adapter
            .encode(refresh_token.clone())
            .map_err(|err| AppError::internal_with_private(INTERNAL_ERR_STR, err.to_string()))?;
        let access_token = AccessToken {
            id: access_token.id,
            user_id: access_token.user_id,
            jwt: access_token_jwt,
            expires_at: access_token.expires_at,
        };
        let refresh_token = RefreshToken {
            id: refresh_token.id,
            user_id: refresh_token.user_id,
            access_token_id: access_token.id,
            jwt: refresh_token_jwt,
            expires_at: refresh_token.expires_at,
        };

        // Persist Tokens
        let token_repo = self.token_repo.clone();
        let result = self
            .unit_of_work
            .run_in_transaction(Box::new(|ctx| {
                Box::pin(async move {
                    let access_token = token_repo
                        .create_access_token(Some(ctx), access_token)
                        .await
                        .map_err(|err| {
                            AppError::internal_with_private(INTERNAL_ERR_STR, err.to_string())
                        })?;
                    let refresh_token = token_repo
                        .create_refresh_token(Some(ctx), refresh_token)
                        .await
                        .map_err(|err| {
                            AppError::internal_with_private(INTERNAL_ERR_STR, err.to_string())
                        })?;
                    if let Some(revoke_access_token_id) = revoke_token_pair_id {
                        token_repo
                            .delete_token_pair(Some(ctx), &revoke_access_token_id)
                            .await
                            .map_err(|err| {
                                AppError::internal_with_private(INTERNAL_ERR_STR, err.to_string())
                            })?;
                    }
                    let token_pair = AuthTokenPair {
                        access_token,
                        refresh_token,
                    };
                    Ok(TransactionResult::new(token_pair))
                })
            }))
            .await?;

        let token_pair = result.extract::<AuthTokenPair>()?;
        Ok(token_pair)
    }

    async fn authenticate_access_token(
        &self,
        token: String,
    ) -> Result<AuthAccessContext, AppError> {
        let (user, access_token, _) = self
            .authenticate_token(token, JsonWebTokenDataVariant::AccessToken)
            .await?;
        let auth_ctx = AuthAccessContext {
            user,
            access_token: access_token.unwrap(),
        };
        Ok(auth_ctx)
    }

    async fn authenticate_refresh_token(
        &self,
        token: String,
    ) -> Result<AuthRefreshContext, AppError> {
        let (user, _, refresh_token) = self
            .authenticate_token(token, JsonWebTokenDataVariant::RefreshToken)
            .await?;
        let auth_ctx = AuthRefreshContext {
            user,
            refresh_token: refresh_token.unwrap(),
        };
        Ok(auth_ctx)
    }
}
