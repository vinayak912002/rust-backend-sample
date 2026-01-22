use crate::services::user_service::UserService;
use axum::{Router, routing::{get, post, delete, put}};
use crate::handlers::user_handler;
use std::sync::Arc;

pub fn user_routes(service:Arc<UserService>)->Router{
    Router::new()
        .route("/users", post(user_handler::create_user))
        .route("/users", get(user_handler::get_all_users))
        .route("/users/{id}", get(user_handler::get_user))
        .route("/users/{id}", put(user_handler::update_user))
        .route("/users/{id}", delete(user_handler::delete_user))
        .with_state(service)
}