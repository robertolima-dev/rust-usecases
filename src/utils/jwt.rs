use crate::models::auth::Claims;
use crate::config::get_settings;
use chrono::{Duration, Utc};
use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode,
};

pub fn generate_jwt(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let settings = get_settings();
    let expires_in = settings.jwt.expires_in;

    let exp = Utc::now()
        .checked_add_signed(Duration::seconds(expires_in as i64))
        .expect("Erro ao calcular expiração do token")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        exp,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(settings.jwt.secret.as_bytes()),
    )
}

#[allow(dead_code)]
pub fn verify_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let settings = get_settings();

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(settings.jwt.secret.as_ref()),
        &Validation::default(),
    )?;

    Ok(token_data.claims)
}

#[allow(dead_code)]
pub fn decode_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let settings = get_settings();
    let token_data: TokenData<Claims> = decode::<Claims>(
        token,
        &DecodingKey::from_secret(settings.jwt.secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    )?;

    Ok(token_data.claims)
}
