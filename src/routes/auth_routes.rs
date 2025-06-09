use crate::models::auth::LoginRequest;
use crate::models::user::UserRequest;
use crate::services::auth_service::login_user;
use crate::services::user_service;
use actix_web::{HttpResponse, Responder, post, web};
use sqlx::PgPool;

#[post("/users/")]
pub async fn create_user(
    user_data: web::Json<UserRequest>,
    db: web::Data<PgPool>,
) -> impl Responder {
    match user_service::create_user_with_request(user_data.into_inner(), db.get_ref()).await {
        Ok(user_response) => HttpResponse::Created().json(user_response),
        Err(err) => {
            eprintln!("Erro ao criar usuário: {}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("/login/")]
pub async fn login(db: web::Data<PgPool>, payload: web::Json<LoginRequest>) -> impl Responder {
    match login_user(payload.into_inner(), db.get_ref()).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => e,
    }
}
