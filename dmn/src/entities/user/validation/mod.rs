use super::{CreateUserData, UpdateUserData};
use common::error::AppError;

impl CreateUserData {
    pub fn validate(self) -> Result<Self, AppError> {
        // Add any necessary validation here if needed in the future...
        Ok(self)
    }
}
impl UpdateUserData {
    pub fn validate(self) -> Result<Self, AppError> {
        // Add any necessary validation here if needed in the future...
        Ok(self)
    }
}
