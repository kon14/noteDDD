use std::sync::Arc;

use crate::auth::pass_service::PasswordService;
use common::error::AppError;
use dmn::{
    entities::user::{CreateUserData, UniqueUserIdentifier, User},
    repos::user::UserRepository,
};

#[derive(Clone)]
pub struct RegisterUserUseCase {
    user_repo: Arc<dyn UserRepository + Send + Sync>,
    pass_service: Arc<dyn PasswordService + Send + Sync>,
}

impl RegisterUserUseCase {
    pub fn new(
        user_repo: Arc<dyn UserRepository + Send + Sync>,
        pass_service: Arc<dyn PasswordService + Send + Sync>,
    ) -> Self {
        Self {
            user_repo,
            pass_service,
        }
    }

    pub async fn execute(&self, input: RegisterUserInput) -> Result<User, AppError> {
        // Validate Password
        self.pass_service
            .validate_password_strength(&input.password)?;

        let password_hash = self.pass_service.hash(&input.password)?;
        let user_data = input.try_into_dmn(password_hash)?;

        let user_id = UniqueUserIdentifier::Email(user_data.email.clone());
        match self.user_repo.get_user(None, &user_id).await {
            Err(AppError::NotFound(_)) => Ok(()),
            Ok(_) => Err(AppError::conflict(format!(
                "Email ({}) already taken!",
                user_id
            ))),
            Err(err) => Err(err.reword("Couldn't verify email availability!".to_string())),
        }?;

        let user = self.user_repo.create_user(None, user_data).await?;
        Ok(user)
    }
}

#[derive(Debug)]
pub struct RegisterUserInput {
    pub email: String,
    pub password: String,
}

impl RegisterUserInput {
    pub fn try_into_dmn(self, password_hash: String) -> Result<CreateUserData, AppError> {
        let email = self
            .email
            .try_into()
            .map_err(|err: AppError| AppError::bad_request(err.public_info.clone()))?;
        let data = CreateUserData {
            email,
            password_hash,
        };
        let valid_data = data.validate()?;
        Ok(valid_data)
    }
}
