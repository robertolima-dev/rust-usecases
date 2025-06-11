use crate::errors::app_error::AppError;
use crate::log_fail;
use crate::logs::model::LogLevel;
use crate::models::user::{UpdateUserRequest, User, UserResponse, UserWithProfile};
use crate::repositories::{profile_repository, user_repository};
use crate::utils::pagination::PaginatedResponse;
use mongodb::Database;
use sqlx::{PgPool, Result};
use std::env;
use uuid::Uuid;

pub async fn get_me_by_user_id(
    user_id: Uuid,
    db: &PgPool,
    mongo_db: &Database,
    token: String,
) -> Result<UserResponse, AppError> {
    // let user = user_repository::find_user_by_id(user_id, db).await?;
    // let profile = profile_repository::find_profile_by_user_id(user.id, db).await?;

    let user = match user_repository::find_user_by_id(user_id, db).await {
        Ok(p) => p,
        Err(err) => {
            log_fail!(
                err,
                LogLevel::Error,
                "Erro ao buscar user",
                "user_service",
                Some(user_id),
                mongo_db
            );
            return Err(AppError::BadRequest(Some("Erro ao buscar perfil".into())));
        }
    };
    let profile = match profile_repository::find_profile_by_user_id(user.id, db).await {
        Ok(p) => p,
        Err(err) => {
            log_fail!(
                err,
                LogLevel::Error,
                "Erro ao buscar perfil",
                "user_service",
                Some(user_id),
                mongo_db
            );
            return Err(AppError::BadRequest(Some("Erro ao buscar perfil".into())));
        }
    };

    // Montar resposta
    let user_with_profile = UserWithProfile::from_user_and_profile(user, profile);
    let expires_in = env::var("JWT_EXPIRES_IN").unwrap_or_else(|_| "86400".to_string());

    Ok(UserResponse {
        user: user_with_profile,
        token,
        expires_in,
    })
}

pub async fn list_users_paginated(
    user_id: Uuid,
    db: &PgPool,
    mongo_db: &Database,
    limit: i64,
    offset: i64,
) -> Result<PaginatedResponse<User>, AppError> {
    let count = user_repository::count_all_users(db)
        .await
        .map_err(|_| AppError::BadRequest(Some("Erro ao contar usuários".into())))?;

    let users = match user_repository::list_users(db, limit, offset).await {
        Ok(p) => p,
        Err(err) => {
            log_fail!(
                err,
                LogLevel::Error,
                "Erro ao listar users",
                "user_service",
                Some(user_id),
                mongo_db
            );
            return Err(AppError::BadRequest(Some("Erro ao listar users".into())));
        }
    };

    Ok(PaginatedResponse {
        count,
        results: users,
        limit,
        offset,
    })
}

pub async fn update_logged_user(
    db: &PgPool,
    mongo_db: &Database,
    user_id: Uuid,
    req: UpdateUserRequest,
) -> Result<UserResponse, AppError> {
    user_repository::find_user_by_id(user_id, db).await?;

    let user = match user_repository::update_user_fields(db, user_id, req.first_name, req.last_name)
        .await
    {
        Ok(p) => p,
        Err(err) => {
            log_fail!(
                err,
                LogLevel::Error,
                "Erro ao atualizar usuário",
                "user_service",
                Some(user_id),
                mongo_db
            );
            return Err(AppError::BadRequest(Some(
                "Erro ao atualizar usuário".into(),
            )));
        }
    };

    let profile = match profile_repository::find_profile_by_user_id(user.id, db).await {
        Ok(p) => p,
        Err(err) => {
            log_fail!(
                err,
                LogLevel::Error,
                "Erro ao buscar perfil",
                "user_service",
                Some(user_id),
                mongo_db
            );
            return Err(AppError::BadRequest(Some("Erro ao buscar perfil".into())));
        }
    };

    let token = "".to_string();
    let expires_in = std::env::var("JWT_EXPIRES_IN").unwrap_or("86400".to_string());

    Ok(UserResponse::from(
        UserWithProfile::from_user_and_profile(user, profile),
        token,
        expires_in,
    ))
}

pub async fn delete_logged_user(
    db: &PgPool,
    mongo_db: &Database,
    user_id: Uuid,
) -> Result<(), AppError> {
    user_repository::find_user_by_id(user_id, db).await?;
    let affected = match user_repository::soft_delete_user(db, user_id).await {
        Ok(p) => p,
        Err(err) => {
            log_fail!(
                err,
                LogLevel::Error,
                "Erro ao deletar usuário",
                "user_service",
                Some(user_id),
                mongo_db
            );
            return Err(AppError::BadRequest(Some("Erro ao deletar usuário".into())));
        }
    };

    if affected == 0 {
        return Err(AppError::NotFound(Some("Usuário não encontrado".into())));
    }

    Ok(())
}
