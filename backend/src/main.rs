use actix_cors::Cors;
use actix_identity::IdentityMiddleware;

use actix_web::{http::header, middleware, web, App, HttpServer};
use dotenv::dotenv;
use std::env; // Adicionado para leitura da variável de ambiente
use std::io;

mod config;
mod models;
mod routes;
mod services;
mod utils;

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok(); // Carrega as variáveis de ambiente do arquivo .env
    env_logger::init(); // Inicializa o logger

    let config = config::Config::new(); // Inicializa a configuração do projeto
    println!("Starting server at {}", &config.server_address);

    // Leitura da chave da API da OpenAI
    let openai_api_key = env::var("OPENAI_KEY").expect("OPENAI_KEY must be set");

    // Inicializa a conexão com o banco de dados
    let database_service = services::database_service::DatabaseService::new(&config.database_url)
        .await
        .expect("Failed to create database service");

    // Criação do serviço OpenAIService com a chave da API da OpenAI
    let openai_service = services::openai_service::OpenAIService::new(openai_api_key);

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default()) // Middleware para log de requisições
            // .wrap(
            //     Cors::new() // Adicione o middleware CORS aqui
            //         .allowed_origin(&config.allowed_origin) // Permitir a origem dinamicamente carregada do arquivo .env
            //         .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            //         .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            //         .max_age(3600),
            // )
            .wrap(Cors::permissive()) // Configuração CORS permissiva
            .wrap(IdentityMiddleware::default())
            .data(database_service.clone()) // Torna o serviço de banco de dados disponível para os handlers
            .data(openai_service.clone()) // Disponibiliza o serviço OpenAIService para a aplicação
            // Configuração das rotas
            .configure(routes::auth_routes::config) // Autenticação
            .configure(routes::event_routes::config) // Rotas de eventos
            .configure(routes::task_routes::config) // Rotas de tarefas
            .configure(routes::chat_routes::config) // Chat
            .configure(routes::calendar_routes::config) // calendario
                                                        // Rotas de eventos
                                                        // Configure mais rotas conforme necessário
                                                        // Inclua mais configurações conforme necessário, por exemplo:
                                                        // .configure(routes::task_routes::config) // Se você tiver rotas para tarefas
                                                        // Inicialização e configuração de serviços podem ser feitas aqui
                                                        // Por exemplo, passar o pool de conexões do banco de dados para os handlers
    })
    .bind(&config.server_address)? // Utiliza o endereço do servidor definido na configuração
    .run()
    .await
}
