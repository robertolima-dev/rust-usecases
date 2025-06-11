use crate::errors::app_error::AppError;
use crate::models::token::UserToken;
use crate::repositories::token_repository;
use chrono::{Duration, Utc};
use rand::{Rng, distributions::Alphanumeric};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn create_user_token(
    user_id: Uuid,
    token_type: &str,
    db: &PgPool,
) -> Result<String, AppError> {
    let code: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(64)
        .map(char::from)
        .collect();

    let expires_at = Utc::now()
        .checked_add_signed(Duration::minutes(180))
        .expect("Erro ao calcular expiração")
        .naive_utc();

    token_repository::create_token(db, user_id, &code, token_type, expires_at).await?;

    Ok(code)
}

pub async fn get_and_validate_token(
    code: &str,
    expected_type: &str,
    db: &PgPool,
) -> Result<UserToken, AppError> {
    let token = token_repository::find_token_by_code(db, code)
        .await?
        .ok_or_else(|| AppError::NotFound(Some("Token não encontrado".into())))?;

    if token.token_type != expected_type {
        return Err(AppError::Unauthorized(Some(
            "Tipo de token inválido".into(),
        )));
    }

    if token.expires_at < Utc::now().naive_utc() {
        return Err(AppError::Unauthorized(Some("Token expirado".into())));
    }

    if token.consumed.unwrap_or(false) {
        return Err(AppError::Unauthorized(Some("Token já utilizado".into())));
    }

    Ok(token)
}
