use crate::logs::routes::get_logs;
use crate::middleware::auth_middleware::AuthMiddleware;
use crate::routes::profile_routes::update_profile;
use crate::routes::user_private_routes::{delete_user, get_me, list_users, update_user};
use crate::routes::user_public_routes::{create_user, login, confirm_email};
use actix_web::{Scope, web};

pub fn api_v1_scope() -> Scope {
    web::scope("/api/v1")
        .service(login)
        .service(create_user)
        .service(confirm_email)
        .service(
            web::scope("") // escopo vazio herda o "/api/v1"
                .wrap(AuthMiddleware)
                .service(get_logs)
                .service(get_me)
                .service(list_users)
                .service(update_user)
                .service(delete_user)
                .service(update_profile),
        )
}
