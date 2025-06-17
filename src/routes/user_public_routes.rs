use crate::config::app_state::AppState;
use crate::errors::app_error::AppError;
use crate::models::auth::{ChangePasswordRequest, ForgotPasswordRequest, LoginRequest};
use crate::models::user::UserRequest;
use crate::services::user_public_service;
use actix_web::{HttpResponse, Responder, get, post, web};

#[post("/users/")]
pub async fn create_user(
    user_data: web::Json<UserRequest>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
    let res = user_public_service::create_user_with_request(user_data.into_inner(), &state).await?;
    Ok(HttpResponse::Ok().json(res))
}

#[post("/login/")]
pub async fn login(
    payload: web::Json<LoginRequest>,
    state: web::Data<AppState>,
) -> Result<impl Responder, AppError> {
    let response = user_public_service::login_user(payload.into_inner(), &state).await?;
    Ok(web::Json(response))
}

#[get("/confirm-email/{code}/")]
pub async fn confirm_email(
    code: web::Path<String>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
    user_public_service::confirm_email(&code, &state).await?;
    Ok(HttpResponse::Ok().finish())
}

#[post("/forgot-password/")]
pub async fn forgot_password(
    payload: web::Json<ForgotPasswordRequest>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
    user_public_service::forgot_password(&payload.into_inner().email, &state).await?;
    Ok(HttpResponse::Ok().finish())
}

#[post("/change-password/")]
pub async fn change_password(
    payload: web::Json<ChangePasswordRequest>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
    let ChangePasswordRequest { code, password } = payload.into_inner();

    user_public_service::change_password(&code, &password, &state).await?;
    Ok(HttpResponse::Ok().finish())
}
