use dotenv::dotenv;
use std::env;

pub struct Config {
    pub database_url: String,
    pub server_address: String,
    // Adicione outras configurações aqui
}

impl Config {
    pub fn new() -> Self {
        dotenv().ok(); // Carrega variáveis de ambiente do arquivo .env

        Config {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL não definida"),
            server_address: env::var("SERVER_ADDRESS").expect("SERVER_ADDRESS must be set"), // Carrega o endereço do servidor do arquivo .env

                                                                                             // Inicialize outras configurações aqui
        }
    }
}
