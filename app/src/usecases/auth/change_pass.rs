use std::sync::Arc;

use crate::auth::{context::AuthAccessContext, pass_service::PasswordService};
use common::error::AppError;
use dmn::{
    entities::user::{UniqueUserIdentifier, UpdateUserData},
    repos::user::UserRepository,
};

#[derive(Clone)]
pub struct ChangeUserPasswordUseCase {
    user_repo: Arc<dyn UserRepository + Send + Sync>,
    pass_service: Arc<dyn PasswordService + Send + Sync>,
}

impl ChangeUserPasswordUseCase {
    pub fn new(
        user_repo: Arc<dyn UserRepository + Send + Sync>,
        pass_service: Arc<dyn PasswordService + Send + Sync>,
    ) -> Self {
        Self {
            user_repo,
            pass_service,
        }
    }

    pub async fn execute(
        &self,
        auth_ctx: AuthAccessContext,
        input: ChangeUserPasswordInput,
    ) -> Result<(), AppError> {
        // Validate New Password
        self.pass_service
            .validate_password_strength(&input.new_password)?;

        // Target User = Authenticated User
        let user_id = UniqueUserIdentifier::Id(auth_ctx.user.id());

        // Verify Password
        self.pass_service
            .verify_password(&user_id, &input.current_password)
            .await?;

        let new_password_hash = self.pass_service.hash(&input.new_password)?;
        let user_data = input.into_dmn(new_password_hash);
        self.user_repo
            .update_user(None, &user_id, user_data)
            .await?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct ChangeUserPasswordInput {
    pub current_password: String,
    pub new_password: String,
}

impl ChangeUserPasswordInput {
    pub fn into_dmn(self, password_hash: String) -> UpdateUserData {
        UpdateUserData {
            email: None,
            password_hash: Some(password_hash),
        }
    }
}
