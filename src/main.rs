mod auth;
mod database;
mod errors;
mod handlers;
mod middleware;
mod models;
mod schema;
mod validate;
use crate::handlers::classroom::get_classrooms;
use crate::middleware::auth::Auth as AuthMiddleware;
use actix_web::{web, App, HttpServer};
use handlers::{
    auth::login,
    classroom::{create_classroom, delete_classroom, update_classroom},
    user::get_profile,
};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("api")
                .wrap(AuthMiddleware)
                .service(web::scope("/login").route("", web::post().to(login)))
                .service(web::scope("/profile").route("", web::get().to(get_profile)))
                .service(
                    web::scope("/classroom")
                        .route("/{id}", web::put().to(update_classroom))
                        .route("/{id}", web::delete().to(delete_classroom))
                        .route("", web::get().to(get_classrooms))
                        .route("", web::post().to(create_classroom)),
                ),
        )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
