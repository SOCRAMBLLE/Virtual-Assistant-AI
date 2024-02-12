use actix_web::{middleware, web, App, HttpServer};
use dotenv::dotenv;
use std::io;

mod config;
mod models;
mod routes;
mod services;
mod utils;

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok(); // Carrega variáveis de ambiente do arquivo .env

    // Inicializa a configuração do projeto
    let config = config::Config::new();
    println!("Starting server at {}", &config.server_address);

    // Inicializa a conexão com o banco de dados (exemplo com sqlx e PostgreSQL)
    let database_service = services::database_service::DatabaseService::new(&config.database_url)
        .await
        .expect("Failed to create database service");

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default()) // Middleware para log de requisições
            .data(database_service.clone()) // Torna o serviço de banco de dados disponível para os handlers
            .configure(routes::auth_routes::config) // Autenticação
            .configure(routes::event_routes::config)
            .configure(routes::task_routes::config) // Rotas de eventos
                                                    // Configure mais rotas conforme necessário
                                                    // Inclua mais configurações conforme necessário, por exemplo:
                                                    // .configure(routes::task_routes::config) // Se você tiver rotas para tarefas
                                                    // Inicialização e configuração de serviços podem ser feitas aqui
                                                    // Por exemplo, passar o pool de conexões do banco de dados para os handlers
    })
    .bind(config.server_address)? // Utiliza o endereço do servidor definido na configuração
    .run()
    .await
}
