use crate::config::app_state::AppState;
use crate::errors::app_error::AppError;
use crate::extensions::request_user_ext::RequestUserExt;
use crate::models::notification::NotificationQuery;
use crate::services::notification_service;
use actix_web::{HttpRequest, HttpResponse, get, web};

#[get("/notifications/")]
pub async fn list_notifications(
    req: HttpRequest,
    query: web::Query<NotificationQuery>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
    let user_id = req.user_id()?; // Se der erro, o AppError trata

    let notifications =
        notification_service::get_user_notifications(user_id, query.into_inner(), &state).await?;

    Ok(HttpResponse::Ok().json(notifications))
}
