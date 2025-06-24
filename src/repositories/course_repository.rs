use crate::errors::app_error::AppError;
use crate::models::course::{Course, CourseCategory, UpdateCourseRequest};
use sqlx::{Error, PgPool, Postgres, Transaction, types::chrono::Utc};
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
    .map_err(|e| AppError::DatabaseError(Some(format!("Erro no banco ao buscar curso: {}", e))))
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
    .await
    .map(|_| ())
}

pub async fn update_course(
    id: Uuid,
    user_id: Uuid,
    payload: &UpdateCourseRequest,
    tx: &mut Transaction<'_, Postgres>,
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
    .fetch_one(&mut **tx)
    .await?;

    Ok(course)
}

pub async fn soft_delete_course_by_id(db: &PgPool, course_id: Uuid) -> Result<(), AppError> {
    let now = Utc::now().naive_utc();

    let result = sqlx::query!(
        r#"
        UPDATE courses
        SET dt_deleted = $1, dt_updated = $1
        WHERE id = $2
        "#,
        now,
        course_id
    )
    .execute(db)
    .await
    .map_err(|e| {
        eprintln!("Erro ao deletar curso: {:?}", e);
        AppError::DatabaseError(Some(format!("Erro ao deletar curso: {}", e)))
    })?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(Some("Curso não encontrado".into())));
    }

    Ok(())
}

pub async fn add_category_to_course(
    course_id: Uuid,
    category_id: Uuid,
    tx: &mut Transaction<'_, Postgres>,
) -> Result<CourseCategory, sqlx::Error> {
    let row = sqlx::query!(
        r#"
        INSERT INTO course_categories (course_id, category_id)
        VALUES ($1, $2)
        RETURNING id, course_id, category_id
        "#,
        course_id,
        category_id
    )
    .fetch_one(&mut **tx)
    .await?;

    Ok(CourseCategory {
        id: row.id,
        course_id: row.course_id,
        category_id: row.category_id,
    })
}

pub async fn delete_categories_by_course(
    course_id: Uuid,
    tx: &mut Transaction<'_, Postgres>,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        DELETE FROM course_categories
        WHERE course_id = $1
        "#,
        course_id
    )
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn get_category_names_by_ids(
    category_ids: Vec<Uuid>,
    db: &PgPool,
) -> Result<Vec<String>, sqlx::Error> {
    let rows = sqlx::query!(
        r#"
        SELECT name
        FROM categories
        WHERE id = ANY($1) AND dt_deleted IS NULL
        "#,
        &category_ids
    )
    .fetch_all(db)
    .await?;

    Ok(rows.into_iter().map(|r| r.name).collect())
}

pub async fn get_category_names_by_course(
    course_id: Uuid,
    db: &PgPool,
) -> Result<Vec<String>, sqlx::Error> {
    let rows = sqlx::query!(
        r#"
        SELECT c.name
        FROM course_categories cc
        JOIN categories c ON c.id = cc.category_id
        WHERE cc.course_id = $1 AND c.dt_deleted IS NULL
        "#,
        course_id
    )
    .fetch_all(db)
    .await?;

    Ok(rows.into_iter().map(|r| r.name).collect())
}
