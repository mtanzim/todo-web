use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use sqlx::sqlite::SqlitePool;
use std::env;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = SqlitePool::connect(&env::var("DATABASE_URL").expect("set up database url env"))
        .await
        .expect("cannot create db pool");
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

// DB
#[derive(Debug)]
struct Task {
    id: i32,
    name: String,
    completed: bool,
}

#[derive(Debug)]
struct NewTask {
    name: String,
    completed: bool,
}

async fn add_todo(pool: &SqlitePool, t: NewTask) -> anyhow::Result<i64> {
    let mut conn = pool.acquire().await?;

    // Insert the task, then obtain the ID of this row
    let id = sqlx::query!(
        r#"
        INSERT INTO tasks (name, completed) VALUES (?1, ?2);
      "#,
        t.name,
        t.completed,
    )
    .execute(&mut *conn)
    .await?
    .last_insert_rowid();

    Ok(id)
}
