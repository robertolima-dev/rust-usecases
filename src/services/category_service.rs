use crate::config::app_state::AppState;
use crate::errors::app_error::AppError;
use crate::models::category::{Category, CreateCategoryRequest, UpdateCategoryRequest};
use crate::repositories::category_repository;
use crate::utils::pagination::PaginatedResponse;
use actix_web::web;
use uuid::Uuid;

pub async fn create_category_service(
    data: CreateCategoryRequest,
    state: &web::Data<AppState>,
) -> Result<Category, AppError> {
    let db = &state.db;

    category_repository::create_category(data, db)
        .await
        .map_err(|e| AppError::DatabaseError(Some(e.to_string())))
}

pub async fn list_categories_service(
    limit: i64,
    offset: i64,
    state: &web::Data<AppState>,
) -> Result<PaginatedResponse<Category>, AppError> {
    let db = &state.db;

    let count = category_repository::count_all_categories(db)
        .await
        .map_err(|e| AppError::DatabaseError(Some(e.to_string())))?;

    let categories = category_repository::list_categories(limit, offset, db)
        .await
        .map_err(|e| AppError::DatabaseError(Some(e.to_string())))?;

    Ok(PaginatedResponse {
        count,
        results: categories,
        limit,
        offset,
    })
}

pub async fn update_category_service(
    category_id: Uuid,
    data: UpdateCategoryRequest,
    state: &web::Data<AppState>,
) -> Result<Category, AppError> {
    let db = &state.db;

    category_repository::update_category(category_id, data, db)
        .await
        .map_err(|e| AppError::DatabaseError(Some(e.to_string())))
}

pub async fn soft_delete_category_service(
    category_id: Uuid,
    state: &web::Data<AppState>,
) -> Result<(), AppError> {
    let db = &state.db;

    category_repository::soft_delete_category(category_id, db)
        .await
        .map_err(|e| AppError::DatabaseError(Some(e.to_string())))
}
