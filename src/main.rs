mod models;
mod repositories;
mod services;
mod handlers;

use axum::{
    routing::{get, post, put, delete},
    Router,
};
use sqlx::mysql::MySqlPoolOptions;
use dotenv::dotenv;
use std::{env, sync::Arc};
use repositories::user_repository::UserRepository;
use services::user_service::UserService;
use handlers::user_handler;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;
    
    // Create repository
    let user_repository = UserRepository::new(pool);
    
    // Create service
    let user_service = Arc::new(UserService::new(user_repository));
    
    // Build routes
    let app = Router::new()
        .route("/users", post(user_handler::create_user))
        .route("/users", get(user_handler::get_all_users))
        .route("/users/{id}", get(user_handler::get_user))
        .route("/users/{id}", put(user_handler::update_user))
        .route("/users/{id}", delete(user_handler::delete_user))
        .with_state(user_service);
    
    // Start server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await?;
    
    println!("Server running on http://127.0.0.1:3000");
    
    axum::serve(listener, app).await?;
    
    Ok(())
}