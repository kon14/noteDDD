use common::error::AppError;

pub trait PasswordHasher: Send + Sync {
    fn hash(&self, password: &str) -> Result<String, AppError>;

    fn verify(&self, password: &str, password_hash: &str) -> Result<bool, AppError>;
}
