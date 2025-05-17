use async_trait::async_trait;
use sqlx::PgPool;
use std::sync::Arc;

use crate::{db::user as db, tx::ctx::PgTransactionContextExt};
use common::{error::AppError, params::PaginationParams, tx::ctx::TransactionContext};
use dmn::{
    entities::user::{CreateUserData, UniqueUserIdentifier, UpdateUserData, User, UserData},
    repos::user::{GetUsersResponse, UserRepository},
};

pub struct PgUserRepository {
    db_pool: Arc<PgPool>,
}

impl PgUserRepository {
    pub fn new(db_pool: Arc<PgPool>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl UserRepository for PgUserRepository {
    async fn get_user(
        &self,
        ctx: Option<&mut dyn TransactionContext>,
        user_id: &UniqueUserIdentifier,
    ) -> Result<User, AppError> {
        let db_user = match ctx {
            Some(ctx) => {
                let pg_tx = ctx.as_postgres_tx().ok_or_else(|| {
                    AppError::internal("Invalid transaction context for Postgres repository")
                })?;
                db::get_user(&mut **pg_tx, user_id).await?
            }
            None => db::get_user(&*self.db_pool, user_id).await?,
        };
        let user_data: UserData = db_user.try_into()?;
        let user = user_data.try_into()?;
        Ok(user)
    }

    async fn get_users(
        &self,
        ctx: Option<&mut dyn TransactionContext>,
        pagination: &PaginationParams,
    ) -> Result<GetUsersResponse, AppError> {
        let (db_users, count) = match ctx {
            Some(ctx) => {
                let pg_tx = ctx.as_postgres_tx().ok_or_else(|| {
                    AppError::internal("Invalid transaction context for Postgres repository")
                })?;
                let db_users = db::get_users(&mut **pg_tx, pagination).await?;
                let count = db::get_user_count(&mut **pg_tx).await?;
                (db_users, count)
            }
            None => {
                let db_users = db::get_users(&*self.db_pool, pagination).await?;
                let count = db::get_user_count(&*self.db_pool).await?;
                (db_users, count)
            }
        };
        let users = db_users
            .into_iter()
            .map(|db_user| {
                let user_data: UserData = db_user.try_into()?;
                user_data.try_into()
            })
            .collect::<Result<_, AppError>>()?;
        let dmn_res = GetUsersResponse { users, count };
        Ok(dmn_res)
    }

    async fn create_user(
        &self,
        ctx: Option<&mut dyn TransactionContext>,
        user_data: CreateUserData,
    ) -> Result<User, AppError> {
        let db_user = match ctx {
            Some(ctx) => {
                let pg_tx = ctx.as_postgres_tx().ok_or_else(|| {
                    AppError::internal("Invalid transaction context for Postgres repository")
                })?;
                db::create_user(&mut **pg_tx, user_data.into()).await?
            }
            None => db::create_user(&*self.db_pool, user_data.into()).await?,
        };
        let user_data: UserData = db_user.try_into()?;
        let user = user_data.try_into()?;
        Ok(user)
    }

    async fn update_user(
        &self,
        ctx: Option<&mut dyn TransactionContext>,
        user_id: &UniqueUserIdentifier,
        user_data: UpdateUserData,
    ) -> Result<User, AppError> {
        let db_user = match ctx {
            Some(ctx) => {
                let pg_tx = ctx.as_postgres_tx().ok_or_else(|| {
                    AppError::internal("Invalid transaction context for Postgres repository")
                })?;
                db::update_user(&mut **pg_tx, user_id, user_data.into()).await?
            }
            None => db::update_user(&*self.db_pool, user_id, user_data.into()).await?,
        };
        let user_data: UserData = db_user.try_into()?;
        let user = user_data.try_into()?;
        Ok(user)
    }

    async fn delete_user(
        &self,
        ctx: Option<&mut dyn TransactionContext>,
        user_id: &UniqueUserIdentifier,
    ) -> Result<(), AppError> {
        match ctx {
            Some(ctx) => {
                let pg_tx = ctx.as_postgres_tx().ok_or_else(|| {
                    AppError::internal("Invalid transaction context for Postgres repository")
                })?;
                db::delete_user(&mut **pg_tx, user_id).await?
            }
            None => db::delete_user(&*self.db_pool, user_id).await?,
        };
        Ok(())
    }

    async fn get_user_password_hash(
        &self,
        ctx: Option<&mut dyn TransactionContext>,
        user_id: &UniqueUserIdentifier,
    ) -> Result<String, AppError> {
        let password_hash = match ctx {
            Some(ctx) => {
                let pg_tx = ctx.as_postgres_tx().ok_or_else(|| {
                    AppError::internal("Invalid transaction context for Postgres repository")
                })?;
                db::get_user_password_hash(&mut **pg_tx, user_id).await?
            }
            None => db::get_user_password_hash(&*self.db_pool, user_id).await?,
        };
        Ok(password_hash)
    }
}

impl From<CreateUserData> for db::CreateUserDataPg {
    fn from(dmn_user_data: CreateUserData) -> Self {
        Self {
            email: dmn_user_data.email.to_string(),
            password_hash: dmn_user_data.password_hash,
        }
    }
}

impl From<UpdateUserData> for db::UpdateUserDataPg {
    fn from(dmn_user_data: UpdateUserData) -> Self {
        Self {
            email: dmn_user_data.email.map(|email| email.to_string()),
            password_hash: dmn_user_data.password_hash,
        }
    }
}
