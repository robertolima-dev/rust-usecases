use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(sqlx::Type, Debug, Serialize, Deserialize, Clone)]
#[sqlx(type_name = "obj_code_type")]
pub enum ObjCodeType {
    #[sqlx(rename = "platform")]
    Platform,
    #[sqlx(rename = "user")]
    User,
}

#[derive(Debug, Deserialize)]
pub struct NotificationQuery {
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Notification {
    pub id: Uuid,
    pub title: String,
    pub message: String,
    pub obj_code: ObjCodeType,
    pub obj_id: Option<Uuid>,
    pub created_at: NaiveDateTime,
}
