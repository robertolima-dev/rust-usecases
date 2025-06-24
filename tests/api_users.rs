use actix_web::{App, http::StatusCode, test};
use rust_usecases::models::user::UserRequest;
use rust_usecases::routes::configure::api_v1_scope;
use rust_usecases::config::app_state::AppState;
use rust_usecases::config::init_settings;
use rust_usecases::db::test_db::setup_test_db;
use rust_usecases::db::mongo::init_mongodb;
use rust_usecases::db::elasticsearch::get_elastic_client;
use rust_usecases::websocket::server::WsServer;
use std::sync::Arc;
use actix::Actor;
use std::sync::Once;

static INIT: Once = Once::new();

fn init() {
    INIT.call_once(|| {
        dotenvy::dotenv().ok();
        init_settings().expect("Falha ao inicializar settings");
    });
}

#[actix_web::test]
async fn test_create_user_success() {
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

    let payload = UserRequest {
        email: "teste@exemplo.com".to_string(),
        password: "Senha123456".to_string(),
        first_name: "Teste".to_string(),
        last_name: "User".to_string(),
        profile: None,
    };

    let req = test::TestRequest::post()
        .uri("/api/v1/users/")
        .set_json(&payload)
        .to_request();

    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_web::test]
async fn test_create_user_invalid_email() {
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

    let payload = UserRequest {
        email: "email_invalido".to_string(),
        password: "SenhaForte123".to_string(),
        first_name: "Test".to_string(),
        last_name: "User".to_string(),
        profile: None,
    };

    let req = test::TestRequest::post()
        .uri("/api/v1/users/")
        .set_json(&payload)
        .to_request();

    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

#[actix_web::test]
async fn test_create_user_password_short() {
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

    let payload = UserRequest {
        email: "test@exemplo.com".to_string(),
        password: "123".to_string(), // Senha muito curta
        first_name: "Test".to_string(),
        last_name: "User".to_string(),
        profile: None,
    };

    let req = test::TestRequest::post()
        .uri("/api/v1/users/")
        .set_json(&payload)
        .to_request();

    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

#[actix_web::test]
async fn test_login_invalid_credentials() {
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

    let payload = serde_json::json!({
        "email": "usuario_inexistente@exemplo.com",
        "password": "Senha123456"
    });

    let req = test::TestRequest::post()
        .uri("/api/v1/login/")
        .set_json(&payload)
        .to_request();

    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}
