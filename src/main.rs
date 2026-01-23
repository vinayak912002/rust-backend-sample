mod models;
mod repositories;
mod services;
mod handlers;
mod routes;

use sqlx::mysql::MySqlPoolOptions;
use redis::Client as RedisClient;

use dotenv::dotenv;
use std::{env, sync::Arc};
use repositories::user_repository::UserRepository;
use services::user_service::UserService;

use crate::routes::user_routes::user_routes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    
    let redis_url = env::var("REDIS_URL")
        .expect("Please set the REDIS url in .env");
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;
    
    // Create repository
    let user_repository = UserRepository::new(pool);
    
     // Build redis client
    let redis_client = RedisClient::open(redis_url).expect("Failed to load Redis");

    // Create service
    let user_service = Arc::new(UserService::new(user_repository, redis_client));
    
    // Build routes
    let app = user_routes(user_service);
    
    // Start server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await?;
    
    println!("Server running on http://127.0.0.1:3000");
    
    axum::serve(listener, app).await?;
    
    Ok(())
}