use crate::models::profile::Profile;
use crate::utils::validation::{validate_email, validate_password};
use bcrypt::{DEFAULT_COST, hash, verify};
use chrono::NaiveDateTime;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

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
    pub dt_deleted: Option<NaiveDateTime>,
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
            dt_deleted: None,
        }
    }

    pub fn verify_password(&self, input: &str) -> bool {
        verify(input, &self.password).unwrap_or(false)
    }
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct ProfileRequest {
    #[validate(length(
        min = 3,
        max = 500,
        message = "A bio deve ter entre 3 e 500 caracteres"
    ))]
    pub bio: Option<String>,

    #[validate(custom = "crate::utils::validation::validate_birth_date")]
    pub birth_date: Option<String>,

    #[validate(custom = "crate::utils::validation::validate_phone")]
    pub phone: Option<String>,

    #[validate(custom = "crate::utils::validation::validate_document")]
    pub document: Option<String>,

    #[validate(length(
        min = 2,
        max = 100,
        message = "A profissão deve ter entre 2 e 100 caracteres"
    ))]
    pub profession: Option<String>,

    #[validate(url(message = "URL do avatar inválida"))]
    pub avatar: Option<String>,

    pub confirm_email: Option<bool>,
    pub unsubscribe: Option<bool>,

    #[validate(length(
        min = 2,
        max = 50,
        message = "O nível de acesso deve ter entre 2 e 50 caracteres"
    ))]
    pub access_level: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UserRequest {
    #[validate(custom = "validate_email")]
    pub email: String,

    #[validate(length(min = 2, max = 50, message = "O nome deve ter entre 2 e 50 caracteres"))]
    pub first_name: String,

    #[validate(length(
        min = 2,
        max = 50,
        message = "O sobrenome deve ter entre 2 e 50 caracteres"
    ))]
    pub last_name: String,

    #[validate(custom = "validate_password")]
    pub password: String,

    #[validate]
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

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}
