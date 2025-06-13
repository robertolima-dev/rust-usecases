use crate::config::get_settings;
use crate::errors::app_error::AppError;
use crate::models::course::{
    Course, CourseQuery, CreateCourseRequest, PaginatedCourseResponse, UpdateCourseRequest,
};
use crate::repositories::course_repository;
use chrono::Utc;
use elasticsearch::{Elasticsearch, IndexParts, SearchParts};
use serde_json::{Value, json};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn create_course_service(
    payload: CreateCourseRequest,
    author_id: Uuid,
    db: &PgPool,
    es_client: &Elasticsearch,
) -> Result<Course, anyhow::Error> {
    println!("payload: {:?}", payload);
    let now = Utc::now().naive_utc();
    let course = Course {
        id: Uuid::new_v4(),
        name: payload.name,
        description: payload.description,
        is_active: true,
        price: payload.price,
        month_duration: payload.month_duration,
        author_id,
        dt_start: payload.dt_start,
        dt_created: now,
        dt_updated: now,
        dt_deleted: None,
    };

    let mut tx = db.begin().await?;
    course_repository::create_course_in_tx(&course, &mut tx).await?;
    tx.commit().await?;

    // 🔍 Indexa no Elasticsearch
    let doc = json!({
        "id": course.id,
        "name": course.name,
        "description": course.description,
        "is_active": course.is_active,
        "price": course.price,
        "month_duration": course.month_duration,
        "author_id": course.author_id,
        "dt_start": course.dt_start,
        "dt_created": course.dt_created,
    });

    let settings = get_settings();
    let index_name = format!("{}_courses", settings.elasticsearch.index_prefix);
    es_client
        .index(elasticsearch::IndexParts::IndexId(
            &index_name,
            course.id.to_string().as_str(),
        ))
        .body(doc)
        .send()
        .await?;

    Ok(course)
}

pub async fn update_course_and_sync(
    id: Uuid,
    payload: UpdateCourseRequest,
    user_id: Uuid,
    db: &PgPool,
    es_client: &Elasticsearch,
) -> Result<Course, AppError> {
    let existing = course_repository::find_course_by_id(id, db)
        .await
        .map_err(|_| AppError::NotFound(Some("Curso não encontrado".into())))?;

    if existing.author_id != user_id {
        return Err(AppError::Unauthorized(Some(
            "Você não tem permissão para alterar este curso".into(),
        )));
    }

    let course = course_repository::update_course(id, user_id, &payload, db)
        .await
        .map_err(|e| AppError::InternalError(Some(format!("Erro ao atualizar curso: {e}"))))?;

    let settings = get_settings();
    let index = format!("{}_courses", settings.elasticsearch.index_prefix);

    let json_body = json!({
        "id": course.id,
        "name": course.name,
        "description": course.description,
        "is_active": course.is_active,
        "price": course.price,
        "month_duration": course.month_duration,
        "author_id": course.author_id,
        "dt_start": course.dt_start,
    });

    es_client
        .index(IndexParts::IndexId(&index, &course.id.to_string()))
        .body(json_body)
        .send()
        .await
        .map_err(|e| {
            AppError::InternalError(Some(format!("Erro ao atualizar Elasticsearch: {e}")))
        })?;

    Ok(course)
}

pub async fn search_courses(
    query: CourseQuery,
    es: &Elasticsearch,
) -> Result<PaginatedCourseResponse, AppError> {
    let settings = get_settings();
    let index = format!("{}_courses", settings.elasticsearch.index_prefix);

    // Monta o filtro com bool/must
    let mut must_clauses = vec![];

    if let Some(name) = &query.name {
        must_clauses.push(json!({
            "match_phrase_prefix": {
                "name": name
            }
        }));
    }

    if let Some(is_active) = query.is_active {
        must_clauses.push(json!({
            "term": {
                "is_active": is_active
            }
        }));
    }

    if let Some(author_id) = query.author_id {
        must_clauses.push(json!({
            "term": {
                "author_id": author_id.to_string()
            }
        }));
    }

    let query_body = json!({
        "query": {
            "bool": {
                "must": must_clauses
            }
        },
        "from": query.offset.unwrap_or(0),
        "size": query.limit.unwrap_or(10),
        "sort": [{ "dt_start": { "order": "desc" } }]
    });

    let response = es
        .search(SearchParts::Index(&[&index]))
        .body(query_body)
        .send()
        .await
        .map_err(|e| AppError::InternalError(Some(format!("Erro no Elasticsearch: {e}"))))?;

    let body: Value = response
        .json()
        .await
        .map_err(|e| AppError::InternalError(Some(format!("Erro ao parsear resultado: {e}"))))?;

    let count = body["hits"]["total"]["value"].as_u64().unwrap_or(0) as usize;

    let hits = body["hits"]["hits"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .filter_map(|hit| serde_json::from_value(hit["_source"].clone()).ok())
        .collect();

    Ok(PaginatedCourseResponse {
        results: hits,
        count,
    })
}

pub async fn delete_course(
    db: &sqlx::PgPool,
    es: &Elasticsearch,
    course_id: Uuid,
) -> Result<(), AppError> {
    // 1. Soft delete no Postgres
    course_repository::soft_delete_course_by_id(db, course_id).await?;

    // 2. Remoção do Elasticsearch
    let settings = get_settings();
    let index = format!("{}_courses", settings.elasticsearch.index_prefix);

    let response = es
        .delete(elasticsearch::DeleteParts::IndexId(
            &index,
            &course_id.to_string(),
        ))
        .send()
        .await;

    if let Err(e) = response {
        eprintln!("Erro ao remover índice do Elasticsearch: {:?}", e);
        // Não causará erro HTTP, mas será registrado.
    }

    Ok(())
}
