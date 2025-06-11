use crate::logs::model::LogQuery;
use crate::logs::service;
use actix_web::{HttpResponse, Responder, get, web};
use mongodb::Database;

#[get("/logs/")]
pub async fn get_logs(db: web::Data<Database>, query: web::Query<LogQuery>) -> impl Responder {
    match service::list_logs(&db, &query).await {
        Ok(logs) => HttpResponse::Ok().json(logs),
        Err(err) => {
            eprintln!("Erro ao buscar logs: {:?}", err);
            HttpResponse::InternalServerError().body("Erro ao buscar logs")
        }
    }
}
