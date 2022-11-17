use std::str::FromStr;

use crate::{base::user::User, error::Error, Result};
use chrono::Utc;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    sub: String,
    name: String,
    exp: usize,
}

pub enum JwtType {
    Access,
    Refresh,
}

pub fn encode_jwt(user: &User, token_type: JwtType) -> Result<String> {
    let jwt_secret = match token_type {
        JwtType::Access => std::env::var("JWT_ACCESS").map_err(Error::Var)?,
        JwtType::Refresh => std::env::var("JWT_REFRESH").map_err(Error::Var)?,
    };

    let duration = match token_type {
        JwtType::Access => chrono::Duration::minutes(30),
        JwtType::Refresh => chrono::Duration::hours(2),
    };

    let expiration = Utc::now()
        .checked_add_signed(duration)
        .ok_or("Cannot create the duration for jwt")?
        .timestamp();

    let claims = Claims {
        sub: user.id.to_string(),
        name: user.username.clone(),
        exp: expiration as usize,
    };

    let header = Header::new(Algorithm::HS512);

    let jwt = encode(
        &header,
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )?;

    Ok(jwt)
}

pub fn decode_jwt(jwt: String, token_type: JwtType) -> Result<ObjectId> {
    let jwt_secret = match token_type {
        JwtType::Access => std::env::var("JWT_ACCESS").map_err(Error::Var)?,
        JwtType::Refresh => std::env::var("JWT_REFRESH").map_err(Error::Var)?,
    };

    let decoded = decode::<Claims>(
        &jwt,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::new(Algorithm::HS512),
    )
    .map_err(Error::Jwt)?;

    let oid = ObjectId::from_str(&decoded.claims.sub)?;
    Ok(oid)
}
