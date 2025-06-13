// src/routes/course_routes.rs
use crate::errors::app_error::AppError;
use crate::extensions::request_user_ext::RequestUserExt;
use crate::models::course::{CourseQuery, CreateCourseRequest, UpdateCourseRequest};
use crate::services::course_service::{
    create_course_service, search_courses, update_course_and_sync,
};
use actix_web::{HttpRequest, HttpResponse, get, post, put, web};
use elasticsearch::Elasticsearch;
use sqlx::PgPool;
use uuid::Uuid;

#[post("/courses/")]
pub async fn create_course(
    req: HttpRequest,
    payload: web::Json<CreateCourseRequest>,
    db: web::Data<PgPool>,
    es: web::Data<Elasticsearch>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = req.user_id()?;

    let course = create_course_service(payload.into_inner(), user_id, db.get_ref(), es.get_ref())
        .await
        .map_err(|e| {
            eprintln!("Erro ao criar curso: {:?}", e);
            actix_web::error::ErrorInternalServerError("Erro ao criar curso")
        })?;

    Ok(HttpResponse::Created().json(course))
}

#[put("/courses/{id}/")]
pub async fn update_course(
    req: HttpRequest,
    path: web::Path<Uuid>,
    payload: web::Json<UpdateCourseRequest>,
    db: web::Data<PgPool>,
    es: web::Data<Elasticsearch>,
) -> Result<HttpResponse, AppError> {
    let user_id = req.user_id()?;
    let id = path.into_inner();
    let course = update_course_and_sync(
        id,
        payload.into_inner(),
        user_id,
        db.get_ref(),
        es.get_ref(),
    )
    .await?;

    Ok(HttpResponse::Ok().json(course))
}

#[get("/courses/")]
pub async fn list_courses(
    query: web::Query<CourseQuery>,
    es: web::Data<Elasticsearch>,
) -> Result<HttpResponse, actix_web::Error> {
    let result = search_courses(query.into_inner(), es.get_ref())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(result))
}
