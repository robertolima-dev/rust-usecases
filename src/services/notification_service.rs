use crate::config::app_state::AppState;
use crate::errors::app_error::AppError;
use crate::models::notification::Notification;
use crate::models::notification::NotificationQuery;
use crate::models::notification::ObjCodeType;
use crate::repositories::notification_repository;
use actix_web::web;
use anyhow::Result;
use uuid::Uuid;

pub async fn get_user_notifications(
    user_id: Uuid,
    query: NotificationQuery,
    state: &AppState,
) -> Result<Vec<Notification>, AppError> {
    let db = &state.db;
    let offset = query.offset.unwrap_or(0);
    let limit = query.limit.unwrap_or(10);

    notification_repository::list_notifications_for_user_or_platform(
        user_id,
        offset as i64,
        limit as i64,
        db,
    )
    .await
    .map_err(|err| {
        eprintln!("Erro ao buscar notificações no banco: {:?}", err);
        AppError::DatabaseError(Some(format!("Erro ao listar notificações: {}", err)))
    })
}

#[allow(dead_code)]
pub async fn create_notification_and_emit(
    title: &str,
    message: &str,
    obj_code: ObjCodeType,
    obj_id: Option<Uuid>,
    state: &web::Data<AppState>,
) -> Result<Notification> {
    let db = &state.db;
    let ws_server = &state.ws_server;

    // 1. Criar a notificação no banco
    let notification =
        notification_repository::create_notification(title, message, obj_code.clone(), obj_id, db)
            .await
            .map_err(|e| anyhow::anyhow!(e))?;

    // 2. Disparar via WebSocket 🎉
    let ws_message = serde_json::to_string(&notification).unwrap();

    match obj_code {
        ObjCodeType::Platform => {
            ws_server.do_send(crate::websocket::server::BroadcastMessage(ws_message));
        }
        ObjCodeType::User => {
            if let Some(user_id) = obj_id {
                ws_server.do_send(crate::websocket::server::UserMessage {
                    user_id,
                    message: ws_message,
                });
            }
        }
    }

    Ok(notification)
}
