use async_trait::async_trait;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use crate::{db::auth_token as db, tx::ctx::PgTransactionContextExt};
use app::{
    repos::auth_token::AuthTokenRepository,
    types::auth_token::{
        AccessToken, AuthTokenPair, RefreshToken, UniqueAccessTokenIdentifier,
        UniqueRefreshTokenIdentifier,
    },
};
use common::{error::AppError, tx::ctx::TransactionContext};

pub struct PgAuthTokenRepository {
    db_pool: Arc<PgPool>,
}

impl PgAuthTokenRepository {
    pub fn new(db_pool: Arc<PgPool>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl AuthTokenRepository for PgAuthTokenRepository {
    async fn get_access_token(
        &self,
        ctx: Option<&mut dyn TransactionContext>,
        token_id: &UniqueAccessTokenIdentifier,
    ) -> Result<AccessToken, AppError> {
        let db_access_token = match ctx {
            Some(ctx) => {
                let pg_tx = ctx.as_postgres_tx().ok_or_else(|| {
                    AppError::internal("Invalid transaction context for Postgres repository")
                })?;
                db::get_access_token(&mut **pg_tx, token_id).await?
            }
            None => db::get_access_token(&*self.db_pool, token_id).await?,
        };
        let access_token = db_access_token.into();
        Ok(access_token)
    }

    async fn get_refresh_token(
        &self,
        ctx: Option<&mut dyn TransactionContext>,
        token_id: &UniqueRefreshTokenIdentifier,
    ) -> Result<RefreshToken, AppError> {
        let db_refresh_token = match ctx {
            Some(ctx) => {
                let pg_tx = ctx.as_postgres_tx().ok_or_else(|| {
                    AppError::internal("Invalid transaction context for Postgres repository")
                })?;
                db::get_refresh_token(&mut **pg_tx, token_id).await?
            }
            None => db::get_refresh_token(&*self.db_pool, token_id).await?,
        };
        let refresh_token = db_refresh_token.into();
        Ok(refresh_token)
    }

    async fn get_user_tokens(
        &self,
        ctx: Option<&mut dyn TransactionContext>,
        user_id: Uuid,
    ) -> Result<Vec<AuthTokenPair>, AppError> {
        let db_auth_token_pair = match ctx {
            Some(ctx) => {
                let pg_tx = ctx.as_postgres_tx().ok_or_else(|| {
                    AppError::internal("Invalid transaction context for Postgres repository")
                })?;
                db::get_user_tokens(&mut **pg_tx, user_id).await?
            }
            None => db::get_user_tokens(&*self.db_pool, user_id).await?,
        };
        let auth_token = db_auth_token_pair
            .into_iter()
            .map(|token_pair| token_pair.into())
            .collect();
        Ok(auth_token)
    }

    async fn create_access_token(
        &self,
        ctx: Option<&mut dyn TransactionContext>,
        access_token: AccessToken,
    ) -> Result<AccessToken, AppError> {
        let db_access_token = match ctx {
            Some(ctx) => {
                let pg_tx = ctx.as_postgres_tx().ok_or_else(|| {
                    AppError::internal("Invalid transaction context for Postgres repository")
                })?;
                db::create_access_token(&mut **pg_tx, access_token.into()).await?
            }
            None => db::create_access_token(&*self.db_pool, access_token.into()).await?,
        };
        let access_token = db_access_token.into();
        Ok(access_token)
    }

    async fn create_refresh_token(
        &self,
        ctx: Option<&mut dyn TransactionContext>,
        refresh_token: RefreshToken,
    ) -> Result<RefreshToken, AppError> {
        let db_refresh_token = match ctx {
            Some(ctx) => {
                let pg_tx = ctx.as_postgres_tx().ok_or_else(|| {
                    AppError::internal("Invalid transaction context for Postgres repository")
                })?;
                db::create_refresh_token(&mut **pg_tx, refresh_token.into()).await?
            }
            None => db::create_refresh_token(&*self.db_pool, refresh_token.into()).await?,
        };
        let refresh_token = db_refresh_token.into();
        Ok(refresh_token)
    }

    async fn delete_token_pair(
        &self,
        ctx: Option<&mut dyn TransactionContext>,
        token_id: &UniqueAccessTokenIdentifier,
    ) -> Result<(), AppError> {
        match ctx {
            Some(ctx) => {
                let pg_tx = ctx.as_postgres_tx().ok_or_else(|| {
                    AppError::internal("Invalid transaction context for Postgres repository")
                })?;
                db::delete_token_pair(&mut **pg_tx, token_id).await?
            }
            None => db::delete_token_pair(&*self.db_pool, token_id).await?,
        };
        Ok(())
    }
}

impl From<AccessToken> for db::CreateAccessTokenDataPg {
    fn from(dmn_access_token: AccessToken) -> Self {
        Self {
            id: dmn_access_token.id,
            user_id: dmn_access_token.user_id,
            jwt: dmn_access_token.jwt,
            expires_at: dmn_access_token.expires_at,
        }
    }
}

impl From<RefreshToken> for db::CreateRefreshTokenDataPg {
    fn from(dmn_refresh_token: RefreshToken) -> Self {
        Self {
            id: dmn_refresh_token.id,
            user_id: dmn_refresh_token.user_id,
            access_token_id: dmn_refresh_token.access_token_id,
            jwt: dmn_refresh_token.jwt,
            expires_at: dmn_refresh_token.expires_at,
        }
    }
}
