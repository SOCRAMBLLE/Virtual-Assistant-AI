use crate::services::controllers;
use actix_web::web;

// pub fn config(cfg: &mut web::ServiceConfig) {
//     cfg.service(
//         web::scope("/api")
//             .service(web::resource("/chat").route(web::post().to(controllers::ask_openai))), // Adicione outros endpoints aqui, se necessário
//     );
// }

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/api/chat", web::post().to(controllers::ask_openai));

    // Adicione mais rotas de autenticação conforme necessário
}
