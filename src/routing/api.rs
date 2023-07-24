use axum::{Router, routing::get, Json, extract::State, debug_handler};

use serde::Deserialize;
use crate::state::{AppState, Todo};

#[derive(Deserialize)]
struct CreateTodo {
    title: String
}

#[debug_handler]
async fn create_todo(State(app_state): State<AppState>,
                     Json(create_todo): Json<CreateTodo>) -> Json<Todo> {
    let mut todos = app_state.todos.lock().unwrap();
    let new_todo = Todo {
        title: create_todo.title,
        completed: false
    };
    todos.push(new_todo.clone());
    Json(new_todo)
}

#[debug_handler]
async fn list_todos(State(app_state): State<AppState>) -> Json<Vec<Todo>> { 
    let todos = app_state.todos.lock().unwrap();
    Json(todos.clone())
}

pub fn create_router() -> Router<AppState> {
    Router::new()
          .route("/", get(list_todos).post(create_todo))
}
