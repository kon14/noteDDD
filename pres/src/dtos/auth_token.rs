use serde::Serialize;
use utoipa::ToSchema;

use app::types::auth_token::AuthTokenPair;

#[derive(Debug, Serialize, ToSchema)]
#[schema(title = "AuthTokenPair")]
pub struct AuthTokenPairDto {
    pub access_token: String,
    pub refresh_token: String,
}

impl From<AuthTokenPair> for AuthTokenPairDto {
    fn from(token_pair: AuthTokenPair) -> Self {
        Self {
            access_token: token_pair.access_token.jwt,
            refresh_token: token_pair.refresh_token.jwt,
        }
    }
}
