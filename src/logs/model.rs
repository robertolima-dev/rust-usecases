use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Info,
    Warn,
    Error,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogEntry {
    pub id: Uuid,
    pub level: LogLevel,
    pub message: String,
    pub module: String,
    pub timestamp: DateTime<Utc>,
    pub user_id: Option<Uuid>, // caso log seja relacionado a usuário
}

#[derive(Debug, Deserialize)]
pub struct LogQuery {
    pub level: Option<LogLevel>,
    pub module: Option<String>,
    pub limit: Option<i64>,
    pub skip: Option<i64>,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            LogLevel::Info => "info",
            LogLevel::Warn => "warn",
            LogLevel::Error => "error",
        };
        write!(f, "{}", s)
    }
}
