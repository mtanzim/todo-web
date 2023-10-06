use actix_web::{error, web, Responder, Result};
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePool;

use crate::AppStateWithDBPool;

pub async fn create(
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

pub async fn complete(data: web::Data<AppStateWithDBPool>, path: web::Path<u32>) -> Result<String> {
    let task_id = path.into_inner();
    let rows_updated = complete_todo(&data.pool, task_id).await;
    match rows_updated {
        Ok(rows) => Ok(format!("Updated {} rows", rows)),
        Err(err) => Err(error::ErrorBadRequest(err)),
    }
}

pub async fn delete(data: web::Data<AppStateWithDBPool>, path: web::Path<u32>) -> Result<String> {
    let task_id = path.into_inner();
    let rows_updated = delete_todo(&data.pool, task_id).await;
    match rows_updated {
        Ok(rows) => Ok(format!("Deleted {} rows!", rows)),
        Err(err) => Err(error::ErrorBadRequest(err)),
    }
}

pub async fn list(data: web::Data<AppStateWithDBPool>) -> Result<impl Responder> {
    let tasks_res = list_todo(&data.pool).await;
    match tasks_res {
        Ok(tasks) => return Ok(web::Json(tasks)),
        Err(err) => Err(error::ErrorBadRequest(err)),
    }
}

// DB
#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    id: i64,
    name: String,
    completed: i64,
}

#[derive(Debug, Deserialize)]
pub struct NewTask {
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

async fn complete_todo(pool: &SqlitePool, id: u32) -> anyhow::Result<u64> {
    let mut conn = pool.acquire().await?;

    // Insert the task, then obtain the ID of this row
    let rows_updated = sqlx::query!(
        r#"
        UPDATE tasks SET completed=1 WHERE id=(?1);
      "#,
        id,
    )
    .execute(&mut *conn)
    .await?
    .rows_affected();

    Ok(rows_updated)
}

async fn delete_todo(pool: &SqlitePool, id: u32) -> anyhow::Result<u64> {
    let mut conn = pool.acquire().await?;

    // Insert the task, then obtain the ID of this row
    let rows_updated = sqlx::query!(
        r#"
        DELETE FROM tasks WHERE id=(?1);
      "#,
        id,
    )
    .execute(&mut *conn)
    .await?
    .rows_affected();

    Ok(rows_updated)
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
