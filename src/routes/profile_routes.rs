use crate::config::app_state::AppState;
use crate::errors::app_error::AppError;
use crate::extensions::has_any_field::HasAnyField;
use crate::extensions::request_user_ext::RequestUserExt;
use crate::models::profile::UpdateProfileRequest;
use crate::services::profile_service;
use actix_web::{HttpRequest, HttpResponse, Responder, post, web};

#[post("/profiles/")]
pub async fn update_profile(
    req: HttpRequest,
    payload: web::Json<UpdateProfileRequest>,
    state: web::Data<AppState>,
) -> Result<impl Responder, AppError> {
    if !payload.has_any_field() {
        return Err(AppError::BadRequest(Some(
            "Nenhum campo informado para atualização".into(),
        )));
    }
    let user_id = req.user_id()?; // 🔐 Garantido pelo middleware
    let profile =
        profile_service::update_profile_by_user_id(user_id, payload.into_inner(), &state).await?;
    Ok(HttpResponse::Ok().json(profile))
}
