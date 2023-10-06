use crate::handlers::{complete, create, delete, list, serve_index};

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePool;
use std::env;

pub mod db;
pub mod handlers;
pub mod models;

pub struct AppStateWithDBPool {
    pool: SqlitePool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let pool = SqlitePool::connect(&env::var("DATABASE_URL").expect("set up database url env"))
        .await
        .expect("cannot create db pool");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppStateWithDBPool { pool: pool.clone() }))
            .route("/", actix_web::web::get().to(serve_index))
            .route("/api/add", web::post().to(create))
            .route("/api/list", web::get().to(list))
            .route("/api/done/{id}", web::patch().to(complete))
            .route("/api/destroy/{id}", web::delete().to(delete))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[derive(Debug, Deserialize, Serialize)]
struct ErrMsg {
    message: String,
}
