use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route(
        "/api/chat",
        web::post().to(crate::services::controllers::ask_openai),
    );
}
