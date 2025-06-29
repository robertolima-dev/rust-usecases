use crate::config::app_state::AppState;
use crate::config::get_settings;
use crate::errors::app_error::AppError;
use crate::log_fail;
use crate::logs::model::LogLevel;
use crate::models::auth::LoginRequest;
use crate::models::{
    profile::Profile,
    user::{User, UserRequest, UserResponse, UserWithProfile},
};
use crate::repositories::{profile_repository, token_repository, user_repository};
#[allow(unused_imports)]
use crate::services::email_service::EmailService;
use crate::services::token_service;
use crate::utils::formatter;
use crate::utils::jwt::generate_jwt;
use actix_web::web;
use chrono::Utc;
use tracing::{error, info, warn};
use uuid::Uuid;
use validator::Validate;
use sqlx::PgPool;
use std::env;

/// Cria user + profile com base em UserRequest
pub async fn create_user_with_request(
    req: UserRequest,
    state: &web::Data<AppState>,
) -> Result<UserResponse, AppError> {
    info!(email = %req.email, "Criando novo usuário");

    // Validar os dados de entrada
    if let Err(e) = req.validate() {
        warn!(error = %e, "Dados inválidos na criação de usuário");
        println!("[DEBUG][create_user_with_request] Erro de validação: {e}");
        return Err(AppError::BadRequest(Some(format!("Dados inválidos: {}", e))));
    }

    // Bloco para capturar qualquer erro inesperado
    match create_user_with_request_inner(req, state).await {
        Ok(res) => Ok(res),
        Err(e) => {
            println!("[DEBUG][create_user_with_request] Erro inesperado: {e:?}");
            Err(e)
        }
    }
}

async fn create_user_with_request_inner(
    req: UserRequest,
    state: &web::Data<AppState>,
) -> Result<UserResponse, AppError> {
    let db = &state.db;
    let mongo_db = &state.mongo;

    let user_id = Uuid::new_v4();
    let now = Utc::now().naive_utc();

    let user = User {
        id: user_id,
        username: formatter::generate_username_from_email(&req.email),
        email: req.email.clone(),
        first_name: req.first_name,
        last_name: req.last_name,
        password: bcrypt::hash(&req.password, bcrypt::DEFAULT_COST).expect("Erro ao hashear senha"),
        dt_created: now,
        dt_updated: now,
        dt_deleted: None,
    };

    let profile = Profile::from_request(user_id, req.profile);

    match create_user_and_profile(&user, &profile, &state).await {
        Ok(p) => p,
        Err(err) => {
            log_fail!(
                err,
                LogLevel::Error,
                format!("Erro ao criar o usuário: {}", &req.email),
                "user_service",
                Some(user_id),
                mongo_db
            );
            return Err(AppError::BadRequest(Some("Erro ao criar o usuario".into())));
        }
    };

    let token_type: &str = "confirm_email";
    let confirm_email_code = token_service::create_user_token(user_id, token_type, db).await?;
    println!("confirm_email_code: {}", confirm_email_code);

    let user_with_profile = UserWithProfile::from_user_and_profile(user, profile);

    let settings = get_settings();
    let token = generate_jwt(&user_id.to_string(), "user").expect("Falha ao gerar token");
    let expires_in = settings.jwt.expires_in.to_string();

    info!(
        user_id = %user_id,
        "Usuário criado com sucesso"
    );

    Ok(UserResponse::from(user_with_profile, token, expires_in))
}

pub async fn create_user_and_profile(
    user: &User,
    profile: &Profile,
    state: &web::Data<AppState>,
) -> Result<(), AppError> {
    let db = &state.db;

    let mut tx = db.begin().await.map_err(|err| {
        error!(
            error = %err,
            "Erro ao iniciar transação"
        );
        AppError::DatabaseError(Some(format!("Erro ao iniciar transação: {}", err)))
    })?;

    user_repository::create_user_in_tx(&user, &mut tx).await?;
    profile_repository::create_profile_in_tx(&profile, &mut tx).await?;

    tx.commit().await.map_err(|err| {
        error!(
            error = %err,
            "Erro ao commitar transação"
        );
        AppError::DatabaseError(Some(format!("Erro ao commitar transação: {}", err)))
    })?;

    Ok(())
}

