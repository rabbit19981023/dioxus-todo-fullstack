#[cfg(feature = "server")]
use crate::database::get_db;

use dioxus::{CapturedError, prelude::*};
use shared::models::Todo;

#[get("/api/todos")]
pub async fn get_all_todos() -> Result<Vec<Todo>> {
    let db = get_db().await;

    let result = sqlx::query_as!(Todo, "SELECT * FROM todos")
        .fetch_all(db)
        .await?;

    Ok(result)
}

#[get("/api/todos/:id")]
pub async fn find_todo(id: i32) -> Result<Todo> {
    let db = get_db().await;

    let result = sqlx::query_as!(Todo, "SELECT * FROM todos WHERE id = $1", id)
        .fetch_one(db)
        .await?;

    Ok(result)
}

#[post("/api/todos")]
pub async fn create_todo(title: String) -> Result<i32> {
    let db = get_db().await;

    let row = sqlx::query!("INSERT INTO todos (title) VALUES ($1) RETURNING id", title)
        .fetch_one(db)
        .await?;

    Ok(row.id)
}

#[delete("/api/todos/:id")]
pub async fn delete_todo(id: i32) -> Result<()> {
    let db = get_db().await;

    let result = sqlx::query!("DELETE FROM todos WHERE id = $1", id)
        .execute(db)
        .await?;

    match result.rows_affected() {
        0 => Err(CapturedError::msg("No rows deleted")),
        _ => Ok(()),
    }
}

#[put("/api/todos/:id")]
pub async fn update_todo(id: i32, title: String, completed: bool) -> Result<()> {
    let db = get_db().await;

    let result = sqlx::query!(
        "UPDATE todos SET title = $1, completed = $2 WHERE id = $3",
        title,
        completed,
        id
    )
    .execute(db)
    .await?;

    match result.rows_affected() {
        0 => Err(CapturedError::msg("No rows updated")),
        _ => Ok(()),
    }
}
