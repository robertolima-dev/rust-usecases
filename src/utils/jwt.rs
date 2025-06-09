use crate::models::auth::Claims;
use chrono::{Duration, Utc};
use dotenvy::dotenv;
use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode,
};
use std::env;

// use serde::Deserialize;
// #[derive(Debug, Deserialize)]
// pub struct Claims {
//     pub sub: String,
//     pub exp: usize,
// }

pub fn generate_jwt(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    dotenvy::dotenv().ok(); // Caso ainda não esteja carregado

    let secret = env::var("JWT_SECRET").expect("JWT_SECRET não definida");

    let expires_in: i64 = env::var("JWT_EXPIRES_IN")
        .unwrap_or_else(|_| "86400".to_string()) // padrão 24h
        .parse()
        .unwrap_or(86400);

    let exp = Utc::now()
        .checked_add_signed(Duration::seconds(expires_in))
        .expect("Erro ao calcular expiração do token")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        exp,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

#[allow(dead_code)]
pub fn verify_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    dotenv().ok();
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET não definida");

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )?;

    Ok(token_data.claims)
}

pub fn decode_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET não definido");
    let token_data: TokenData<Claims> = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    )?;

    Ok(token_data.claims)
}
