use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Category {
    pub id: Uuid,
    pub name: String,
    pub dt_created: NaiveDateTime,
    pub dt_updated: Option<NaiveDateTime>,
    #[serde(skip_serializing)]
    pub dt_deleted: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCategoryRequest {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCategoryRequest {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CategorySimple {
    pub id: Uuid,
    pub name: String,
}
