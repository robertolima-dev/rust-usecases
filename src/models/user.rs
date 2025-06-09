use crate::models::profile::Profile;
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

#[derive(Debug, Deserialize)]
pub struct ProfileRequest {
    pub bio: Option<String>,
    pub birth_date: Option<String>,
    pub phone: Option<String>,
    pub document: Option<String>,
    pub profession: Option<String>,
    pub avatar: Option<String>,
    pub confirm_email: Option<bool>,
    pub unsubscribe: Option<bool>,
    pub access_level: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UserRequest {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
    pub profile: Option<ProfileRequest>,
}

#[derive(Serialize)]
pub struct UserWithProfile {
    pub id: uuid::Uuid,
    pub username: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub profile: Profile,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub user: UserWithProfile,
    pub expires_in: String,
    pub token: String,
}

#[allow(dead_code)]
impl UserWithProfile {
    pub fn from(user: User, profile: Profile) -> Self {
        Self {
            id: user.id,
            username: user.username,
            email: user.email,
            first_name: user.first_name,
            last_name: user.last_name,
            profile,
        }
    }

    pub fn from_user_and_profile(user: User, profile: Profile) -> Self {
        UserWithProfile {
            id: user.id,
            username: user.username,
            email: user.email,
            first_name: user.first_name,
            last_name: user.last_name,
            profile,
        }
    }
}

impl UserResponse {
    pub fn from(user_with_profile: UserWithProfile, token: String, expires_in: String) -> Self {
        Self {
            user: user_with_profile,
            token,
            expires_in,
        }
    }
}
