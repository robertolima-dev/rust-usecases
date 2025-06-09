use crate::models::{
    auth::LoginRequest,
    profile::Profile,
    user::{User, UserResponse, UserWithProfile},
};
use crate::utils::jwt::generate_jwt;
use actix_web::HttpResponse;
use sqlx::PgPool;
use std::env;

pub async fn login_user(payload: LoginRequest, db: &PgPool) -> Result<UserResponse, HttpResponse> {
    // 1. Buscar o usuário
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT id, username, email, first_name, last_name, password, dt_created, dt_updated
        FROM users WHERE email = $1
        "#,
        payload.email
    )
    .fetch_optional(db)
    .await
    .map_err(|err| {
        eprintln!("Erro na consulta: {}", err);
        HttpResponse::InternalServerError().finish()
    })?;

    let user = match user {
        Some(u) => u,
        None => return Err(HttpResponse::NotFound().json("❌ Usuário não encontrado")),
    };

    // 2. Validar senha
    if !user.verify_password(&payload.password) {
        return Err(HttpResponse::Unauthorized().json("❌ Senha incorreta"));
    }

    // 3. Buscar o profile
    let profile = sqlx::query_as_unchecked!(
        Profile,
        r#"
        SELECT id, user_id, bio, birth_date, phone, document, profession, avatar,
               confirm_email, unsubscribe, access_level, dt_created, dt_updated
        FROM profiles WHERE user_id = $1
        "#,
        user.id
    )
    .fetch_one(db)
    .await
    .map_err(|err| {
        eprintln!("Erro ao buscar profile: {}", err);
        HttpResponse::InternalServerError().finish()
    })?;

    // 4. Gerar token
    let token = generate_jwt(&user.id.to_string()).map_err(|err| {
        eprintln!("Erro ao gerar token: {}", err);
        HttpResponse::InternalServerError().finish()
    })?;

    let expires_in = env::var("JWT_EXPIRES_IN").unwrap_or_else(|_| "86400".to_string());

    let user_with_profile = UserWithProfile::from_user_and_profile(user, profile);
    let response = UserResponse::from(user_with_profile, token, expires_in);

    Ok(response)
}
