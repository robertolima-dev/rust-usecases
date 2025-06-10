use lazy_static::lazy_static;
use regex::Regex;
use validator::ValidationError;

lazy_static! {
    static ref EMAIL_REGEX: Regex =
        Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    static ref PASSWORD_REGEX: Regex =
        Regex::new(r"^(?=.*[A-Za-z])(?=.*\d)[A-Za-z\d@$!%*#?&]{8,}$").unwrap();
    static ref PHONE_REGEX: Regex = Regex::new(r"^\+?[1-9]\d{1,14}$").unwrap();
    static ref DOCUMENT_REGEX: Regex = Regex::new(r"^\d{3}\.\d{3}\.\d{3}-\d{2}$").unwrap();
}

pub fn validate_email(email: &str) -> Result<(), ValidationError> {
    if !EMAIL_REGEX.is_match(email) {
        let mut err = ValidationError::new("invalid_email");
        err.message = Some("Email inválido".into());
        return Err(err);
    }
    Ok(())
}

pub fn validate_password(password: &str) -> Result<(), ValidationError> {
    if !PASSWORD_REGEX.is_match(password) {
        let mut err = ValidationError::new("invalid_password");
        err.message =
            Some("A senha deve ter pelo menos 8 caracteres, incluindo letras e números".into());
        return Err(err);
    }
    Ok(())
}

pub fn validate_phone(phone: &str) -> Result<(), ValidationError> {
    if !PHONE_REGEX.is_match(phone) {
        let mut err = ValidationError::new("invalid_phone");
        err.message = Some("Número de telefone inválido".into());
        return Err(err);
    }
    Ok(())
}

pub fn validate_document(document: &str) -> Result<(), ValidationError> {
    if !DOCUMENT_REGEX.is_match(document) {
        let mut err = ValidationError::new("invalid_document");
        err.message = Some("CPF inválido. Use o formato: 000.000.000-00".into());
        return Err(err);
    }
    Ok(())
}

pub fn validate_birth_date(birth_date: &str) -> Result<(), ValidationError> {
    if let Err(_) = chrono::NaiveDate::parse_from_str(birth_date, "%Y-%m-%d") {
        let mut err = ValidationError::new("invalid_birth_date");
        err.message = Some("Data de nascimento inválida. Use o formato: YYYY-MM-DD".into());
        return Err(err);
    }
    Ok(())
}
