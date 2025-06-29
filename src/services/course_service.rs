use crate::config::app_state::AppState;
use crate::config::get_settings;
use crate::errors::app_error::AppError;
use crate::models::course::{
    Course, CourseQuery, CreateCourseRequest, PaginatedCourseResponse, UpdateCourseRequest,
};
use crate::models::notification::ObjCodeType;
use crate::repositories::course_repository;
use crate::services::notification_service;
use crate::utils::logging::log_elastic_response;
use actix_web::web;
use anyhow::Context;
use chrono::Utc;
use elasticsearch::{IndexParts, SearchParts};
use serde_json::Value;
use serde_json::json;
use uuid::Uuid;

pub async fn create_course_service(
    payload: CreateCourseRequest,
    author_id: Uuid,
    state: &web::Data<AppState>,
) -> Result<Course, anyhow::Error> {
    let db = &state.db;
    let es_client = &state.es;

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
    course_repository::create_course_in_tx(&course, &mut tx)
        .await
        .context("Erro ao criar curso no banco")?;

    // 1. Vincular categorias (se tiver)
    if let Some(category_ids) = payload.category_ids.clone() {
        for category_id in category_ids {
            course_repository::add_category_to_course(course.id, category_id, &mut tx)
                .await
                .map_err(|e| anyhow::anyhow!(AppError::DatabaseError(Some(e.to_string()))))?;
        }
    }

    tx.commit().await?;

    // 🔍 Busca os nomes das categorias para indexar no Elasticsearch
    let categories = course_repository::get_category_names_by_course(course.id, db)
        .await
        .map_err(|e| {
            anyhow::anyhow!(AppError::InternalError(Some(format!(
                "Erro ao buscar categorias: {e}"
            ))))
        })?;

    let categories_json: Vec<Value> = categories
        .iter()
        .map(|cat| {
            json!({
                "id": cat.id,
                "name": cat.name,
            })
        })
        .collect();

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
        "categories": categories_json,
    });

    let settings = get_settings();
    let index_name = format!("{}_courses", settings.elasticsearch.index_prefix);
    let index_response = es_client
        .index(elasticsearch::IndexParts::IndexId(
            &index_name,
            course.id.to_string().as_str(),
        ))
        .body(doc)
        .send()
        .await?;

    log_elastic_response(index_response).await;

    // Cria notificação no Postgres e dispara via WebSocket
    notification_service::create_notification_and_emit(
        "Novo Curso Criado",
        &format!("Curso '{}' foi criado com sucesso", course.name),
        ObjCodeType::Platform,
        None,
        &state,
    )
    .await?;

    Ok(course)
}

pub async fn update_course_and_sync(
    id: Uuid,
    payload: UpdateCourseRequest,
    user_id: Uuid,
    state: &web::Data<AppState>,
) -> Result<Course, AppError> {
    let db = &state.db;
    let es_client = &state.es;

    let existing = course_repository::find_course_by_id(id, db)
        .await
        .map_err(|_| AppError::NotFound(Some("Curso não encontrado".into())))?;

    if existing.author_id != user_id {
        return Err(AppError::Unauthorized(Some(
            "Você não tem permissão para alterar este curso".into(),
        )));
    }

    let mut tx = db
        .begin()
        .await
        .map_err(|e| AppError::InternalError(Some(format!("Falha ao iniciar transação: {e}"))))?;

    // Atualiza curso
    let course = course_repository::update_course(id, user_id, &payload, &mut tx)
        .await
        .map_err(|e| AppError::InternalError(Some(format!("Erro ao atualizar curso: {e}"))))?;

    // Limpa categorias antigas
    course_repository::delete_categories_by_course(id, &mut tx)
        .await
        .map_err(|e| AppError::InternalError(Some(format!("Erro ao limpar categorias: {e}"))))?;

    // Se veio nova lista de categorias, insere elas
    if let Some(category_ids) = &payload.category_ids {
        for category_id in category_ids {
            course_repository::add_category_to_course(id, *category_id, &mut tx)
                .await
                .map_err(|e| {
                    AppError::InternalError(Some(format!("Erro ao adicionar categoria: {e}")))
                })?;
        }
    }

    tx.commit()
        .await
        .map_err(|e| AppError::InternalError(Some(format!("Erro ao commitar transação: {e}"))))?;

    // 🔍 Busca os nomes das categorias para indexar no Elasticsearch
    let categories = course_repository::get_category_names_by_course(id, db)
        .await
        .map_err(|e| AppError::InternalError(Some(format!("Erro ao buscar categorias: {e}"))))?;

    let categories_json: Vec<Value> = categories
        .iter()
        .map(|cat| {
            json!({
                "id": cat.id,
                "name": cat.name,
            })
        })
        .collect();

    // 🔄 Atualiza o Elasticsearch
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
        "categories": categories_json,
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
    state: &web::Data<AppState>,
) -> Result<PaginatedCourseResponse, AppError> {
    let es_client = &state.es;
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

    if let Some(min_price) = query.min_price {
        must_clauses.push(json!({
            "range": {
                "price": { "gte": min_price }
            }
        }));
    }

    if let Some(max_price) = query.max_price {
        must_clauses.push(json!({
            "range": {
                "price": { "lte": max_price }
            }
        }));
    }

    if let Some(start_from) = query.start_from {
        must_clauses.push(json!({
            "range": {
                "dt_start": { "gte": start_from.format("%Y-%m-%d").to_string() }
            }
        }));
    }

    if let Some(month_duration) = query.month_duration {
        must_clauses.push(json!({
            "term": { "month_duration": month_duration }
        }));
    }

    if let Some(category_name) = query.category_name {
        must_clauses.push(json!({
            "term": {
                "categories.name": category_name.to_string()
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

    let response = es_client
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

pub async fn delete_course(course_id: Uuid, state: &web::Data<AppState>) -> Result<(), AppError> {
    let db = &state.db;
    let es_client = &state.es;

    // 1. Soft delete no Postgres
    course_repository::soft_delete_course_by_id(db, course_id).await?;

    // 2. Remoção do Elasticsearch
    let settings = get_settings();
    let index = format!("{}_courses", settings.elasticsearch.index_prefix);

    let response = es_client
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

pub async fn reindex_courses(state: &web::Data<AppState>) -> Result<i32, anyhow::Error> {
    let db = &state.db;
    let es_client = &state.es;
    let settings = get_settings();

    let courses = course_repository::get_all_active_courses(db).await?;

    let mut indexed = 0;
    for course in courses {
        let categories = course_repository::get_category_names_by_course(course.id, db)
            .await
            .unwrap_or_default();

        let categories_json: Vec<Value> = categories
            .into_iter()
            .map(|cat| json!({ "id": cat.id, "name": cat.name }))
            .collect();

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
            "categories": categories_json,
        });

        let index_name = format!("{}_courses", settings.elasticsearch.index_prefix);

        let response = es_client
            .index(elasticsearch::IndexParts::IndexId(
                &index_name,
                &course.id.to_string(),
            ))
            .body(doc)
            .send()
            .await?;

        log_elastic_response(response).await;

        indexed += 1;
    }

    Ok(indexed)
}
