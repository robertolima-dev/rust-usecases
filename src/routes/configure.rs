// use crate::middleware::auth_middleware::AuthMiddleware;
// use crate::routes::auth_routes::login;
// use crate::routes::user_routes::{create_user, get_me};
// use actix_web::web;

// pub fn config(cfg: &mut web::ServiceConfig) {
//     cfg
//         // Rotas públicas
//         .service(login)
//         .service(create_user)
//         // Rotas protegidas
//         .service(web::scope("/api/v1").wrap(AuthMiddleware).service(get_me));
// }

use crate::middleware::auth_middleware::AuthMiddleware;
use crate::routes::auth_routes::login;
use crate::routes::user_routes::{create_user, get_me};
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
