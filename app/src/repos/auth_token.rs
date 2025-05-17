use async_trait::async_trait;
use uuid::Uuid;

use crate::types::auth_token::{
    AccessToken, AuthTokenPair, RefreshToken, UniqueAccessTokenIdentifier,
    UniqueRefreshTokenIdentifier,
};
use common::{error::AppError, tx::ctx::TransactionContext};

#[async_trait]
pub trait AuthTokenRepository: Send + Sync {
    async fn get_access_token(
        &self,
        ctx: Option<&mut dyn TransactionContext>,
        token_id: &UniqueAccessTokenIdentifier,
    ) -> Result<AccessToken, AppError>;

    async fn get_refresh_token(
        &self,
        ctx: Option<&mut dyn TransactionContext>,
        token_id: &UniqueRefreshTokenIdentifier,
    ) -> Result<RefreshToken, AppError>;

    async fn get_user_tokens(
        &self,
        ctx: Option<&mut dyn TransactionContext>,
        user_id: Uuid,
    ) -> Result<Vec<AuthTokenPair>, AppError>;

    async fn create_access_token(
        &self,
        ctx: Option<&mut dyn TransactionContext>,
        access_token: AccessToken,
    ) -> Result<AccessToken, AppError>;

    async fn create_refresh_token(
        &self,
        ctx: Option<&mut dyn TransactionContext>,
        refresh_token: RefreshToken,
    ) -> Result<RefreshToken, AppError>;

    async fn delete_token_pair(
        &self,
        ctx: Option<&mut dyn TransactionContext>,
        token_id: &UniqueAccessTokenIdentifier,
    ) -> Result<(), AppError>;
}
