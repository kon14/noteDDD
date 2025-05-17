use bcrypt::{hash, verify, DEFAULT_COST};

use app::auth::pass_hasher::PasswordHasher;
use common::error::AppError;

pub struct BcryptPasswordHasher;

impl PasswordHasher for BcryptPasswordHasher {
    fn hash(&self, password: &str) -> Result<String, AppError> {
        hash(password, DEFAULT_COST).map_err(|err| {
            AppError::internal_with_private("Failed to hash password!", err.to_string())
        })
    }

    fn verify(&self, password: &str, password_hash: &str) -> Result<bool, AppError> {
        verify(password, password_hash).map_err(|err| {
            AppError::internal_with_private("Failed to verify password!", err.to_string())
        })
    }
}
