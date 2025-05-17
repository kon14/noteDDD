use std::sync::Arc;

use crate::{
    auth::{authenticator::Authenticator, pass_service::PasswordService},
    types::auth_token::AuthTokenPair,
};
use common::error::AppError;
use dmn::{entities::user::UniqueUserIdentifier, repos::user::UserRepository};

#[derive(Clone)]
pub struct AuthLoginUseCase {
    authenticator: Arc<dyn Authenticator + Send + Sync>,
    user_repo: Arc<dyn UserRepository>,
    pass_service: Arc<dyn PasswordService + Send + Sync>,
}

impl AuthLoginUseCase {
    pub fn new(
        authenticator: Arc<dyn Authenticator + Send + Sync>,
        user_repo: Arc<dyn UserRepository + Send + Sync>,
        pass_service: Arc<dyn PasswordService + Send + Sync>,
    ) -> Self {
        Self {
            authenticator,
            user_repo,
            pass_service,
        }
    }

    pub async fn execute(&self, input: AuthLoginInput) -> Result<AuthTokenPair, AppError> {
        const UNAUTHORIZED_ERR_STR: &str =
            "Login failed. Please check your credentials and try again.";

        // Obfuscate sensitive login failure information
        let token_pair = self.attempt_login(input).await.map_err(|err| {
            AppError::unauthorized_with_private(UNAUTHORIZED_ERR_STR, err.to_string())
        })?;
        Ok(token_pair)
    }

    async fn attempt_login(&self, input: AuthLoginInput) -> Result<AuthTokenPair, AppError> {
        let user_email = input.email.try_into()?;
        let user_id = UniqueUserIdentifier::Email(user_email);

        // Fetch User
        let user = self.user_repo.get_user(None, &user_id).await?;

        // Verify Password
        self.pass_service
            .verify_password(&user_id, &input.password)
            .await?;

        // Generate Authentication Tokens
        let token_pair = self
            .authenticator
            .generate_auth_tokens(user.id(), None)
            .await?;

        Ok(token_pair)
    }
}

#[derive(Debug)]
pub struct AuthLoginInput {
    pub email: String,
    pub password: String,
}
