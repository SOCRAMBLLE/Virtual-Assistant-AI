// use actix_web::{HttpResponse, ResponseError};

use std::fmt;

use actix_web::{error, HttpResponse};
use reqwest::StatusCode;

#[derive(Debug)]
pub enum ServiceError {
    BadRequest,
    NotFound,
    // Adicione mais tipos de erros conforme necessário
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// impl error::ResponseError for ServiceError {
//     fn error_response(&self) -> HttpResponse {
//         // Implemente a resposta com base no tipo de erro
//         HttpResponse::InternalServerError().json("An internal error occurred")
//     }
// }
//apartir daqui pode ser o novo codigo e substitui o de cima, tenho que ver pq esta com erro

impl error::ResponseError for ServiceError {
    fn status_code(&self) -> StatusCode {
        match *self {
            ServiceError::BadRequest => StatusCode::BAD_REQUEST,
            ServiceError::NotFound => StatusCode::NOT_FOUND,
            // Adicione mais mapeamentos conforme necessário
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .content_type("application/json")
            .body(self.to_string())
    }
}
