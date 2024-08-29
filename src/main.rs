mod database;
mod handlers;
mod models;
mod schema;
use crate::handlers::classroom::get_classrooms;
use actix_web::{web, App, HttpServer};
use handlers::classroom::{create_classroom, delete_classroom, update_classroom};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/classroom", web::post().to(create_classroom))
            .route("/classrooms", web::get().to(get_classrooms))
            // changed this route path
            .route("/classroom/{id}", web::put().to(update_classroom))
            .route("/classroom/{id}", web::delete().to(delete_classroom))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
