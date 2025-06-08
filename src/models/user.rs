use bcrypt::{DEFAULT_COST, hash, verify};
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct User {
    #[serde(skip_deserializing)]
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub birth_date: Option<NaiveDate>,
    #[serde(skip_serializing)]
    pub password: String,
    pub created_at: Option<NaiveDateTime>,
}

#[allow(dead_code)]
impl User {
    pub fn new(
        name: &str,
        email: &str,
        phone: Option<&str>,
        birth_date: Option<String>,
        password: &str,
    ) -> Self {
        let birth_date_parsed = birth_date
            .as_deref()
            .map(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").expect("Data inválida"));

        let hashed = hash(password, DEFAULT_COST).expect("Erro ao hashear");

        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            email: email.to_string(),
            phone: phone.map(|s| s.to_string()),
            birth_date: birth_date_parsed,
            password: hashed,
            created_at: None,
        }
    }

    pub fn verify_password(&self, input: &str) -> bool {
        verify(input, &self.password).unwrap_or(false)
    }
}
