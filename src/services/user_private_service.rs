use crate::config::app_state::AppState;
use crate::errors::app_error::AppError;
use crate::log_fail;
use crate::logs::model::LogLevel;
use crate::models::user::{UpdateUserRequest, User, UserResponse, UserWithProfile};
use crate::repositories::{profile_repository, user_repository};
use crate::utils::jwt::calculate_remaining_expiration;
use crate::utils::pagination::PaginatedResponse;
use actix_web::web;
use sqlx::Result;
use uuid::Uuid;

pub async fn get_me_by_user_id(
    user_id: Uuid,
    token: String,
    state: &web::Data<AppState>,
) -> Result<UserResponse, AppError> {
    let db = &state.db;
    let mongo_db = &state.mongo;

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
    let remaining_seconds = calculate_remaining_expiration(&token)
        .expect("Falha ao calcular tempo de expiração restante");

    Ok(UserResponse {
        user: user_with_profile,
        token,
        expires_in: remaining_seconds.to_string(),
    })
}

pub async fn list_users_paginated(
    user_id: Uuid,
    limit: i64,
    offset: i64,
    state: &web::Data<AppState>,
) -> Result<PaginatedResponse<User>, AppError> {
    let db = &state.db;
    let mongo_db = &state.mongo;

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
    user_id: Uuid,
    req: UpdateUserRequest,
    token: String,
    state: &web::Data<AppState>,
) -> Result<UserResponse, AppError> {
    let db = &state.db;
    let mongo_db = &state.mongo;

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

    let remaining_seconds = calculate_remaining_expiration(&token)
        .expect("Falha ao calcular tempo de expiração restante");

    Ok(UserResponse::from(
        UserWithProfile::from_user_and_profile(user, profile),
        token,
        remaining_seconds.to_string(),
    ))
}

pub async fn delete_logged_user(
    user_id: Uuid,
    state: &web::Data<AppState>,
) -> Result<(), AppError> {
    let db = &state.db;
    let mongo_db = &state.mongo;

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
