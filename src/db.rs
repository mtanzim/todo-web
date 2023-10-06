use crate::models::NewTask;
use crate::models::Task;
use sqlx::sqlite::SqlitePool;

pub async fn create_todo(pool: &SqlitePool, t: NewTask) -> anyhow::Result<i64> {
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

pub async fn complete_todo(pool: &SqlitePool, id: u32) -> anyhow::Result<u64> {
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

pub async fn delete_todo(pool: &SqlitePool, id: u32) -> anyhow::Result<u64> {
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

pub async fn list_todo(pool: &SqlitePool) -> anyhow::Result<Vec<Task>> {
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
