use crate::errors::ApiError;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use argon2rs::argon2i_simple;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct PrivateClaim {
    pub user_id: i32,
    pub username: String,
    exp: i64,
}

impl PrivateClaim {
    pub fn new(user_id: i32, username: String) -> Self {
        Self {
            user_id,
            username,
            exp: (Utc::now() + Duration::hours(24)).timestamp(),
        }
    }
}

/// Create a json web token (JWT)
pub fn create_jwt(private_claim: PrivateClaim) -> Result<String, ApiError> {
    let encoding_key = EncodingKey::from_secret(
        b"4125442A472D4B614E645267556B58703273357638792F423F4528482B4D6251",
    );
    encode(&Header::default(), &private_claim, &encoding_key)
        .map_err(|e| ApiError::CannotEncodeJwtToken(e.to_string()))
}

/// Decode a json web token (JWT)
pub fn decode_jwt(token: &str) -> Result<PrivateClaim, ApiError> {
    let decoding_key = DecodingKey::from_secret(
        b"4125442A472D4B614E645267556B58703273357638792F423F4528482B4D6251",
    );
    decode::<PrivateClaim>(token, &decoding_key, &Validation::default())
        .map(|data| data.claims)
        .map_err(|e| ApiError::CannotDecodeJwtToken(e.to_string()))
}

/// Encrypt a password
///
/// Uses the argon2i algorithm.
/// auth_salt is environment-configured.
pub fn hash(password: &str) -> String {
    argon2i_simple(&password, "AKUIMUET")
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect()
}

/// Gets the identidy service for injection into an Actix app
pub fn get_identity_service() -> IdentityService<CookieIdentityPolicy> {
    IdentityService::new(
        CookieIdentityPolicy::new(
            b"4125442A472D4B614E645267556B58703273357638792F423F4528482B4D6251",
        )
        .name("auth")
        .max_age_time(chrono::Duration::minutes(20))
        .secure(false),
    )
}
