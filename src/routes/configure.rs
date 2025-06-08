use crate::routes::user_routes::create_user;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(create_user);
    // .service(get_users)
    // .service(get_user_by_id);
}
