use crate::config::app_state::AppState;
use crate::extensions::request_user_ext::RequestUserExt;
use crate::services::notification_service;
use actix_web::ResponseError;
use actix_web::{HttpRequest, HttpResponse, Responder, get, web};

#[get("/notifications/")]
pub async fn list_notifications(req: HttpRequest, state: web::Data<AppState>) -> impl Responder {
    let user_id = match req.user_id() {
        Ok(id) => id,
        Err(err) => return err.error_response(),
    };

    match notification_service::get_user_notifications(user_id, &state).await {
        Ok(notifications) => HttpResponse::Ok().json(notifications),
        Err(_) => HttpResponse::InternalServerError().body("Erro ao buscar notificações"),
    }
}
