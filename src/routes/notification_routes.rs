use crate::extensions::request_user_ext::RequestUserExt;
use crate::services::notification_service;
use actix_web::ResponseError;
use actix_web::{HttpRequest, HttpResponse, Responder, get, web};
use sqlx::PgPool;

#[get("/notifications/")]
pub async fn list_notifications(req: HttpRequest, db: web::Data<PgPool>) -> impl Responder {
    let user_id = match req.user_id() {
        Ok(id) => id,
        Err(err) => return err.error_response(),
    };

    match notification_service::get_user_notifications(user_id, db.get_ref()).await {
        Ok(notifications) => HttpResponse::Ok().json(notifications),
        Err(_) => HttpResponse::InternalServerError().body("Erro ao buscar notificações"),
    }
}
