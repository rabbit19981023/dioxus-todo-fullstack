#[cfg(feature = "server")]
use crate::database::get_db;

use crate::models::Todo;

use dioxus::prelude::*;

#[server]
pub async fn get_all_todos() -> Result<Vec<Todo>, ServerFnError> {
    let db = get_db().await;

    let result = sqlx::query_as!(Todo, "SELECT * FROM todos")
        .fetch_all(db)
        .await?;

    Ok(result)
}

#[server]
pub async fn find_todo(id: i32) -> Result<Todo, ServerFnError> {
    let db = get_db().await;

    let result = sqlx::query_as!(Todo, "SELECT * FROM todos WHERE id = $1", id)
        .fetch_one(db)
        .await?;

    Ok(result)
}

#[server]
pub async fn create_todo(title: String) -> Result<i32, ServerFnError> {
    let db = get_db().await;

    let row = sqlx::query!("INSERT INTO todos (title) VALUES ($1) RETURNING id", title)
        .fetch_one(db)
        .await?;

    Ok(row.id)
}

#[server]
pub async fn delete_todo(id: i32) -> Result<(), ServerFnError> {
    let db = get_db().await;

    let result = sqlx::query!("DELETE FROM todos WHERE id = $1", id)
        .execute(db)
        .await?;

    match result.rows_affected() {
        0 => Err(ServerFnError::Request("No rows deleted".to_string())),
        _ => Ok(()),
    }
}

#[server]
pub async fn update_todo(id: i32, title: String, completed: bool) -> Result<(), ServerFnError> {
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
        0 => Err(ServerFnError::Request("No rows updated".to_string())),
        _ => Ok(()),
    }
}
