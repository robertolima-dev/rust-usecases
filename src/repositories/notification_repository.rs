use crate::errors::app_error::AppError;
use crate::models::notification::{Notification, ObjCodeType};
use sqlx::{Error, PgPool};
use uuid::Uuid;

pub async fn create_notification(
    title: &str,
    message: &str,
    obj_code: ObjCodeType,
    obj_id: Option<Uuid>,
    db: &PgPool,
) -> Result<(), AppError> {
    let id = Uuid::new_v4();
    sqlx::query!(
        r#"
        INSERT INTO notifications (id, title, message, obj_code, obj_id, created_at)
        VALUES ($1, $2, $3, $4::obj_code_type, $5, NOW())
        "#,
        id,
        title,
        message,
        obj_code as ObjCodeType,
        obj_id
    )
    .execute(db)
    .await
    .map_err(|err| {
        eprintln!("Erro ao criar notificação: {:?}", err);
        AppError::InternalError(Some("Falha ao criar notificação".into()))
    })?;

    Ok(())
}

pub async fn list_notifications_for_user_or_platform(
    user_id: Uuid,
    db: &PgPool,
) -> Result<Vec<Notification>, Error> {
    let notifications = sqlx::query_as_unchecked!(
        Notification,
        r#"
        SELECT id, title, message, obj_code::TEXT, obj_id, created_at
        FROM notifications
        WHERE obj_code = 'platform' OR (obj_code = 'user' AND obj_id = $1)
        ORDER BY created_at DESC
        "#,
        user_id
    )
    .fetch_all(db)
    .await?;

    Ok(notifications)
}
