use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub sub: u32,
    pub exp: usize,
}

pub fn create_jwt(user_id: u32) -> Result<String, jsonwebtoken::errors::Error> {
    let secret = std::env::var("SECRET").expect("Env Error: SECRET is not defined");
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;
    let claims = Claims {
        sub: user_id,
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(&secret.into_bytes()),
    )
}

pub fn verify_jwt(token: &str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    let secret = std::env::var("SECRET").expect("Env Error: SECRET is not defined");
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(&secret.into_bytes()),
        &Validation::new(jsonwebtoken::Algorithm::HS256),
    )
}
