use crate::errors::app_error::AppError;
use crate::models::user::User;
use chrono::Utc;
use sqlx::{PgPool, Postgres, Result, Transaction};
use uuid::Uuid;

#[allow(dead_code)]
pub async fn find_user_by_id(id: Uuid, db: &PgPool) -> Result<User, AppError> {
    sqlx::query_as!(
        User,
        r#"
        SELECT id, username, email, first_name, last_name, password, dt_created, dt_updated, dt_deleted
        FROM users
        WHERE dt_deleted IS NULL AND id = $1
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
        SELECT id, username, email, first_name, last_name, password, dt_created, dt_updated, dt_deleted
        FROM users
        WHERE dt_deleted IS NULL AND email = $1
        "#,
        email
    )
    .fetch_optional(db)
    .await
    .map_err(|_| AppError::NotFound(Some("Usuário não encontrado".into())))
}

pub async fn count_all_users(db: &PgPool) -> Result<i64> {
    let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users WHERE dt_deleted IS NULL")
        .fetch_one(db)
        .await?;
    Ok(row.0)
}

#[allow(dead_code)]
pub async fn list_users(db: &PgPool, limit: i64, offset: i64) -> Result<Vec<User>, AppError> {
    sqlx::query_as!(
        User,
        r#"
        SELECT id, username, email, first_name, last_name, password,
               dt_created, dt_updated, dt_deleted
        FROM users
        WHERE dt_deleted IS NULL
        ORDER BY dt_created DESC
        LIMIT $1 OFFSET $2
        "#,
        limit,
        offset
    )
    .fetch_all(db)
    .await
    .map_err(|_| AppError::BadRequest(Some("Erro ao listar usuários".into())))
}

pub async fn create_user_in_tx(
    user: &User,
    tx: &mut Transaction<'_, Postgres>,
) -> Result<(), AppError> {
    let result = sqlx::query!(
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
    .await;

    match result {
        Ok(_) => Ok(()),
        Err(e) => {
            if let Some(db_err) = e.as_database_error() {
                if let Some(code) = db_err.code() {
                    if code == "23505" {
                        // Código 23505 = unique_violation no PostgreSQL
                        return Err(AppError::Conflict(Some("Email já cadastrado".into())));
                    }
                }
            }
            Err(AppError::BadRequest(Some("Erro ao criar usuário".into())))
        }
    }
}

#[allow(dead_code)]
pub async fn update_user(user_id: Uuid, user: &User, db: &PgPool) -> Result<(), AppError> {
    sqlx::query!(
        r#"
        UPDATE users
        SET username = $1, email = $2, first_name = $3, last_name = $4, dt_updated = $5
        WHERE dt_deleted IS NULL AND id = $6
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
    .map_err(|_| AppError::BadRequest(Some("Erro ao atualizar usuário".into())))?;

    Ok(())
}

pub async fn update_user_fields(
    db: &PgPool,
    user_id: Uuid,
    first_name: Option<String>,
    last_name: Option<String>,
) -> Result<User> {
    let user = sqlx::query_as!(
        User,
        r#"
        UPDATE users
        SET 
            first_name = COALESCE($1, first_name),
            last_name = COALESCE($2, last_name),
            dt_updated = $3
        WHERE id = $4 AND dt_deleted IS NULL
        RETURNING id, username, email, first_name, last_name, password,
                  dt_created, dt_updated, dt_deleted
        "#,
        first_name,
        last_name,
        Utc::now().naive_utc(),
        user_id
    )
    .fetch_one(db)
    .await?;

    Ok(user)
}

pub async fn soft_delete_user(db: &PgPool, user_id: Uuid) -> Result<u64> {
    let result = sqlx::query!(
        r#"
        UPDATE users
        SET dt_deleted = $1
        WHERE id = $2
        "#,
        Utc::now().naive_utc(),
        user_id
    )
    .execute(db)
    .await?;

    Ok(result.rows_affected())
}

pub async fn update_user_password(
    db: &PgPool,
    user_id: Uuid,
    hashed_password: &str,
) -> Result<(), AppError> {
    sqlx::query!(
        r#"
        UPDATE users
        SET password = $1,
            dt_updated = NOW()
        WHERE id = $2
        "#,
        hashed_password,
        user_id
    )
    .execute(db)
    .await
    .map_err(|err| {
        eprintln!("Erro ao atualizar senha: {:?}", err);
        AppError::InternalError(Some("Erro ao atualizar senha".into()))
    })?;

    Ok(())
}
