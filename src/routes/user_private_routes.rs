use crate::errors::app_error::AppError;
use crate::extensions::request_user_ext::RequestUserExt;
use crate::services::user_private_service::get_me_by_user_id;
use actix_web::{HttpRequest, HttpResponse, get, web};
use sqlx::PgPool;

#[get("/me/")]
pub async fn get_me(req: HttpRequest, db: web::Data<PgPool>) -> Result<HttpResponse, AppError> {
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.strip_prefix("Token "))
        .map(|s| s.to_string())
        .ok_or(AppError::Unauthorized(None))?;
    let user_id = req.user_id()?;
    println!("token: {:?}", token);

    let response = get_me_by_user_id(user_id, db.get_ref(), token).await?;
    Ok(HttpResponse::Ok().json(response))
}
