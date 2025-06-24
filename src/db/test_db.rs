use sqlx::PgPool;
use std::env;

pub async fn get_test_db_pool() -> PgPool {
    let db_url = env::var("DATABASE_URL_TEST")
        .expect("DATABASE_URL_TEST não definida");
    PgPool::connect(&db_url)
        .await
        .expect("Falha ao conectar no banco de testes")
}

pub async fn run_migrations(db: &PgPool) {
    sqlx::migrate!("./migrations")
        .run(db)
        .await
        .expect("Falha ao executar migrations no banco de testes");
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
    run_migrations(&pool).await;
    clean_test_db(&pool).await;
    pool
}