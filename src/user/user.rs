use actix_web::{web, HttpResponse, Responder};
use crate::user::models::User;

pub async fn create_user(item: web::Json<User>) -> impl Responder {
    HttpResponse::Ok().json(item.into_inner()) // Simulating creation
}

pub async fn get_user(user_id: web::Path<u32>) -> impl Responder {
    HttpResponse::Ok().json(User {
        id: user_id.into_inner(),
        name: "John Doe".to_string(),
        email: "john@example.com".to_string(),
    }) // Simulating a user fetch
}

pub async fn update_user(user_id: web::Path<u32>, item: web::Json<User>) -> impl Responder {
    println!("Updating user with id: {}", user_id.into_inner());
    HttpResponse::Ok().json(item.into_inner())
}

pub async fn delete_user(user_id: web::Path<u32>) -> impl Responder {
    HttpResponse::Ok().body(format!("Deleted user {}", user_id)) // Simulating deletion
}
