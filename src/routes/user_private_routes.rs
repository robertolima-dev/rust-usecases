use crate::errors::app_error::AppError;
use crate::extensions::request_user_ext::RequestUserExt;
use crate::models::user::UpdateUserRequest;
use crate::services::user_private_service::{
    delete_logged_user, get_me_by_user_id, list_users_paginated, update_logged_user,
};
use crate::utils::pagination::PaginationParams;
use actix_web::{HttpRequest, HttpResponse, delete, get, patch, web};
use sqlx::PgPool;

#[get("/me/")]
pub async fn get_me(req: HttpRequest, db: web::Data<PgPool>) -> Result<HttpResponse, AppError> {
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.strip_prefix("Token "))
        .map(|s| s.to_string())
        .ok_or(AppError::Unauthorized(None))?;
    let user_id = req.user_id()?;

    let response = get_me_by_user_id(user_id, db.get_ref(), token).await?;
    Ok(HttpResponse::Ok().json(response))
}

#[get("/users/")]
pub async fn list_users(
    db: web::Data<PgPool>,
    query: web::Query<PaginationParams>,
) -> Result<HttpResponse, AppError> {
    let PaginationParams { limit, offset } = query.into_inner();

    let paginated = list_users_paginated(db.get_ref(), limit, offset).await?;

    Ok(HttpResponse::Ok().json(paginated))
}

#[patch("/users/")]
pub async fn update_user(
    req: HttpRequest,
    payload: web::Json<UpdateUserRequest>,
    db: web::Data<PgPool>,
) -> Result<HttpResponse, AppError> {
    let user_id = req.user_id()?;
    let response = update_logged_user(db.get_ref(), user_id, payload.into_inner()).await?;
    Ok(HttpResponse::Ok().json(response))
}

#[delete("/users/")]
pub async fn delete_user(
    req: HttpRequest,
    db: web::Data<PgPool>,
) -> Result<HttpResponse, AppError> {
    let user_id = req.user_id()?;
    delete_logged_user(db.get_ref(), user_id).await?;
    Ok(HttpResponse::NoContent().finish())
}
