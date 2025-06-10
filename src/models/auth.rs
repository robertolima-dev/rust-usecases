// models/auth.rs
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::utils::validation::{validate_email, validate_password};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // ID do usuário ou email
    pub exp: usize,  // timestamp de expiração
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(custom = "validate_email")]
    pub email: String,
    
    #[validate(custom = "validate_password")]
    pub password: String,
}
