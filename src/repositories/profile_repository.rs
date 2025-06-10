use crate::errors::app_error::AppError;
use crate::models::profile::Profile;
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

#[allow(dead_code)]
pub async fn find_profile_by_user_id(user_id: Uuid, db: &PgPool) -> Result<Profile, AppError> {
    sqlx::query_as_unchecked!(
        Profile,
        r#"
        SELECT id, user_id, bio, birth_date, phone, document, profession, avatar,
               confirm_email, unsubscribe, access_level, dt_created, dt_updated
        FROM profiles
        WHERE user_id = $1
        "#,
        user_id
    )
    .fetch_one(db)
    .await
    .map_err(|_| AppError::InternalError(Some("Erro ao buscar perfil".into())))
}

pub async fn create_profile_in_tx(
    profile: &Profile,
    tx: &mut Transaction<'_, Postgres>,
) -> Result<(), AppError> {
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
    .execute(&mut **tx)
    .await
    .map_err(|_| AppError::InternalError(Some("Erro ao criar perfil".into())))?;

    Ok(())
}
