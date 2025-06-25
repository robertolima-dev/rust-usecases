use crate::logs::routes::get_logs;
use crate::middleware::auth_middleware::AuthMiddleware;

use crate::routes::{
    category_routes, course_routes, notification_routes, profile_routes, user_private_routes,
    user_public_routes,
};
use crate::websocket::routes::websocket_entry;
use actix_web::{Scope, web};

pub fn api_v1_scope() -> Scope {
    web::scope("/api/v1")
        .service(user_public_routes::login)
        .service(user_public_routes::create_user)
        .service(user_public_routes::confirm_email)
        .service(user_public_routes::forgot_password)
        .service(user_public_routes::change_password)
        .service(websocket_entry)
        .service(
            web::scope("") // escopo vazio herda o "/api/v1"
                .wrap(AuthMiddleware)
                .service(get_logs)
                .service(user_private_routes::get_me)
                .service(user_private_routes::list_users)
                .service(user_private_routes::update_user)
                .service(user_private_routes::delete_user)
                .service(profile_routes::update_profile)
                .service(course_routes::create_course)
                .service(course_routes::list_courses)
                .service(course_routes::update_course)
                .service(course_routes::sync_courses_to_elasticsearch)
                .service(notification_routes::list_notifications)
                .service(category_routes::create_category)
                .service(category_routes::list_categories)
                .service(category_routes::update_category)
                .service(category_routes::delete_category)
                .service(course_routes::delete_course),
        )
}
