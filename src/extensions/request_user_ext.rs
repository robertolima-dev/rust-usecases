use crate::errors::app_error::AppError;
use crate::models::auth::Claims;
use actix_web::HttpMessage;
use actix_web::HttpRequest;
use uuid::Uuid;

pub trait RequestUserExt {
    fn user_id(&self) -> Result<Uuid, AppError>;
    fn access_level(&self) -> Result<String, AppError>;
}

impl RequestUserExt for HttpRequest {
    fn user_id(&self) -> Result<Uuid, AppError> {
        let extensions = self.extensions();
        let claims = extensions
            .get::<Claims>()
            .ok_or(AppError::Unauthorized(None))?;

        Uuid::parse_str(&claims.sub).map_err(|_| AppError::Unauthorized(None))
    }
    fn access_level(&self) -> Result<String, AppError> {
        self.extensions()
            .get::<Claims>()
            .map(|claims| claims.access_level.clone())
            .ok_or(AppError::Unauthorized(Some("Token inválido".into())))
    }
}
