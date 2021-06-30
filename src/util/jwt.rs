use crate::error::UserServerError;
use chrono::Local;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

static JWT_SECRET_KEY: Lazy<String> =
    Lazy::new(|| std::env::var("JWT_SECRET_KEY").expect("未设置 JWT_SECRET_KEY"));

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub grant_type: String,
    pub email: String,
    pub sub: u32,
    pub exp: usize,
    pub iat: usize,
}

impl Claims {
    fn new(grant_type: String, exp: u32, sub: u32, email: String) -> Claims {
        let now = Local::now().timestamp() as usize;
        Claims {
            grant_type,
            email,
            sub,
            exp: now + exp as usize,
            iat: now,
        }
    }
}

pub fn get_token(
    grant_type: String,
    exp: u32,
    sub: u32,
    email: String,
) -> Result<String, UserServerError> {
    let claims = Claims::new(grant_type, exp, sub, email);
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET_KEY.as_bytes()),
    );
    match token {
        Ok(token) => Ok(token),
        Err(err) => Err(UserServerError::JWTGenerationError(err.to_string())),
    }
}

pub fn verify(token: &str) -> Result<Claims, UserServerError> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET_KEY.as_bytes()),
        &Validation::new(Algorithm::HS256),
    )?;
    Ok(token_data.claims)
}
