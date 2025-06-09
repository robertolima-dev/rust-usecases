use crate::models::user::User;
use sqlx::PgPool;
use uuid::Uuid;

#[allow(dead_code)]
pub async fn find_user_by_id(id: Uuid, db: &PgPool) -> Result<User, sqlx::Error> {
    sqlx::query_as!(
        User,
        r#"
        SELECT id, username, email, first_name, last_name, password, dt_created, dt_updated
        FROM users
        WHERE id = $1
        "#,
        id
    )
    .fetch_one(db)
    .await
}
