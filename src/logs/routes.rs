use crate::config::app_state::AppState;
use crate::logs::model::LogQuery;
use crate::logs::service;
use actix_web::{HttpResponse, Responder, get, web};

#[get("/logs/")]
pub async fn get_logs(query: web::Query<LogQuery>, state: web::Data<AppState>) -> impl Responder {
    match service::list_logs(&query, &state).await {
        Ok(logs) => HttpResponse::Ok().json(logs),
        Err(err) => {
            eprintln!("Erro ao buscar logs: {:?}", err);
            HttpResponse::InternalServerError().body("Erro ao buscar logs")
        }
    }
}
