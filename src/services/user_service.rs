use crate::models::{
    profile::Profile,
    user::{User, UserRequest, UserResponse, UserWithProfile},
};
use crate::utils::jwt::generate_jwt;
use chrono::Utc;
use sqlx::PgPool;
use std::env;
use uuid::Uuid;

use crate::utils::jwt::decode_token;
use actix_web::HttpResponse;

pub async fn get_me_from_token(token: &str, db: &PgPool) -> Result<UserResponse, HttpResponse> {
    // 1. Decodificar token
    let claims =
        decode_token(token).map_err(|_| HttpResponse::Unauthorized().body("Token inválido"))?;

    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| HttpResponse::Unauthorized().body("ID inválido no token"))?;

    // 2. Buscar usuário
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT id, username, email, first_name, last_name, password, dt_created, dt_updated
        FROM users
        WHERE id = $1
        "#,
        user_id
    )
    .fetch_one(db)
    .await
    .map_err(|_| HttpResponse::NotFound().body("Usuário não encontrado"))?;

    // 3. Buscar profile
    let profile = sqlx::query_as_unchecked!(
        Profile,
        r#"
        SELECT id, user_id, bio, birth_date, phone, document, profession, avatar,
               confirm_email, unsubscribe, access_level, dt_created, dt_updated
        FROM profiles
        WHERE user_id = $1
        "#,
        user.id
    )
    .fetch_one(db)
    .await
    .map_err(|_| HttpResponse::InternalServerError().body("Erro ao buscar perfil"))?;

    let user_with_profile = UserWithProfile::from_user_and_profile(user, profile);
    let expires_in = env::var("JWT_EXPIRES_IN").unwrap_or("86400".to_string());

    Ok(UserResponse::from(
        user_with_profile,
        token.to_string(),
        expires_in,
    ))
}

/// Cria user + profile com base em UserRequest
pub async fn create_user_with_request(
    req: UserRequest,
    db: &PgPool,
) -> Result<UserResponse, sqlx::Error> {
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

    create_user_with_profile(&user, &profile, db).await?;

    let user_with_profile = UserWithProfile::from_user_and_profile(user, profile);

    let token = generate_jwt(&user_id.to_string()).expect("Falha ao gerar token");
    let expires_in = env::var("JWT_EXPIRES_IN").unwrap_or("604800".to_string());
    Ok(UserResponse::from(user_with_profile, token, expires_in))
}

/// Insere user e profile dentro de uma transação
pub async fn create_user_with_profile(
    user: &User,
    profile: &Profile,
    db: &PgPool,
) -> Result<(), sqlx::Error> {
    let mut tx = db.begin().await?;

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
    .execute(&mut *tx)
    .await?;

    sqlx::query!(
        r#"
        INSERT INTO profiles (
            id, user_id, bio, birth_date, phone, document, profession, avatar,
            confirm_email, unsubscribe, access_level, dt_created, dt_updated
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
        "#,
        profile.id,
        profile.user_id,
        profile.bio,
        profile.birth_date,
        profile.phone,
        profile.document,
        profile.profession,
        profile.avatar,
        profile.confirm_email,
        profile.unsubscribe,
        profile.access_level,
        profile.dt_created,
        profile.dt_updated
    )
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(())
}
