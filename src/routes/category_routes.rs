use crate::config::app_state::AppState;
use crate::errors::app_error::AppError;
use crate::extensions::request_user_ext::RequestUserExt;
use crate::models::category::{
    Category, CategoryQuery, CreateCategoryRequest, UpdateCategoryRequest,
};
use crate::services::category_service;
use actix_web::{HttpRequest, HttpResponse, delete, get, post, put, web};
use uuid::Uuid;

#[post("/categories/")]
pub async fn create_category(
    req: HttpRequest,
    payload: web::Json<CreateCategoryRequest>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
    let access_level = req.access_level()?;
    if access_level != "admin" {
        return Ok(HttpResponse::Forbidden().body("Permissão negada"));
    }

    let category: Category =
        category_service::create_category_service(payload.into_inner(), &state).await?;

    Ok(HttpResponse::Ok().json(category))
}

#[get("/categories/")]
pub async fn list_categories(
    query: web::Query<CategoryQuery>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
    let limit = query.limit.unwrap_or(10);
    let offset = query.offset.unwrap_or(0);

    let categories = category_service::list_categories_service(limit, offset, &state).await?;

    Ok(HttpResponse::Ok().json(categories))
}

#[put("/categories/{id}/")]
pub async fn update_category(
    req: HttpRequest,
    id: web::Path<Uuid>,
    payload: web::Json<UpdateCategoryRequest>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
    let access_level = req.access_level()?;
    if access_level != "admin" {
        return Ok(HttpResponse::Forbidden().body("Permissão negada"));
    }

    let category =
        category_service::update_category_service(id.into_inner(), payload.into_inner(), &state)
            .await?;

    Ok(HttpResponse::Ok().json(category))
}

#[delete("/categories/{id}/")]
pub async fn delete_category(
    req: HttpRequest,
    id: web::Path<Uuid>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
    let access_level = req.access_level()?;
    if access_level != "admin" {
        return Ok(HttpResponse::Forbidden().body("Permissão negada"));
    }

    category_service::soft_delete_category_service(id.into_inner(), &state).await?;
    Ok(HttpResponse::NoContent().finish())
}
