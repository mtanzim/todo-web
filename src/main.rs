use crate::handlers::{complete, create, delete, list};

use actix_files as fs;
use actix_web::{middleware, web, App, HttpServer};
use dotenv::dotenv;
use libsql_client::Client;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

pub mod db;
pub mod handlers;
pub mod models;

pub struct AppStateWithDBPool {
    client: Arc<Mutex<Client>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let client = Arc::new(Mutex::new(libsql_client::Client::from_env().await.unwrap()));
    let port = 8080;
    println!("serving on {}", port);
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppStateWithDBPool {
                client: client.to_owned(),
            }))
            .wrap(middleware::Logger::default())
            .route("/api/add", web::post().to(create))
            .route("/api/list", web::get().to(list))
            .route("/api/done/{id}", web::patch().to(complete))
            .route("/api/destroy/{id}", web::delete().to(delete))
            .service(
                fs::Files::new("/", "./public")
                    .index_file("index.html")
                    .use_last_modified(true),
            )
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}

#[derive(Debug, Deserialize, Serialize)]
struct ErrMsg {
    message: String,
}
