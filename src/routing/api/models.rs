use sqlx::{PgPool, FromRow};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, FromRow )]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub completed: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

pub async fn list_todos(pool: PgPool) -> Vec<Todo> {
    let recs = sqlx::query_as!(Todo, "SELECT * FROM todos")
        .fetch_all(&pool).await.unwrap();
    recs
}

#[derive(Deserialize)]
pub struct CreateTodo {
    pub title: String
}

pub async fn create_todo(pool: PgPool, create_todo: CreateTodo) -> Todo {
    sqlx::query_as!(Todo, "INSERT INTO todos (title) VALUES ($1) returning *", create_todo.title)
        .fetch_one(&pool).await.unwrap()
}
