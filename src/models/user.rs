use bcrypt::{DEFAULT_COST, hash, verify};
use chrono::NaiveDateTime;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub dt_created: NaiveDateTime,
    pub dt_updated: NaiveDateTime,
}

#[allow(dead_code)]
impl User {
    pub fn new(
        username: &str,
        email: &str,
        first_name: &str,
        last_name: &str,
        password: &str,
    ) -> Self {
        let hashed = hash(password, DEFAULT_COST).expect("Erro ao hashear");
        let now: NaiveDateTime = Utc::now().naive_utc();

        Self {
            id: Uuid::new_v4(),
            username: username.to_string(),
            email: email.to_string(),
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            password: hashed,
            dt_created: now,
            dt_updated: now,
        }
    }

    pub fn verify_password(&self, input: &str) -> bool {
        verify(input, &self.password).unwrap_or(false)
    }
}
