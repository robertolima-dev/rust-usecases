// src/services/profile_service.rs
use crate::config::app_state::AppState;
use crate::errors::app_error::AppError;
use crate::models::profile::UpdateProfileRequest;
use crate::models::user::{UserResponse, UserWithProfile};
use crate::repositories::{profile_repository, user_repository};
use actix_web::web;
use uuid::Uuid;

pub async fn update_profile_by_user_id(
    user_id: Uuid,
    data: UpdateProfileRequest,
    state: &web::Data<AppState>,
) -> Result<UserResponse, AppError> {
    let db = &state.db;

    let user = user_repository::find_user_by_id(user_id, db).await?;

    profile_repository::update_profile_fields_by_user_id(user.id, &data, db)
        .await
        .map_err(|e| {
            eprintln!("Erro ao atualizar profile: {:?}", e);
            AppError::BadRequest(Some("Erro ao atualizar perfil".into()))
        })?;

    let profile = profile_repository::find_profile_by_user_id(user_id, db).await?;

    let token = "".to_string();
    let expires_in = std::env::var("JWT_EXPIRES_IN").unwrap_or_else(|_| "604800".into());

    Ok(UserResponse::from(
        UserWithProfile::from_user_and_profile(user, profile),
        token,
        expires_in,
    ))
}
