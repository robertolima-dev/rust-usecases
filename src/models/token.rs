use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
#[allow(dead_code)]
pub struct UserToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub code: String,
    pub token_type: String,
    pub expires_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub consumed: Option<bool>,
}
