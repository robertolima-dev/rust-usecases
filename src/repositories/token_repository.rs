use crate::errors::app_error::AppError;
use crate::models::token::UserToken;
use chrono::NaiveDateTime;
use sqlx::{PgPool, Result};
use uuid::Uuid;

pub async fn create_token(
    db: &PgPool,
    user_id: Uuid,
    code: &str,
    token_type: &str,
    expires_at: NaiveDateTime,
) -> Result<(), AppError> {
    sqlx::query!(
        r#"
        INSERT INTO user_tokens (user_id, code, token_type, expires_at)
        VALUES ($1, $2, $3, $4)
        "#,
        user_id,
        code,
        token_type,
        expires_at,
    )
    .execute(db)
    .await
    .map_err(|err| {
        eprintln!("Erro ao criar token: {:?}", err);
        AppError::InternalError(Some("Erro ao criar token".into()))
    })?;

    Ok(())
}

pub async fn find_token_by_code(db: &PgPool, code: &str) -> Result<Option<UserToken>, AppError> {
    let result = sqlx::query_as!(
        UserToken,
        r#"
        SELECT id, user_id, code, token_type::TEXT, expires_at, created_at, consumed
        FROM user_tokens
        WHERE code = $1
        "#,
        code
    )
    .fetch_optional(db)
    .await
    .map_err(|err| {
        eprintln!("Erro ao buscar token por código: {:?}", err);
        AppError::InternalError(Some("Erro ao buscar token".into()))
    })?;

    Ok(result)
}

pub async fn update_token(db: &PgPool, code: &str) -> Result<(), AppError> {
    sqlx::query!(
        r#"
        UPDATE user_tokens
        SET consumed = true
        WHERE code = $1
        "#,
        code
    )
    .execute(db)
    .await
    .map_err(|err| {
        eprintln!("Erro ao atualizar token: {:?}", err);
        AppError::InternalError(Some("Erro ao atualizar token".into()))
    })?;

    Ok(())
}
