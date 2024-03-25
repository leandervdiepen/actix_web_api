mod db;
mod user;
use actix_web::{web, App, HttpResponse, HttpServer};
use db::connection::establish_connection;
use mongodb::Database;
use user::{create_user, delete_user, get_user, update_user};

pub struct AppState {
    db: Database,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let mongo_client = establish_connection()
        .await
        .expect("Failed to connect to MongoDB");
    let db = mongo_client.database("db");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { db: db.clone() }))
            .route(
                "/",
                web::get().to(|| async { HttpResponse::Ok().body("Hello world") }),
            )
            .route("/users", web::post().to(create_user))
            .route("/users/{id}", web::get().to(get_user))
            .route("/users/{id}", web::put().to(update_user))
            .route("/users/{id}", web::delete().to(delete_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
