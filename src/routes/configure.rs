use crate::middleware::auth_middleware::AuthMiddleware;
use crate::routes::user_private_routes::get_me;
use crate::routes::user_public_routes::{create_user, login};
use actix_web::{Scope, web};

pub fn api_v1_scope() -> Scope {
    web::scope("/api/v1")
        .service(login)
        .service(create_user)
        .service(
            web::scope("") // escopo vazio herda o "/api/v1"
                .wrap(AuthMiddleware)
                .service(get_me),
        )
}
