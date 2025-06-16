use crate::errors::app_error::AppError;
use crate::extensions::request_user_ext::RequestUserExt;
use crate::models::course::{CourseQuery, CreateCourseRequest, UpdateCourseRequest};
use crate::services::course_service;
use crate::websocket::server::WsServer;
use actix::Addr;
use actix_web::{HttpRequest, HttpResponse, delete, get, post, put, web};
use elasticsearch::Elasticsearch;
use sqlx::PgPool;
use uuid::Uuid;

#[post("/courses/")]
pub async fn create_course(
    req: HttpRequest,
    payload: web::Json<CreateCourseRequest>,
    db: web::Data<PgPool>,
    es: web::Data<Elasticsearch>,
    ws_server: web::Data<Addr<WsServer>>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = req.user_id()?;

    let course = course_service::create_course_service(
        payload.into_inner(),
        user_id,
        db.get_ref(),
        es.get_ref(),
        ws_server,
    )
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
    let course = course_service::update_course_and_sync(
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
    let result = course_service::search_courses(query.into_inner(), es.get_ref())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(result))
}

#[delete("/courses/{id}/")]
pub async fn delete_course(
    path: web::Path<Uuid>,
    db: web::Data<PgPool>,
    es: web::Data<Elasticsearch>,
) -> Result<HttpResponse, AppError> {
    let course_id = path.into_inner();

    course_service::delete_course(db.get_ref(), es.get_ref(), course_id).await?;

    Ok(HttpResponse::NoContent().finish())
}
