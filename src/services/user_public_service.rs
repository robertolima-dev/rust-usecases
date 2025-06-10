use crate::errors::app_error::AppError;
use crate::models::auth::LoginRequest;
use crate::models::{
    profile::Profile,
    user::{User, UserRequest, UserResponse, UserWithProfile},
};
use crate::repositories::{profile_repository, user_repository};
use crate::utils::jwt::generate_jwt;
use chrono::Utc;
use sqlx::PgPool;
use std::env;
use uuid::Uuid;

pub async fn login_user(payload: LoginRequest, db: &PgPool) -> Result<UserResponse, AppError> {
    // 1. Buscar usuário por email
    let user = user_repository::find_user_by_email(&payload.email, db)
        .await
        .map_err(|err| {
            eprintln!("Erro ao buscar usuário: {:?}", err);
            AppError::InternalError(Some("Erro ao buscar usuário".into()))
        })?
        .ok_or(AppError::NotFound(Some("Usuário não encontrado".into())))?;

    // 2. Verificar senha
    if !user.verify_password(&payload.password) {
        return Err(AppError::Unauthorized(Some("❌ Senha incorreta".into())));
    }

    // 3. Buscar o perfil
    let profile = profile_repository::find_profile_by_user_id(user.id, db)
        .await
        .map_err(|err| {
            eprintln!("Erro ao buscar perfil: {:?}", err);
            AppError::InternalError(Some("Erro ao buscar perfil".into()))
        })?;

    // 4. Gerar token JWT
    let token = generate_jwt(&user.id.to_string()).map_err(|err| {
        eprintln!("Erro ao gerar token: {:?}", err);
        AppError::InternalError(Some("Erro ao gerar token".into()))
    })?;

    let expires_in = env::var("JWT_EXPIRES_IN").unwrap_or_else(|_| "86400".to_string());
    let user_with_profile = UserWithProfile::from_user_and_profile(user, profile);

    Ok(UserResponse::from(user_with_profile, token, expires_in))
}

/// Cria user + profile com base em UserRequest
pub async fn create_user_with_request(
    req: UserRequest,
    db: &PgPool,
) -> Result<UserResponse, AppError> {
    let user_id = Uuid::new_v4();
    let now = Utc::now().naive_utc();

    fn generate_username_from_email(email: &str) -> String {
        let prefix = email.split('@').next().unwrap_or("");

        prefix
            .chars()
            .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
            .collect()
    }

    let user = User {
        id: user_id,
        username: generate_username_from_email(&req.email),
        email: req.email,
        first_name: req.first_name,
        last_name: req.last_name,
        password: bcrypt::hash(&req.password, bcrypt::DEFAULT_COST).expect("Erro ao hashear senha"),
        dt_created: now,
        dt_updated: now,
    };

    let profile = Profile::from_request(user_id, req.profile);

    create_user_and_profile(&user, &profile, db).await?;

    let user_with_profile = UserWithProfile::from_user_and_profile(user, profile);

    let token = generate_jwt(&user_id.to_string()).expect("Falha ao gerar token");
    let expires_in = env::var("JWT_EXPIRES_IN").unwrap_or("604800".to_string());
    Ok(UserResponse::from(user_with_profile, token, expires_in))
}

pub async fn create_user_and_profile(
    user: &User,
    profile: &Profile,
    db: &PgPool,
) -> Result<(), AppError> {
    let mut tx = db.begin().await.map_err(|err| {
        AppError::DatabaseError(Some(format!("Erro ao iniciar transação: {}", err)))
    })?;

    user_repository::create_user_in_tx(&user, &mut tx).await?;
    profile_repository::create_profile_in_tx(&profile, &mut tx).await?;

    tx.commit().await.map_err(|err| {
        AppError::DatabaseError(Some(format!("Erro ao commitar transação: {}", err)))
    })?;

    Ok(())
}
