use async_trait::async_trait;
use std::sync::Arc;

use app::auth::{pass_hasher::PasswordHasher, pass_service::PasswordService};
use common::error::AppError;
use dmn::{entities::user::UniqueUserIdentifier, repos::user::UserRepository};

pub struct DefaultPasswordService {
    user_repo: Arc<dyn UserRepository + Send + Sync>,
    pass_hasher: Arc<dyn PasswordHasher + Send + Sync>,
}

impl DefaultPasswordService {
    pub fn new(
        user_repo: Arc<dyn UserRepository + Send + Sync>,
        pass_hasher: Arc<dyn PasswordHasher + Send + Sync>,
    ) -> Self {
        Self {
            user_repo,
            pass_hasher,
        }
    }
}

#[async_trait]
impl PasswordService for DefaultPasswordService {
    fn hash(&self, password: &str) -> Result<String, AppError> {
        let password_hash = self.pass_hasher.hash(password)?;
        Ok(password_hash)
    }

    async fn verify_password(
        &self,
        user_id: &UniqueUserIdentifier,
        password: &str,
    ) -> Result<(), AppError> {
        const UNAUTHORIZED_ERR_STR: &str = "Failed to verify user password!";

        let current_password_hash = self
            .user_repo
            .get_user_password_hash(None, user_id)
            .await
            .map_err(|err| {
                AppError::unauthorized_with_private(
                    UNAUTHORIZED_ERR_STR,
                    format!("Password hash fetch failed for user ({user_id})! Reason: {err}"),
                )
            })?;

        let password_matches = self
            .pass_hasher
            .verify(password, &current_password_hash)
            .map_err(|err| {
                AppError::unauthorized_with_private(
                    UNAUTHORIZED_ERR_STR,
                    format!("Password verification failed for user ({user_id}): {err}"),
                )
            })?;

        if !password_matches {
            return Err(AppError::unauthorized_with_private(
                UNAUTHORIZED_ERR_STR,
                format!("Password mismatch for user ({})!", user_id),
            ));
        }

        Ok(())
    }

    #[allow(unused_variables, unreachable_code)]
    fn validate_password_strength(&self, password: &str) -> Result<(), AppError> {
        return Ok(());
        todo!()
    }
}
