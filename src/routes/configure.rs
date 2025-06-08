use crate::routes::user_routes::{create_user, get_user_by_id, get_users};
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_users)
        .service(get_user_by_id)
        .service(create_user);
}
