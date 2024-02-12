use super::super::services::google_calendar_service::GoogleCalendarService;
use actix_web::{web, HttpResponse, Responder};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/events", web::get().to(list_events))
        .route("/events", web::post().to(create_event));
    // Adicione rotas para atualizar e deletar conforme necessário
}

async fn list_events() -> impl Responder {
    // Exemplo: Implementação de listagem de eventos
    HttpResponse::Ok().json("Listagem de eventos")
}

async fn create_event() -> impl Responder {
    // Exemplo: Implementação de criação de evento
    HttpResponse::Ok().json("Evento criado")
}
