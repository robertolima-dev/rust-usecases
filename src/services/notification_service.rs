use crate::config::app_state::AppState;
use crate::errors::app_error::AppError;
use crate::log_fail;
use crate::logs::model::LogLevel;
use crate::models::notification::Notification;
use crate::models::notification::NotificationQuery;
use crate::models::notification::ObjCodeType;
use crate::repositories::notification_repository;
use crate::utils::pagination::PaginatedResponse;
use actix_web::web;
use anyhow::Result;
use uuid::Uuid;

pub async fn get_user_notifications(
    user_id: Uuid,
    query: NotificationQuery,
    state: &AppState,
) -> Result<PaginatedResponse<Notification>, AppError> {
    let db = &state.db;
    let mongo_db = &state.mongo;
    let offset = query.offset.unwrap_or(0);
    let limit = query.limit.unwrap_or(10);

    let count = notification_repository::count_all_notifications(user_id, db)
        .await
        .map_err(|_| AppError::BadRequest(Some("Erro ao contar notificações".into())))?;

    let notifications = match notification_repository::list_notifications_for_user_or_platform(
        user_id,
        offset as i64,
        limit as i64,
        db,
    )
    .await
    {
        Ok(p) => p,
        Err(err) => {
            log_fail!(
                err,
                LogLevel::Error,
                "Erro ao listar users",
                "user_service",
                Some(user_id),
                mongo_db
            );
            return Err(AppError::DatabaseError(Some(format!(
                "Erro ao listar notificações: {}",
                err
            ))));
        }
    };

    Ok(PaginatedResponse {
        count,
        results: notifications,
        limit: limit as i64,
        offset: offset as i64,
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
