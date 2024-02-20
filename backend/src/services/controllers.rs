// use crate::services::openai_service::OpenAIService;
// use actix_web::{web, HttpResponse};

// pub async fn ask_openai(prompt: web::Json<String>) -> HttpResponse {
//     match OpenAIService::send_to_openai(&prompt).await {
//         Ok(response) => HttpResponse::Ok().body(response),
//         Err(_) => HttpResponse::InternalServerError().finish(),
//     }
// }
use crate::services::openai_service::OpenAIService;
use actix_web::{web, HttpResponse, Responder};

pub async fn ask_openai(
    openai_service: web::Data<OpenAIService>,
    payload: web::Json<serde_json::Value>,
) -> impl Responder {
    let text = payload["text"].as_str().unwrap_or_default();

    match openai_service.send_to_openai(text).await {
        Ok(response) => HttpResponse::Ok().body(response),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
