use crate::config::get_settings;
use actix_web::body::BoxBody;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready};
use actix_web::{Error, HttpMessage, HttpResponse};
use futures::future::{LocalBoxFuture, Ready, ok};
use jsonwebtoken::{DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use tracing::{error, info, warn};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub struct AuthMiddleware;

impl<S> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error> + 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddlewareService {
            service: Rc::new(service),
        })
    }
}

pub struct AuthMiddlewareService<S> {
    service: Rc<S>,
}

impl<S> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error> + 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let auth_header = req
            .headers()
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .unwrap_or("");

        let token = auth_header.strip_prefix("Token ").unwrap_or("");

        if token.is_empty() {
            warn!("Tentativa de acesso sem token");
            let response = HttpResponse::Unauthorized()
                .body("Token não fornecido")
                .map_into_boxed_body();
            return Box::pin(async { Ok(req.into_response(response)) });
        }

        let settings = get_settings();
        let jwt_secret = &settings.jwt.secret;

        match decode::<Claims>(
            token,
            &DecodingKey::from_secret(jwt_secret.as_bytes()),
            &Validation::default(),
        ) {
            Ok(token_data) => {
                info!(
                    user_id = %token_data.claims.sub,
                    "Token válido"
                );
                req.extensions_mut().insert(token_data.claims);
                let fut = self.service.call(req);
                Box::pin(async move { fut.await })
            }
            Err(err) => {
                error!(
                    error = %err,
                    "Token inválido"
                );
                let response = HttpResponse::Unauthorized()
                    .body("Token inválido")
                    .map_into_boxed_body();
                Box::pin(async { Ok(req.into_response(response)) })
            }
        }
    }
}
