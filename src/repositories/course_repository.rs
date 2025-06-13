use crate::errors::app_error::AppError;
use crate::models::course::{Course, UpdateCourseRequest};
use sqlx::{Error, PgPool, Postgres, Transaction};
use uuid::Uuid;

pub async fn find_course_by_id(id: Uuid, db: &PgPool) -> Result<Course, AppError> {
    sqlx::query_as!(
        Course,
        r#"
        SELECT id, name, description, is_active, price, month_duration, author_id, dt_start, dt_created, dt_updated, dt_deleted
        FROM courses
        WHERE id = $1
        "#,
        id
    )
    .fetch_one(db)
    .await
    .map_err(|_| AppError::NotFound(Some("Curso não encontrado!".into())))
}

pub async fn create_course_in_tx(
    course: &Course,
    tx: &mut Transaction<'_, Postgres>,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO courses (id, name, description, is_active, price, month_duration, author_id, dt_start, dt_created, dt_updated)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        "#,
        course.id,
        course.name,
        course.description,
        course.is_active,
        course.price,
        course.month_duration,
        course.author_id,
        course.dt_start,
        course.dt_created,
        course.dt_updated
    )
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn update_course(
    id: Uuid,
    user_id: Uuid,
    payload: &UpdateCourseRequest,
    db: &PgPool,
) -> Result<Course, Error> {
    let course = sqlx::query_as!(
        Course,
        r#"
        UPDATE courses
        SET
            name = COALESCE($1, name),
            description = COALESCE($2, description),
            is_active = COALESCE($3, is_active),
            price = COALESCE($4, price),
            month_duration = COALESCE($5, month_duration),
            dt_start = COALESCE($6, dt_start),
            dt_updated = NOW()
        WHERE id = $7 AND author_id = $8
        RETURNING id, name, description, is_active, price, month_duration, author_id, dt_start, dt_created, dt_updated, dt_deleted
        "#,
        payload.name,
        payload.description,
        payload.is_active,
        payload.price,
        payload.month_duration,
        payload.dt_start,
        id,
        user_id
    )
    .fetch_one(db)
    .await?;

    Ok(course)
}
