use crate::errors::app_error::AppError;
use crate::models::user::UserRequest;
use crate::services::user_service;
use crate::services::user_service::get_me_from_token;
use actix_web::{HttpRequest, get};
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

#[get("/me/")]
pub async fn get_me(req: HttpRequest, db: web::Data<PgPool>) -> Result<HttpResponse, AppError> {
    // 1. Extrair token do cabeçalho
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.strip_prefix("Token "))
        .ok_or(AppError::Unauthorized(None))?;

    // 2. Chamar service com token
    let response = get_me_from_token(token, db.get_ref()).await?;

    // 3. Resposta HTTP final
    Ok(HttpResponse::Ok().json(response))
}
