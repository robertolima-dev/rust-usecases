use crate::errors::app_error::AppError;
use crate::models::user::User;
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

#[allow(dead_code)]
pub async fn find_user_by_id(id: Uuid, db: &PgPool) -> Result<User, AppError> {
    sqlx::query_as!(
        User,
        r#"
        SELECT id, username, email, first_name, last_name, password, dt_created, dt_updated
        FROM users
        WHERE id = $1
        "#,
        id
    )
    .fetch_one(db)
    .await
    .map_err(|_| AppError::NotFound(Some("Usuário não encontrado!".into())))
}

#[allow(dead_code)]
pub async fn find_user_by_email(email: &str, db: &PgPool) -> Result<Option<User>, AppError> {
    sqlx::query_as!(
        User,
        r#"
        SELECT id, username, email, first_name, last_name, password, dt_created, dt_updated
        FROM users
        WHERE email = $1
        "#,
        email
    )
    .fetch_optional(db)
    .await
    .map_err(|_| AppError::NotFound(Some("Usuário não encontrado".into())))
}

#[allow(dead_code)]
pub async fn list_users(db: &PgPool) -> Result<Vec<User>, AppError> {
    sqlx::query_as!(
        User,
        r#"
        SELECT id, username, email, first_name, last_name, password, dt_created, dt_updated
        FROM users
        ORDER BY dt_created DESC
        "#
    )
    .fetch_all(db)
    .await
    .map_err(|_| AppError::InternalError(Some("Erro ao listar usuários".into())))
}

/// ✅ Cria o usuário usando uma transação (já aberta no service)
pub async fn create_user_in_tx(
    user: &User,
    tx: &mut Transaction<'_, Postgres>,
) -> Result<(), AppError> {
    sqlx::query!(
        r#"
        INSERT INTO users (id, username, email, first_name, last_name, password, dt_created, dt_updated)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        "#,
        user.id,
        user.username,
        user.email,
        user.first_name,
        user.last_name,
        user.password,
        user.dt_created,
        user.dt_updated
    )
    .execute(&mut **tx)
    .await
    .map_err(|_| AppError::InternalError(Some("Erro ao criar usuário".into())))?;

    Ok(())
}

#[allow(dead_code)]
pub async fn update_user(user_id: Uuid, user: &User, db: &PgPool) -> Result<(), AppError> {
    sqlx::query!(
        r#"
        UPDATE users
        SET username = $1, email = $2, first_name = $3, last_name = $4, dt_updated = $5
        WHERE id = $6
        "#,
        user.username,
        user.email,
        user.first_name,
        user.last_name,
        user.dt_updated,
        user_id
    )
    .execute(db)
    .await
    .map_err(|_| AppError::InternalError(Some("Erro ao atualizar usuário".into())))?;

    Ok(())
}
