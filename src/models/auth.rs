// models/auth.rs
use crate::utils::validation::validate_password;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // ID do usuário ou email
    pub exp: usize,  // timestamp de expiração
    pub access_level: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,

    #[validate(custom = "validate_password")]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct ForgotPasswordRequest {
    #[validate(email)]
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct ChangePasswordRequest {
    pub code: String,
    #[validate(custom = "validate_password")]
    pub password: String,
}
