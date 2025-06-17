use crate::config::app_state::AppState;
use crate::errors::app_error::AppError;
use crate::extensions::request_user_ext::RequestUserExt;
use crate::models::user::UpdateUserRequest;
use crate::services::user_private_service;
use crate::utils::pagination::PaginationParams;
use actix_web::{HttpRequest, HttpResponse, delete, get, patch, web};

#[get("/me/")]
pub async fn get_me(
    req: HttpRequest,
    state: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.strip_prefix("Token "))
        .map(|s| s.to_string())
        .ok_or(AppError::Unauthorized(None))?;
    let user_id = req.user_id()?;

    let response = user_private_service::get_me_by_user_id(user_id, token, &state).await?;
    Ok(HttpResponse::Ok().json(response))
}

#[get("/users/")]
pub async fn list_users(
    req: HttpRequest,
    query: web::Query<PaginationParams>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
    let user_id = req.user_id()?;
    let PaginationParams { limit, offset } = query.into_inner();

    let paginated = user_private_service::list_users_paginated(
        user_id, limit, offset, // db.get_ref(),
        &state,
    )
    .await?;

    Ok(HttpResponse::Ok().json(paginated))
}

#[patch("/users/")]
pub async fn update_user(
    req: HttpRequest,
    payload: web::Json<UpdateUserRequest>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
    let user_id = req.user_id()?;
    let response =
        user_private_service::update_logged_user(user_id, payload.into_inner(), &state).await?;
    Ok(HttpResponse::Ok().json(response))
}

#[delete("/users/")]
pub async fn delete_user(
    req: HttpRequest,
    state: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
    let user_id = req.user_id()?;
    user_private_service::delete_logged_user(user_id, &state).await?;
    Ok(HttpResponse::NoContent().finish())
}
