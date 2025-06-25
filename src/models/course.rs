use crate::models::category::CategorySimple;
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Course {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub is_active: bool,
    pub price: f64,
    pub month_duration: i32,
    pub author_id: Uuid,
    pub dt_start: NaiveDate,
    pub dt_created: NaiveDateTime,
    pub dt_updated: NaiveDateTime,
    #[serde(skip_serializing)]
    pub dt_deleted: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateCourseRequest {
    pub name: String,
    pub description: Option<String>,
    pub price: f64,
    pub month_duration: i32,
    pub dt_start: NaiveDate,
    pub category_ids: Option<Vec<Uuid>>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCourseRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub price: Option<f64>,
    pub month_duration: Option<i32>,
    pub dt_start: Option<NaiveDate>,
    pub is_active: Option<bool>,
    pub category_ids: Option<Vec<Uuid>>,
}

#[derive(Debug, Deserialize)]
pub struct CourseQuery {
    pub author_id: Option<Uuid>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
    pub name: Option<String>,
    pub is_active: Option<bool>,
    pub min_price: Option<f64>,
    pub max_price: Option<f64>,
    pub start_from: Option<NaiveDate>,
    pub month_duration: Option<i32>,
    pub category_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CourseSearchHit {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub is_active: bool,
    pub price: f64,
    pub month_duration: i32,
    pub author_id: Uuid,
    pub dt_start: NaiveDate,
    pub categories: Option<Vec<CategorySimple>>,
}

#[derive(Debug, Serialize)]
pub struct PaginatedCourseResponse {
    pub results: Vec<CourseSearchHit>,
    pub count: usize,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct CourseCategory {
    pub id: Uuid,
    pub course_id: Uuid,
    pub category_id: Uuid,
}
