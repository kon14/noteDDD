use std::sync::Arc;

use crate::auth::{context::AuthAccessContext, pass_service::PasswordService};
use common::error::AppError;
use dmn::{entities::user::UniqueUserIdentifier, repos::user::UserRepository};

#[derive(Clone)]
pub struct DeleteSelfUserUseCase {
    user_repo: Arc<dyn UserRepository + Send + Sync>,
    pass_service: Arc<dyn PasswordService + Send + Sync>,
}

impl DeleteSelfUserUseCase {
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
        input: DeleteSelfUserInput,
    ) -> Result<(), AppError> {
        // Target User = Authenticated User
        let user_id = UniqueUserIdentifier::Id(auth_ctx.user.id());

        // Verify Password
        self.pass_service
            .verify_password(&user_id, &input.password)
            .await?;

        self.user_repo.delete_user(None, &user_id).await?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct DeleteSelfUserInput {
    pub password: String,
}
