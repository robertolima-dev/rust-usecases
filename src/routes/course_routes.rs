use crate::config::app_state::AppState;
use crate::errors::app_error::AppError;
use crate::extensions::request_user_ext::RequestUserExt;
use crate::models::course::{CourseQuery, CreateCourseRequest, UpdateCourseRequest};
use crate::services::course_service;
use actix_web::{HttpRequest, HttpResponse, Responder, delete, get, post, put, web};
use serde_json::json;
use uuid::Uuid;

#[post("/courses/")]
pub async fn create_course(
    req: HttpRequest,
    payload: web::Json<CreateCourseRequest>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, actix_web::Error> {
    let access_level = req.access_level()?;
    if access_level != "admin" {
        return Ok(HttpResponse::Forbidden().body("Permissão negada"));
    }

    let user_id = req.user_id()?;

    let course = course_service::create_course_service(payload.into_inner(), user_id, &state)
        .await
        .map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!("Erro ao criar curso: {}", e))
        })?;

    Ok(HttpResponse::Created().json(course))
}

#[put("/courses/{id}/")]
pub async fn update_course(
    req: HttpRequest,
    path: web::Path<Uuid>,
    payload: web::Json<UpdateCourseRequest>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
    let access_level = req.access_level()?;
    if access_level != "admin" {
        return Ok(HttpResponse::Forbidden().body("Permissão negada"));
    }

    let user_id = req.user_id()?;
    let id = path.into_inner();
    let course =
        course_service::update_course_and_sync(id, payload.into_inner(), user_id, &state).await?;
    Ok(HttpResponse::Ok().json(course))
}

#[get("/courses/")]
pub async fn list_courses(
    query: web::Query<CourseQuery>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, actix_web::Error> {
    let result = course_service::search_courses(query.into_inner(), &state)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(result))
}

#[delete("/courses/{id}/")]
pub async fn delete_course(
    path: web::Path<Uuid>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
    let course_id = path.into_inner();

    course_service::delete_course(course_id, &state).await?;

    Ok(HttpResponse::NoContent().finish())
}

#[post("/courses/sync/")]
async fn sync_courses_to_elasticsearch(
    req: HttpRequest,
    state: web::Data<AppState>,
) -> Result<impl Responder, AppError> {
    let access_level = req.access_level()?;
    if access_level != "admin" {
        return Ok(HttpResponse::Forbidden().body("Permissão negada"));
    }

    let indexed = course_service::reindex_courses(&state)
        .await
        .map_err(|e| AppError::InternalError(Some(format!("Erro ao reindexar cursos: {}", e))))?;

    Ok(HttpResponse::Ok().json(json!({
        "message": format!("{} cursos sincronizados com sucesso.", indexed)
    })))
}
