use crate::models::notification::Notification;
use crate::repositories::notification_repository;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn get_user_notifications(
    user_id: Uuid,
    db: &PgPool,
) -> Result<Vec<Notification>, sqlx::Error> {
    notification_repository::list_notifications_for_user_or_platform(user_id, db).await
}
