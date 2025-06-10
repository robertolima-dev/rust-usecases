// src/routes/profile_routes.rs
use crate::errors::app_error::AppError;
use crate::extensions::has_any_field::HasAnyField;
use crate::extensions::request_user_ext::RequestUserExt;
use crate::models::profile::UpdateProfileRequest;
use crate::services::profile_service;
use actix_web::{HttpRequest, HttpResponse, Responder, post, web};
use sqlx::PgPool;

#[post("/profiles/")]
pub async fn update_profile(
    req: HttpRequest,
    db: web::Data<PgPool>,
    payload: web::Json<UpdateProfileRequest>,
) -> Result<impl Responder, AppError> {
    if !payload.has_any_field() {
        return Err(AppError::BadRequest(Some(
            "Nenhum campo informado para atualização".into(),
        )));
    }
    let user_id = req.user_id()?; // 🔐 Garantido pelo middleware
    let profile =
        profile_service::update_profile_by_user_id(user_id, payload.into_inner(), db.get_ref())
            .await?;
    Ok(HttpResponse::Ok().json(profile))
}
