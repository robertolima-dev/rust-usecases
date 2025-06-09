use actix_web::{HttpResponse, ResponseError};
use derive_more::Display; // essa macro implementa Display por você!
// use std::fmt::{self, Display, Formatter};

#[derive(Debug, Display)]
#[allow(dead_code)]
pub enum AppError {
    #[display(fmt = "Recurso não encontrado")]
    NotFound(Option<String>),

    #[display(fmt = "Não autorizado")]
    Unauthorized(Option<String>),

    #[display(fmt = "Requisição inválida")]
    BadRequest(Option<String>),

    #[display(fmt = "Erro interno do servidor")]
    InternalError(Option<String>),
}

#[allow(dead_code)]
impl AppError {
    pub fn not_found<S: Into<String>>(msg: S) -> Self {
        AppError::NotFound(Some(msg.into()))
    }

    pub fn unauthorized<S: Into<String>>(msg: S) -> Self {
        AppError::Unauthorized(Some(msg.into()))
    }

    pub fn bad_request<S: Into<String>>(msg: S) -> Self {
        AppError::BadRequest(Some(msg.into()))
    }

    pub fn internal<S: Into<String>>(msg: S) -> Self {
        AppError::InternalError(Some(msg.into()))
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::NotFound(msg) => {
                HttpResponse::NotFound().json(msg.as_deref().unwrap_or("Recurso não encontrado"))
            }
            AppError::Unauthorized(msg) => {
                HttpResponse::Unauthorized().json(msg.as_deref().unwrap_or("Não autorizado"))
            }
            AppError::BadRequest(msg) => {
                HttpResponse::BadRequest().json(msg.as_deref().unwrap_or("Requisição inválida"))
            }
            AppError::InternalError(msg) => HttpResponse::InternalServerError()
                .json(msg.as_deref().unwrap_or("Erro interno do servidor")),
        }
    }
}
