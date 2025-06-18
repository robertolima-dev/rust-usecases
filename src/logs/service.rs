use crate::config::app_state::AppState;
use crate::logs::model::{LogEntry, LogLevel, LogQuery};
use crate::logs::repository;
use actix_web::web;
use chrono::Utc;
use mongodb::Database;
use uuid::Uuid;

pub async fn log_event(
    level: LogLevel,
    message: &str,
    module: &str,
    user_id: Option<Uuid>,
    mongodb: &Database,
) {
    let entry = LogEntry {
        id: Uuid::new_v4(),
        level,
        message: message.to_string(),
        module: module.to_string(),
        timestamp: Utc::now(),
        user_id,
    };

    // Ignora erros ao inserir log (não queremos travar o sistema)
    if let Err(err) = mongodb
        .collection::<LogEntry>("logs")
        .insert_one(entry, None)
        .await
    {
        eprintln!("❗ Falha ao salvar log no MongoDB: {:?}", err);
    }
}

pub async fn list_logs(
    query: &LogQuery,
    state: &web::Data<AppState>,
) -> Result<Vec<LogEntry>, mongodb::error::Error> {
    let mongo = &state.mongo;
    repository::find_logs(mongo, query).await
}
