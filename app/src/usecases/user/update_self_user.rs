use std::sync::Arc;

use crate::auth::context::AuthAccessContext;
use common::error::AppError;
use dmn::{
    entities::user::{UniqueUserIdentifier, UpdateUserData, User},
    repos::user::UserRepository,
};

#[derive(Clone)]
pub struct UpdateSelfUserUseCase {
    user_repo: Arc<dyn UserRepository + Send + Sync>,
}

impl UpdateSelfUserUseCase {
    pub fn new(user_repo: Arc<dyn UserRepository + Send + Sync>) -> Self {
        Self { user_repo }
    }

    pub async fn execute(
        &self,
        auth_ctx: AuthAccessContext,
        input: UpdateSelfUserInput,
    ) -> Result<User, AppError> {
        // Target User = Authenticated User
        let user_id = UniqueUserIdentifier::Id(auth_ctx.user.id());
        let user_data = input.try_into_dmn()?;

        if let Some(email) = user_data.email.clone() {
            let user_id = UniqueUserIdentifier::Email(email);
            match self.user_repo.get_user(None, &user_id).await {
                Err(AppError::NotFound(_)) => Ok(()),
                Err(err) => Err(err.reword("Couldn't verify email availability!".to_string())),
                Ok(user) => {
                    if auth_ctx.user.id() == user.id() {
                        Ok(())
                    } else {
                        Err(AppError::conflict(format!(
                            "Email ({}) already taken!",
                            user_id
                        )))
                    }
                }
            }?;
        }

        let user = self
            .user_repo
            .update_user(None, &user_id, user_data)
            .await?;
        Ok(user)
    }
}

#[derive(Debug)]
pub struct UpdateSelfUserInput {
    pub email: String,
}

impl UpdateSelfUserInput {
    pub fn try_into_dmn(self) -> Result<UpdateUserData, AppError> {
        let email = self
            .email
            .try_into()
            .map_err(|err: AppError| AppError::bad_request(err.public_info.clone()))?;
        let data = UpdateUserData {
            email: Some(email),
            password_hash: None,
        };
        let valid_data = data.validate()?;
        Ok(valid_data)
    }
}
