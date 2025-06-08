use crate::models::{profile::Profile, user::User};
use serde::Serialize;

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
    pub fn from(user_with_profile: UserWithProfile) -> Self {
        Self {
            user: user_with_profile,
            expires_in: "604800".to_string(),
            token: "token_mock_gerado".to_string(),
        }
    }
}
