use async_trait::async_trait;

use common::error::AppError;
use dmn::entities::user::UniqueUserIdentifier;

#[async_trait]
pub trait PasswordService: Send + Sync {
    fn hash(&self, password: &str) -> Result<String, AppError>;

    async fn verify_password(
        &self,
        user_id: &UniqueUserIdentifier,
        password: &str,
    ) -> Result<(), AppError>;

    fn validate_password_strength(&self, password: &str) -> Result<(), AppError>;
}
