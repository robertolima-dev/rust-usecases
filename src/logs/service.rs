use crate::logs::model::{LogEntry, LogLevel, LogQuery};
use crate::logs::repository;
use chrono::Utc;
use mongodb::Database;
use uuid::Uuid;

pub async fn log_event(
    level: LogLevel,
    message: &str,
    module: &str,
    user_id: Option<Uuid>,
    db: &Database,
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
    if let Err(err) = db
        .collection::<LogEntry>("logs")
        .insert_one(entry, None)
        .await
    {
        eprintln!("❗ Falha ao salvar log no MongoDB: {:?}", err);
    }
}

pub async fn list_logs(
    db: &Database,
    query: &LogQuery,
) -> Result<Vec<LogEntry>, mongodb::error::Error> {
    repository::find_logs(db, query).await
}
