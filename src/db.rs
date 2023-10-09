use crate::models::NewTask;
use crate::models::Task;
use libsql_client::{args, Client, Statement, Value};
use std::sync::{Arc, Mutex};

pub async fn create_todo(client: Arc<Mutex<Client>>, t: NewTask) -> anyhow::Result<i64> {
    let client = client.lock().unwrap();
    let rs = client
        .execute(Statement::with_args(
            "INSERT INTO tasks (name) VALUES (?)",
            args!(t.name),
        ))
        .await?;

    Ok(rs.last_insert_rowid.unwrap_or(-1))
}

pub async fn complete_todo(client: Arc<Mutex<Client>>, id: u32) -> anyhow::Result<u64> {
    let client = client.lock().unwrap();
    let rs = client
        .execute(Statement::with_args(
            "UPDATE tasks SET completed=1 WHERE id=(?);",
            args!(id),
        ))
        .await?;
    Ok(rs.rows_affected)
}

pub async fn delete_todo(client: Arc<Mutex<Client>>, id: u32) -> anyhow::Result<u64> {
    let client = client.lock().unwrap();
    let rs = client
        .execute(Statement::with_args(
            "DELETE FROM tasks WHERE id=(?);",
            args!(id),
        ))
        .await?;
    Ok(rs.rows_affected)
}

pub async fn list_todo(client: Arc<Mutex<Client>>) -> anyhow::Result<Vec<Task>> {
    let db = client.lock().unwrap();
    let rs = db.execute("select * from tasks").await?;

    let mut tasks = vec![];
    for r in rs.rows {
        let v = r.value_map;
        let name = v.get("name");
        let completed = v.get("completed");
        let id = v.get("id");
        match (id, name, completed) {
            (
                Some(Value::Integer { value: id_val }),
                Some(Value::Text { value: name_val }),
                Some(Value::Integer {
                    value: completed_val,
                }),
            ) => tasks.push(Task {
                completed: completed_val.clone(),
                id: id_val.clone(),
                name: name_val.clone(),
            }),
            _ => {}
        }
    }

    Ok(tasks)
}
