use chrono::{DateTime, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use app::{
    auth::token_adapter::TokenAdapter,
    types::auth_token::{JsonWebTokenData, JsonWebTokenDataVariant},
};
use common::error::AppError;

pub struct JwtTokenAdapter {
    jwt_secret: String,
}

impl JwtTokenAdapter {
    pub fn new(jwt_secret: String) -> Self {
        Self { jwt_secret }
    }
}

impl TokenAdapter for JwtTokenAdapter {
    fn encode(&self, token: JsonWebTokenData) -> Result<String, AppError> {
        let claims: JwtClaims = token.into();
        let encoded_token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_ref()),
        )
        .map_err(|err| {
            AppError::internal_with_private("Failed to encode token!", err.to_string())
        })?;
        Ok(encoded_token)
    }

    fn decode(&self, token: &str) -> Result<JsonWebTokenData, AppError> {
        let token_data = decode::<JwtClaims>(
            &token,
            &DecodingKey::from_secret(self.jwt_secret.as_ref()),
            &Validation::default(),
        )
        .map_err(|err| {
            AppError::internal_with_private("Failed to decode token!", err.to_string())
        })?;
        let decoded_token = token_data.claims.into();
        Ok(decoded_token)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct JwtClaims {
    #[serde(rename = "tokenId")]
    id: Uuid,
    #[serde(rename = "sub")]
    user_id: Uuid,
    #[serde(rename = "exp", with = "chrono::serde::ts_seconds")]
    expires_at: DateTime<Utc>,
    #[serde(rename = "type")]
    variant: JwtClaimsVariant,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) enum JwtClaimsVariant {
    #[serde(rename = "access")]
    AccessToken,
    #[serde(rename = "refresh")]
    RefreshToken,
}

impl From<JsonWebTokenData> for JwtClaims {
    fn from(data: JsonWebTokenData) -> Self {
        Self {
            id: data.id,
            user_id: data.user_id,
            expires_at: data.expires_at,
            variant: data.variant.into(),
        }
    }
}

impl From<JsonWebTokenDataVariant> for JwtClaimsVariant {
    fn from(variant: JsonWebTokenDataVariant) -> Self {
        match variant {
            JsonWebTokenDataVariant::AccessToken => JwtClaimsVariant::AccessToken,
            JsonWebTokenDataVariant::RefreshToken => JwtClaimsVariant::RefreshToken,
        }
    }
}

impl From<JwtClaims> for JsonWebTokenData {
    fn from(data: JwtClaims) -> Self {
        Self {
            id: data.id,
            user_id: data.user_id,
            expires_at: data.expires_at,
            variant: data.variant.into(),
        }
    }
}

impl From<JwtClaimsVariant> for JsonWebTokenDataVariant {
    fn from(variant: JwtClaimsVariant) -> Self {
        match variant {
            JwtClaimsVariant::AccessToken => JsonWebTokenDataVariant::AccessToken,
            JwtClaimsVariant::RefreshToken => JsonWebTokenDataVariant::RefreshToken,
        }
    }
}
