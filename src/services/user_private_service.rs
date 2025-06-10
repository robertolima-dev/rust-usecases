use crate::errors::app_error::AppError;
use crate::models::user::{UserResponse, UserWithProfile};
use crate::repositories::{profile_repository, user_repository};
use sqlx::PgPool;
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
