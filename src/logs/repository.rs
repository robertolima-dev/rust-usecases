use crate::logs::model::{LogEntry, LogQuery};
use futures::StreamExt;
use mongodb::Database;
use mongodb::{bson::doc, options::FindOptions};

pub async fn find_logs(
    db: &Database,
    query: &LogQuery,
) -> Result<Vec<LogEntry>, mongodb::error::Error> {
    let collection = db.collection::<LogEntry>("logs");

    let mut filter = doc! {};
    if let Some(level) = &query.level {
        filter.insert("level", level.to_string());
    }
    if let Some(module) = &query.module {
        filter.insert("module", module);
    }

    let options = FindOptions::builder()
        .limit(query.limit.unwrap_or(50))
        .skip(query.skip.map(|v| v as u64))
        .sort(doc! { "timestamp": -1 })
        .build();

    let cursor = collection.find(filter, options).await?;

    let logs: Vec<LogEntry> = cursor.filter_map(|doc| async { doc.ok() }).collect().await;

    Ok(logs)
}
