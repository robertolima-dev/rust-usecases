use crate::models::auth::LoginRequest;
use crate::services::auth_service::login_user;
use actix_web::{HttpResponse, Responder, post, web};
use sqlx::PgPool;

#[post("/login/")]
pub async fn login(db: web::Data<PgPool>, payload: web::Json<LoginRequest>) -> impl Responder {
    match login_user(payload.into_inner(), db.get_ref()).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => e,
    }
}
