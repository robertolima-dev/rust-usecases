use crate::models::user::UserRequest;
use crate::services::user_service;
use actix_web::{HttpResponse, Responder, post, web};
use sqlx::PgPool;

use crate::services::user_service::get_me_from_token;
use actix_web::{HttpRequest, get};

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

#[get("/me/")]
pub async fn get_me(req: HttpRequest, db: web::Data<PgPool>) -> impl Responder {
    // Extrair token do header
    let auth_header = req.headers().get("Authorization");
    let token = match auth_header.and_then(|v| v.to_str().ok()) {
        Some(header) if header.starts_with("Token ") => header[6..].to_string(),
        _ => return HttpResponse::Unauthorized().body("Token inválido"),
    };

    // Usar o service para retornar a resposta
    match get_me_from_token(&token, db.get_ref()).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(error_response) => error_response,
    }
}
