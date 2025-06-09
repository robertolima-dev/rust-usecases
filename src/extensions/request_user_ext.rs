use crate::errors::app_error::AppError;
use crate::middleware::auth_middleware::Claims;
use actix_web::HttpMessage;
use actix_web::HttpRequest;
use uuid::Uuid;

pub trait RequestUserExt {
    fn user_id(&self) -> Result<Uuid, AppError>;
}

impl RequestUserExt for HttpRequest {
    fn user_id(&self) -> Result<Uuid, AppError> {
        let extensions = self.extensions();
        let claims = extensions
            .get::<Claims>()
            .ok_or(AppError::Unauthorized(None))?;

        Uuid::parse_str(&claims.sub).map_err(|_| AppError::Unauthorized(None))
    }
}
