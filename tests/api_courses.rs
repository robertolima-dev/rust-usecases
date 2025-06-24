use actix::Actor;
use actix_web::{App, http::StatusCode, test};
use rust_usecases::config::{app_state::AppState, init_settings};
use rust_usecases::db::elasticsearch::get_elastic_client;
use rust_usecases::db::mongo::init_mongodb;
use rust_usecases::db::test_db::setup_test_db;
use rust_usecases::routes::configure::api_v1_scope;
use rust_usecases::websocket::server::WsServer;
use std::sync::Arc;
use std::sync::Once;

static INIT: Once = Once::new();

fn init() {
    INIT.call_once(|| {
        dotenvy::dotenv().ok();
        init_settings().expect("Falha ao inicializar settings");
    });
}

#[actix_web::test]
async fn test_create_course_unauthenticated() {
    init();

    let db = setup_test_db().await;
    let mongo = init_mongodb().await.unwrap();
    let es = get_elastic_client().unwrap();
    let ws_server = WsServer::new().start();

    let app_state = Arc::new(AppState {
        db,
        mongo,
        es,
        ws_server,
    });

    let app = test::init_service(
        App::new()
            .app_data(actix_web::web::Data::from(app_state.clone()))
            .service(api_v1_scope()),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/v1/courses/")
        .to_request();

    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}
