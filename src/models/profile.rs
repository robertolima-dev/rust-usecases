use crate::models::user::ProfileRequest;
use chrono::Utc;
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct Profile {
    pub id: uuid::Uuid,
    pub user_id: Uuid,
    pub bio: Option<String>,
    pub birth_date: Option<chrono::NaiveDate>,
    pub phone: Option<String>,
    pub document: Option<String>,
    pub profession: Option<String>,
    pub avatar: Option<String>,
    pub confirm_email: bool,
    pub unsubscribe: bool,
    pub access_level: String,
    pub dt_updated: NaiveDateTime,
    pub dt_created: NaiveDateTime,
}

#[allow(dead_code)]
impl Profile {
    pub fn new(user_id: Uuid) -> Self {
        let now = Utc::now().naive_utc();

        Self {
            id: Uuid::new_v4(),
            user_id,
            bio: None,
            birth_date: None,
            phone: None,
            document: None,
            profession: None,
            avatar: None,
            confirm_email: false,
            unsubscribe: false,
            access_level: "user".to_string(),
            dt_created: now,
            dt_updated: now,
        }
    }
    pub fn from_request(user_id: Uuid, req: Option<ProfileRequest>) -> Self {
        let now = Utc::now().naive_utc();

        if let Some(profile) = req {
            Self {
                id: Uuid::new_v4(),
                user_id,
                bio: profile.bio,
                birth_date: profile
                    .birth_date
                    .and_then(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok()),
                phone: profile.phone,
                document: profile.document,
                profession: profile.profession,
                avatar: profile.avatar,
                confirm_email: profile.confirm_email.unwrap_or(false),
                unsubscribe: profile.unsubscribe.unwrap_or(false),
                access_level: profile.access_level.unwrap_or_else(|| "user".to_string()),
                dt_created: now,
                dt_updated: now,
            }
        } else {
            // Se nenhum profile foi enviado, cria um default
            Self {
                id: Uuid::new_v4(),
                user_id,
                bio: None,
                birth_date: None,
                phone: None,
                document: None,
                profession: None,
                avatar: None,
                confirm_email: false,
                unsubscribe: false,
                access_level: "user".to_string(),
                dt_created: now,
                dt_updated: now,
            }
        }
    }
}
