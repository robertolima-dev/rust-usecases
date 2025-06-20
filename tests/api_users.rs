use actix_web::{App, http::StatusCode, test};
use rust_usecases::models::user::UserRequest;
use rust_usecases::routes::user_public_routes::{create_user, login};

#[actix_web::test]
async fn test_create_user_success() {
    let app = test::init_service(App::new().service(create_user)).await;

    let payload = UserRequest {
        email: "teste@teste.com".to_string(),
        password: "senha123".to_string(),
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
    let app = test::init_service(App::new().service(create_user)).await;

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
    let app = test::init_service(App::new().service(create_user)).await;

    let payload = UserRequest {
        email: "test@teste.com".to_string(),
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
    let app = test::init_service(App::new().service(login)).await;

    let payload = serde_json::json!({
        "email": "naoexiste@teste.com",
        "password": "senhaerrada"
    });

    let req = test::TestRequest::post()
        .uri("/api/v1/login/")
        .set_json(&payload)
        .to_request();

    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}
