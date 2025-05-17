use crate::types::auth_token::JsonWebTokenData;
use common::error::AppError;

pub trait TokenAdapter: Send + Sync {
    fn encode(&self, token: JsonWebTokenData) -> Result<String, AppError>;
    fn decode(&self, token: &str) -> Result<JsonWebTokenData, AppError>;
}
