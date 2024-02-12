use actix_web::{web, HttpResponse, Responder};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/auth/login", web::post().to(login));
    // Adicione mais rotas de autenticação conforme necessário
}

async fn login() -> impl Responder {
    HttpResponse::Ok().body("Login successful")
}
