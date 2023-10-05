use actix_web::{error, web, App, HttpServer, Responder, Result};
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePool;
use std::env;

async fn create(
    data: web::Data<AppStateWithDBPool>,
    new_task: web::Json<NewTask>,
) -> Result<String> {
    println!("{:?}", new_task);
    let added_id = create_todo(&data.pool, new_task.into_inner()).await;
    match added_id {
        Ok(id) => Ok(format!("Adding {}!", id)),
        Err(err) => Err(error::ErrorBadRequest(err)),
    }
}

async fn list(data: web::Data<AppStateWithDBPool>) -> Result<impl Responder> {
    let tasks_res = list_todo(&data.pool).await;
    match tasks_res {
        Ok(tasks) => return Ok(web::Json(tasks)),
        Err(err) => Err(error::ErrorBadRequest(err)),
    }
}

struct AppStateWithDBPool {
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
            .route("/api/add", web::post().to(create))
            .route("/api/list", web::get().to(list))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[derive(Debug, Deserialize, Serialize)]
struct ErrMsg {
    message: String,
}
// DB
#[derive(Debug, Deserialize, Serialize)]
struct Task {
    id: i64,
    name: String,
    completed: i64,
}

#[derive(Debug, Deserialize)]
struct NewTask {
    name: String,
}

async fn create_todo(pool: &SqlitePool, t: NewTask) -> anyhow::Result<i64> {
    let mut conn = pool.acquire().await?;

    // Insert the task, then obtain the ID of this row
    let id = sqlx::query!(
        r#"
        INSERT INTO tasks (name) VALUES (?1);
      "#,
        t.name,
    )
    .execute(&mut *conn)
    .await?
    .last_insert_rowid();

    Ok(id)
}

async fn list_todo(pool: &SqlitePool) -> anyhow::Result<Vec<Task>> {
    // Insert the task, then obtain the ID of this row
    let tasks = sqlx::query_as!(
        Task,
        "
        SELECT id, name, completed FROM tasks;
      ",
    )
    .fetch_all(pool)
    .await?;
    Ok(tasks)
}