pub async fn login_user(
    payload: LoginRequest,
    state: &web::Data<AppState>,
) -> Result<UserResponse, AppError> {
    let db = &state.db;
    let mongo_db = &state.mongo;

    info!(
        email = %payload.email,
        "Tentativa de login"
    );

    // Validar os dados de entrada
    if let Err(e) = payload.validate() {
        warn!(error = %e, "Dados inválidos no login");
        println!("[DEBUG][login_user] Erro de validação: {e}");
        return Err(AppError::BadRequest(Some(format!("Dados inválidos: {}", e))));
    }

    // 1. Buscar usuário por email
    let user = match user_repository::find_user_by_email(&payload.email, db).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            warn!(email = %payload.email, "Usuário não encontrado");
            log_fail!(
                "Usuário não encontrado",
                LogLevel::Error,
                "Usuário não encontrado",
                "auth_service",
                None,
                mongo_db
            );
            return Err(AppError::Unauthorized(Some("Credenciais inválidas".into())));
        }
        Err(err) => {
            error!(error = %err, "Erro ao buscar usuário");
            log_fail!(
                err,
                LogLevel::Error,
                "Usuário não encontrado",
                "auth_service",
                None,
                mongo_db
            );
            return Err(AppError::BadRequest(Some("Erro ao buscar usuário".into())));
        }
    };

    // 2. Verificar senha
    if !user.verify_password(&payload.password) {
        warn!(
            email = %payload.email,
            "Senha incorreta"
        );
        log_fail!(
            format!("Senha incorreta para o email: {}", payload.email),
            LogLevel::Warn,
            "Erro ao verificar a senha!",
            "auth_service",
            Some(user.id), // ou None se não tiver ainda o user_id
            mongo_db
        );
        return Err(AppError::Unauthorized(Some("❌ Senha incorreta".into())));
    }

    // 3. Buscar o perfil
    let profile = profile_repository::find_profile_by_user_id(user.id, db)
        .await
        .map_err(|err| {
            error!(
                error = %err,
                "Erro ao buscar perfil"
            );
            AppError::BadRequest(Some("Erro ao buscar perfil".into()))
        })?;

    // testar a falha do log e ver lista de log no mongo
    // let fake_user_id = Uuid::new_v4();
    // let token = generate_jwt(&fake_user_id.to_string()).expect("Falha ao gerar token");

    // 4. Gerar token JWT
    let token = generate_jwt(&user.id.to_string(), &profile.access_level).map_err(|err| {
        error!(
            error = %err,
            "Erro ao gerar token"
        );
        AppError::BadRequest(Some("Erro ao gerar token".into()))
    })?;

    let settings = get_settings();
    let expires_in = settings.jwt.expires_in.to_string();
    let user_with_profile = UserWithProfile::from_user_and_profile(user, profile);

    info!(
        user_id = %user_with_profile.id,
        "Login realizado com sucesso"
    );

    Ok(UserResponse::from(user_with_profile, token, expires_in))
}

#[allow(unused_variables)]
pub async fn confirm_email(code: &str, state: &web::Data<AppState>) -> Result<(), AppError> {
    let db = &state.db;
    let mongo_db = &state.mongo;
    // Valida o token de confirmação de email
    let token = token_service::get_and_validate_token(code, "confirm_email", db).await?;

    // Marca o token como consumido
    token_repository::update_token(db, code).await?;

    // Marca o email como confirmado no perfil
    profile_repository::confirm_email(token.user_id, db).await?;

    Ok(())
}

#[allow(unused_variables)]
pub async fn forgot_password(email: &str, state: &web::Data<AppState>) -> Result<(), AppError> {
    let db = &state.db;
    let mongo_db = &state.mongo;
    // Busca o usuário pelo email
    let user = user_repository::find_user_by_email(email, db)
        .await?
        .ok_or_else(|| AppError::NotFound(Some("Usuário não encontrado".into())))?;

    // Cria um token para mudança de senha
    let token_type = "change_password";
    let code = token_service::create_user_token(user.id, token_type, db).await?;
    println!("code change password: {}", code);

    // let email_service = EmailService::new().await?;
    // let email_link = format!("https://seusite.com/reset?token={}", code);
    // email_service
    //     .send_reset_password(email, &user.first_name, &email_link)
    //     .await?;

    // Por enquanto apenas retorna Ok, depois implementaremos o envio de email
    Ok(())
}

#[allow(unused_variables)]
pub async fn change_password(
    code: &str,
    new_password: &str,
    // db: &PgPool,
    // mongo_db: &Database,
    state: &web::Data<AppState>,
) -> Result<(), AppError> {
    let db = &state.db;
    let mongo_db = &state.mongo;
    // Valida o token de mudança de senha
    let token = token_service::get_and_validate_token(code, "change_password", db).await?;

    // Hash da nova senha
    let hashed_password = bcrypt::hash(new_password, bcrypt::DEFAULT_COST)
        .map_err(|_| AppError::InternalError(Some("Erro ao hashear senha".into())))?;

    // Atualiza a senha do usuário
    user_repository::update_user_password(db, token.user_id, &hashed_password).await?;

    // Marca o token como consumido
    token_repository::update_token(db, code).await?;

    Ok(())
}

pub async fn get_test_db_pool() -> PgPool {
    let db_url = env::var("DATABASE_URL_TEST")
        .expect("DATABASE_URL_TEST não definida");
    PgPool::connect(&db_url)
        .await
        .expect("Falha ao conectar no banco de testes")
}

pub async fn clean_test_db(db: &PgPool) {
    // Truncar tabelas na ordem correta (respeitando foreign keys)
    sqlx::query!("TRUNCATE TABLE user_tokens CASCADE").execute(db).await.unwrap();
    sqlx::query!("TRUNCATE TABLE profiles CASCADE").execute(db).await.unwrap();
    sqlx::query!("TRUNCATE TABLE users CASCADE").execute(db).await.unwrap();
    // Adicione outras tabelas conforme necessário
}

pub async fn setup_test_db() -> PgPool {
    let pool = get_test_db_pool().await;
    clean_test_db(&pool).await;
    pool
}
