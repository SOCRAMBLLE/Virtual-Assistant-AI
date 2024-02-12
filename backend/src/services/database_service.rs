use crate::models::user::User;
use sqlx::{postgres::PgPoolOptions, Executor as _, PgPool};
use std::clone::Clone;
use std::env;

pub struct DatabaseService {
    pool: PgPool,
}

impl DatabaseService {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let database_url = env::var("DATABASE_URL")
            .map_err(|_| sqlx::Error::Configuration("DATABASE_URL must be set".into()))?;
        let pool = PgPoolOptions::new().connect(&database_url).await?;

        Ok(DatabaseService { pool })
    }

    /// Adiciona um novo usuário ao banco de dados.
    pub async fn add_user(&self, user: &User) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO users (name, email) VALUES ($1, $2)",
            user.name,
            user.email
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}

impl Clone for DatabaseService {
    fn clone(&self) -> Self {
        DatabaseService {
            // Clonar campos conforme necessário
            pool: self.pool.clone(),
        }
    }
}
