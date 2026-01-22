use axum::{
    extract::{Path, State},
    http::{StatusCode},
    Json,
};
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use crate::services::user_service::UserService;

// REQUEST DTO
#[derive(Deserialize)]
pub struct CreateUserRequest{
    name:String,
    age:i32,
}

// RESPONSE DTO
#[derive(Serialize)]
pub struct UserResponse{
    id:String,
    name:String,
    age:i32,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error : String,
}

pub async fn create_user(
    State(service): State<Arc<UserService>>,
    Json(payload): Json<CreateUserRequest>
)->Result<(StatusCode, Json<UserResponse>),(StatusCode, Json<ErrorResponse>)>{
    match service.create_user(payload.name, payload.age).await {
        Ok(user)=> Ok((
            StatusCode::CREATED,
            Json(UserResponse{
                id:user.id,
                name:user.name,
                age:user.age,
            }),
        )),
        Err(e) => Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse{
                error: e.to_string(),
            }),
        )),
    }
}

// GET USER BY ID
pub async fn get_user(
    State(service): State<Arc<UserService>>,
    Path(id): Path<String>,
) -> Result<Json<UserResponse>, (StatusCode, Json<ErrorResponse>)> {
    match service.get_user(&id).await {
        Ok(user) => Ok(Json(UserResponse {
            id: user.id,
            name: user.name,
            age: user.age,
        })),
        Err(_) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "User not found".to_string(),
            }),
        )),
    }
}

// GET ALL USERS
pub async fn get_all_users(
    State(service): State<Arc<UserService>>,
) -> Result<Json<Vec<UserResponse>>, (StatusCode, Json<ErrorResponse>)> {
    match service.get_all_users().await {
        Ok(users) => {
            let response: Vec<UserResponse> = users
                .into_iter()
                .map(|user| UserResponse {
                    id: user.id,
                    name: user.name,
                    age: user.age,
                })
                .collect();
            Ok(Json(response))
        }
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: e.to_string(),
            }),
        )),
    }
}

// UPDATE USER
pub async fn update_user(
    State(service): State<Arc<UserService>>,
    Path(id): Path<String>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<UserResponse>, (StatusCode, Json<ErrorResponse>)> {
    match service.update_user(&id, payload.name, payload.age).await {
        Ok(user) => Ok(Json(UserResponse {
            id: user.id,
            name: user.name,
            age: user.age,
        })),
        Err(_) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "User not found".to_string(),
            }),
        )),
    }
}

// DELETE USER
pub async fn delete_user(
    State(service): State<Arc<UserService>>,
    Path(id): Path<String>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    match service.delete_user(&id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "User not found".to_string(),
            }),
        )),
    }
}