use crate::models::user_request::UserRequest;
use actix_web::{HttpResponse, Responder, post, web};
// use actix_web::{HttpResponse, Responder, post, web};
use crate::services::user_service;
use sqlx::PgPool;

#[post("/api/v1/users/")]
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
