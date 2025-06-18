use crate::cli_init::init_cli_environment;
use elasticsearch::IndexParts;
use rust_usecases::db::elasticsearch::get_elastic_client;
use rust_usecases::db::postgres::get_db_pool;
use rust_usecases::models::course::Course;
use serde_json::json;

pub async fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    init_cli_environment(); // ✅ Isso vai rodar dotenv + init_settings + setup logging

    // Agora pode usar get_settings() tranquilo
    let settings = rust_usecases::config::get_settings();
    println!(
        "✅ JWT Secret carregado com sucesso: {}",
        settings.jwt.secret
    );

    let db = get_db_pool().await;
    let elastic = get_elastic_client()?; // Ajuste se necessário

    let courses: Vec<Course> = sqlx::query_as!(
        Course,
        r#"
        SELECT id, name, description, is_active, price, month_duration, author_id, dt_start, dt_created, dt_updated, dt_deleted
        FROM courses
        WHERE dt_created IS NULL
        "#
    )
    .fetch_all(&db)
    .await?;

    println!("👉 Encontrados {} cursos", courses.len());

    for course in courses {
        let doc_id = course.id.to_string();

        let body = json!({
            "id": course.id,
            "name": course.name,
            "description": course.description,
            "is_active": course.is_active,
            "price": course.price,
            "month_duration": course.month_duration,
            "author_id": course.author_id,
            "dt_start": course.dt_start,
            "dt_created": course.dt_created,
        });

        let response = elastic
            .index(IndexParts::IndexId("courses", &doc_id))
            .body(body)
            .send()
            .await?;

        println!(
            "✅ Indexado curso: {} | Status: {:?}",
            course.name,
            response.status_code()
        );
    }

    Ok(())
}
