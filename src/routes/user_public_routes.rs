use crate::errors::app_error::AppError;
use crate::models::auth::LoginRequest;
use crate::models::user::UserRequest;
use crate::services::user_public_service;
use actix_web::{HttpResponse, Responder, post, get, web};
use mongodb::Database;
use sqlx::PgPool;

#[post("/users/")]
pub async fn create_user(
    user_data: web::Json<UserRequest>,
    db: web::Data<PgPool>,
    mongo_db: web::Data<Database>,
) -> Result<HttpResponse, AppError> {
    let res = user_public_service::create_user_with_request(
        user_data.into_inner(),
        db.get_ref(),
        mongo_db.get_ref(),
    )
    .await?;
    Ok(HttpResponse::Ok().json(res))
}

#[post("/login/")]
pub async fn login(
    db: web::Data<PgPool>,
    mongo_db: web::Data<Database>,
    payload: web::Json<LoginRequest>,
) -> Result<impl Responder, AppError> {
    let response =
        user_public_service::login_user(payload.into_inner(), db.get_ref(), mongo_db.get_ref())
            .await?;
    Ok(web::Json(response))
}

#[get("/confirm-email/{code}/")]
pub async fn confirm_email(
    code: web::Path<String>,
    db: web::Data<PgPool>,
    mongo_db: web::Data<Database>,
) -> Result<HttpResponse, AppError> {
    user_public_service::confirm_email(&code, db.get_ref(), mongo_db.get_ref()).await?;
    Ok(HttpResponse::Ok().finish())
}
