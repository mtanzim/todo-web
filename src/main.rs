use actix_web::{web, App, HttpServer};
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePool;
use std::env;

pub mod handlers;

pub struct AppStateWithDBPool {
    pool: SqlitePool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = SqlitePool::connect(&env::var("DATABASE_URL").expect("set up database url env"))
        .await
        .expect("cannot create db pool");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppStateWithDBPool { pool: pool.clone() }))
            .route("/api/add", web::post().to(crate::handlers::create))
            .route("/api/list", web::get().to(crate::handlers::list))
            .route("/api/done/{id}", web::patch().to(crate::handlers::complete))
            .route(
                "/api/destroy/{id}",
                web::delete().to(crate::handlers::delete),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[derive(Debug, Deserialize, Serialize)]
struct ErrMsg {
    message: String,
}
