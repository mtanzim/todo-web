use crate::db::{complete_todo, create_todo, delete_todo, list_todo};
use crate::models::NewTask;
use crate::AppStateWithDBPool;
use actix_files::NamedFile;
use actix_web::{error, web, Responder, Result};
use std::sync::Arc;

pub async fn serve_index() -> Result<NamedFile> {
    Ok(NamedFile::open("public/index.html").expect("could not open html file"))
}

pub async fn create(
    data: web::Data<AppStateWithDBPool>,
    new_task: web::Json<NewTask>,
) -> Result<String> {
    println!("{:?}", new_task);
    let added_id = create_todo(Arc::clone(&data.client), new_task.into_inner()).await;
    match added_id {
        Ok(id) => Ok(format!("Adding {}!", id)),
        Err(err) => Err(error::ErrorBadRequest(err)),
    }
}

pub async fn complete(data: web::Data<AppStateWithDBPool>, path: web::Path<u32>) -> Result<String> {
    let task_id = path.into_inner();
    let rows_updated = complete_todo(Arc::clone(&data.client), task_id).await;
    match rows_updated {
        Ok(rows) => Ok(format!("Updated {} rows", rows)),
        Err(err) => Err(error::ErrorBadRequest(err)),
    }
}

pub async fn delete(data: web::Data<AppStateWithDBPool>, path: web::Path<u32>) -> Result<String> {
    let task_id = path.into_inner();
    let rows_updated = delete_todo(Arc::clone(&data.client), task_id).await;
    match rows_updated {
        Ok(rows) => Ok(format!("Deleted {} rows!", rows)),
        Err(err) => Err(error::ErrorBadRequest(err)),
    }
}

pub async fn list(data: web::Data<AppStateWithDBPool>) -> Result<impl Responder> {
    let tasks_res = list_todo(Arc::clone(&data.client)).await;
    match tasks_res {
        Ok(tasks) => return Ok(web::Json(tasks)),
        Err(err) => Err(error::ErrorBadRequest(err)),
    }
}
