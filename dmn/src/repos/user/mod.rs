use async_trait::async_trait;

use crate::entities::user::{CreateUserData, UniqueUserIdentifier, UpdateUserData, User};
use common::{error::AppError, params::PaginationParams, tx::ctx::TransactionContext};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn get_user(
        &self,
        ctx: Option<&mut dyn TransactionContext>,
        user_id: &UniqueUserIdentifier,
    ) -> Result<User, AppError>;

    async fn get_users(
        &self,
        ctx: Option<&mut dyn TransactionContext>,
        pagination: &PaginationParams,
    ) -> Result<GetUsersResponse, AppError>;

    async fn create_user(
        &self,
        ctx: Option<&mut dyn TransactionContext>,
        user_data: CreateUserData,
    ) -> Result<User, AppError>;

    async fn update_user(
        &self,
        ctx: Option<&mut dyn TransactionContext>,
        user_id: &UniqueUserIdentifier,
        user_data: UpdateUserData,
    ) -> Result<User, AppError>;

    async fn delete_user(
        &self,
        ctx: Option<&mut dyn TransactionContext>,
        user_id: &UniqueUserIdentifier,
    ) -> Result<(), AppError>;

    async fn get_user_password_hash(
        &self,
        ctx: Option<&mut dyn TransactionContext>,
        user_id: &UniqueUserIdentifier,
    ) -> Result<String, AppError>;
}

#[derive(Debug)]
pub struct GetUsersResponse {
    pub users: Vec<User>,
    pub count: u32,
}
