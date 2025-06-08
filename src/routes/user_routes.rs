use crate::models::user::User;
use crate::services::user_service;
use actix_web::{HttpResponse, Responder, get, post, web};
use sqlx::PgPool;
use uuid::Uuid;

#[get("/api/v1/users/")]
pub async fn get_users(db: web::Data<PgPool>) -> impl Responder {
    match user_service::get_all_users(db.get_ref()).await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(err) => {
            eprintln!("Erro ao buscar usuários: {}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("/api/v1/users/")]
pub async fn create_user(user_data: web::Json<User>, db: web::Data<PgPool>) -> impl Responder {
    match user_service::create_user(user_data.into_inner(), db.get_ref()).await {
        Ok(_) => HttpResponse::Created().json("Usuário criado com sucesso"),
        Err(err) => {
            eprintln!("Erro ao criar usuário: {}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/api/v1/users/{id}/")]
pub async fn get_user_by_id(path: web::Path<Uuid>, db: web::Data<PgPool>) -> impl Responder {
    let id = path.into_inner();

    match user_service::get_user_by_id(id, &db).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => {
            eprintln!("❌ Erro ao buscar usuário: {}", err);
            HttpResponse::NotFound().body("Usuário não encontrado")
        }
    }
}
