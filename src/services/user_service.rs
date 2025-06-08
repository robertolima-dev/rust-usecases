use crate::models::user::User;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn get_all_users(db: &PgPool) -> Result<Vec<User>, sqlx::Error> {
    let users = sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(db)
        .await?;

    Ok(users)
}

pub async fn create_user(user: User, db: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO users (id, name, email, phone, birth_date, password)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        user.id,
        user.name,
        user.email,
        user.phone,
        user.birth_date,
        user.password
    )
    .execute(db)
    .await?;

    Ok(())
}

pub async fn get_user_by_id(id: Uuid, db: &PgPool) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT id, name, email, phone, birth_date, password, created_at
        FROM users
        WHERE id = $1
        "#,
        id
    )
    .fetch_one(db)
    .await?;

    Ok(user)
}
