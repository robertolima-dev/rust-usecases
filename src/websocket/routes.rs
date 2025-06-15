use actix_web::{Error, HttpRequest, HttpResponse, get, web};
use actix_web_actors::ws;
use uuid::Uuid;

use super::session::WsSession;
use crate::models::auth::Claims;
use crate::utils::jwt::decode_token;

#[get("/ws/")]
pub async fn websocket_entry(
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    // Extrair o token da URL: ?token=xxx
    let query_string = req.query_string();
    let token_opt =
        web::Query::<std::collections::HashMap<String, String>>::from_query(query_string)
            .ok()
            .and_then(|q| q.get("token").cloned());

    let token = match token_opt {
        Some(t) => t,
        None => {
            return Ok(HttpResponse::Unauthorized().body("Token ausente"));
        }
    };

    // Decodificar token
    let claims: Claims = match decode_token(&token) {
        Ok(c) => c,
        Err(_) => return Ok(HttpResponse::Unauthorized().body("Token inválido")),
    };

    let user_id = match Uuid::parse_str(&claims.sub) {
        Ok(u) => u,
        Err(_) => return Ok(HttpResponse::Unauthorized().body("ID inválido no token")),
    };

    ws::start(WsSession { user_id }, &req, stream)
}
