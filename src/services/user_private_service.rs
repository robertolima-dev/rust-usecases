use crate::errors::app_error::AppError;
use crate::models::user::{UpdateUserRequest, User, UserResponse, UserWithProfile};
use crate::repositories::{profile_repository, user_repository};
use crate::utils::pagination::PaginatedResponse;
use sqlx::{PgPool, Result};
use std::env;
use uuid::Uuid;

pub async fn get_me_by_user_id(
    user_id: Uuid,
    db: &PgPool,
    token: String,
) -> Result<UserResponse, AppError> {
    let user = user_repository::find_user_by_id(user_id, db).await?;
    let profile = profile_repository::find_profile_by_user_id(user.id, db).await?;

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
    db: &PgPool,
    limit: i64,
    offset: i64,
) -> Result<PaginatedResponse<User>, AppError> {
    let count = user_repository::count_all_users(db)
        .await
        .map_err(|_| AppError::InternalError(Some("Erro ao contar usuários".into())))?;

    let users = user_repository::list_users(db, limit, offset)
        .await
        .map_err(|_| AppError::InternalError(Some("Erro ao listar usuários".into())))?;

    Ok(PaginatedResponse {
        count,
        results: users,
        limit,
        offset,
    })
}

pub async fn update_logged_user(
    db: &PgPool,
    user_id: Uuid,
    req: UpdateUserRequest,
) -> Result<UserResponse, AppError> {
    user_repository::find_user_by_id(user_id, db).await?;

    let user = user_repository::update_user_fields(db, user_id, req.first_name, req.last_name)
        .await
        .map_err(|_| AppError::InternalError(Some("Erro ao atualizar usuário".into())))?;

    let profile = profile_repository::find_profile_by_user_id(user.id, db)
        .await
        .map_err(|_| AppError::InternalError(Some("Erro ao buscar perfil".into())))?;

    let token = "".to_string();
    let expires_in = std::env::var("JWT_EXPIRES_IN").unwrap_or("86400".to_string());

    Ok(UserResponse::from(
        UserWithProfile::from_user_and_profile(user, profile),
        token,
        expires_in,
    ))
}

pub async fn delete_logged_user(db: &PgPool, user_id: Uuid) -> Result<(), AppError> {
    user_repository::find_user_by_id(user_id, db).await?;
    let affected = user_repository::soft_delete_user(db, user_id)
        .await
        .map_err(|_| AppError::InternalError(Some("Erro ao deletar usuário".into())))?;

    if affected == 0 {
        return Err(AppError::NotFound(Some("Usuário não encontrado".into())));
    }

    Ok(())
}
