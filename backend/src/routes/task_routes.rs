use actix_web::{web, HttpResponse, Responder};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/tasks", web::get().to(list_tasks))
        .route("/tasks", web::post().to(create_task));
    // Adicione rotas para atualizar e deletar conforme necessário
}

async fn list_tasks() -> impl Responder {
    // Implementação de listagem de tarefas
    HttpResponse::Ok().json("Listagem de tarefas")
}

async fn create_task() -> impl Responder {
    // Implementação de criação de tarefa
    HttpResponse::Ok().json("Tarefa criada")
}
