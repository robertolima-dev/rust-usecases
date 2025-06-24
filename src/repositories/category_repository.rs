use crate::models::category::{Category, CreateCategoryRequest, UpdateCategoryRequest};
use sqlx::{PgPool, Result};
use uuid::Uuid;

pub async fn create_category(
    data: CreateCategoryRequest,
    db: &PgPool,
) -> Result<Category, sqlx::Error> {
    let category = sqlx::query_as!(
        Category,
        r#"
        INSERT INTO categories (name)
        VALUES ($1)
        RETURNING id, name, dt_created, dt_updated, dt_deleted
        "#,
        data.name
    )
    .fetch_one(db)
    .await?;

    Ok(category)
}

pub async fn count_all_categories(db: &PgPool) -> Result<i64> {
    let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM categories WHERE dt_deleted IS NULL")
        .fetch_one(db)
        .await?;
    Ok(row.0)
}

pub async fn list_categories(
    limit: i64,
    offset: i64,
    db: &PgPool,
) -> Result<Vec<Category>, sqlx::Error> {
    let categories = sqlx::query_as!(
        Category,
        r#"
        SELECT id, name, dt_created, dt_updated, dt_deleted
        FROM categories
        WHERE dt_deleted IS NULL
        ORDER BY dt_created DESC
        LIMIT $1 OFFSET $2
        "#,
        limit,
        offset
    )
    .fetch_all(db)
    .await?;

    Ok(categories)
}

pub async fn update_category(
    category_id: Uuid,
    data: UpdateCategoryRequest,
    db: &PgPool,
) -> Result<Category, sqlx::Error> {
    let category = sqlx::query_as!(
        Category,
        r#"
        UPDATE categories
        SET name = $1,
            dt_updated = now()
        WHERE id = $2 AND dt_deleted IS NULL
        RETURNING id, name, dt_created, dt_updated, dt_deleted
        "#,
        data.name,
        category_id
    )
    .fetch_one(db)
    .await?;

    Ok(category)
}

pub async fn soft_delete_category(category_id: Uuid, db: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE categories
        SET dt_deleted = now()
        WHERE id = $1 AND dt_deleted IS NULL
        "#,
        category_id
    )
    .execute(db)
    .await?;

    Ok(())
}
