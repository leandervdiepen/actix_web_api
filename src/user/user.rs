use crate::{user::models::User, AppState};
use actix_web::{web, HttpResponse, Responder};
use mongodb::{
    bson::doc,
    Collection,
};
use serde_json::json;

pub async fn create_user(data: web::Data<AppState>, item: web::Json<User>) -> impl Responder {
    let users_collection: Collection<User> = data.db.collection("users");
    match users_collection.insert_one(item.into_inner(), None).await {
        Ok(_) => HttpResponse::Ok().body("User created"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn get_user(data: web::Data<AppState>, user_id: web::Path<u32>) -> impl Responder {
    let users_collection: Collection<User> = data.db.collection("users");

    let filter = doc! { "id": user_id.into_inner() };
    match users_collection.find_one(filter, None).await {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().json(json!({"error": "User not found"})),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

pub async fn update_user(
    data: web::Data<AppState>,
    user_id: web::Path<u32>,
    item: web::Json<User>,
) -> impl Responder {
    let users_collection: Collection<User> = data.db.collection("users");
    let filter = doc! { "id": user_id.into_inner() };
    let update = doc! { "$set": bson::to_document(&item.into_inner()).unwrap() };

    match users_collection.update_one(filter, update, None).await {
        Ok(update_result) => {
            if update_result.matched_count == 1 {
                HttpResponse::Ok().json("User updated successfully")
            } else {
                HttpResponse::NotFound().json("User not found")
            }
        }
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

pub async fn delete_user(data: web::Data<AppState>, user_id: web::Path<u32>) -> impl Responder {
    let users_collection: Collection<User> = data.db.collection("users");
    let filter = doc! { "id": user_id.into_inner() };

    match users_collection.delete_one(filter, None).await {
        Ok(delete_result) => {
            if delete_result.deleted_count == 1 {
                HttpResponse::Ok().json("User deleted successfully")
            } else {
                HttpResponse::NotFound().json("User not found")
            }
        }
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}
