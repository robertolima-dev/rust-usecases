use crate::models::{
    profile::Profile,
    user::User,
    user_request::UserRequest,
    user_response::{UserResponse, UserWithProfile},
};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

/// Cria user + profile com base em UserRequest
pub async fn create_user_with_request(
    req: UserRequest,
    db: &PgPool,
) -> Result<UserResponse, sqlx::Error> {
    let user_id = Uuid::new_v4();
    let now = Utc::now().naive_utc();

    let user = User {
        id: user_id,
        username: req.username,
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

    Ok(UserResponse::from(user_with_profile))
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
