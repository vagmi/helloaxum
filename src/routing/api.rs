mod models;

use axum::{Router, routing::get, Json, extract::State, debug_handler};


use serde::Deserialize;
use crate::{state::{AppState}, routing::api::models::{Todo, CreateTodo, create_todo}};


#[debug_handler]
async fn new_todo(State(app_state): State<AppState>,
                     Json(new_todo): Json<CreateTodo>) -> Json<Todo> {
    Json(create_todo(app_state.pool, new_todo).await)
}

#[debug_handler]
async fn list_todos(State(app_state): State<AppState>) -> Json<Vec<Todo>> { 
    let todos = models::list_todos(app_state.pool.clone()).await;
    Json(todos)
}

pub fn create_router() -> Router<AppState> {
    Router::new()
          .route("/", get(list_todos).post(new_todo))
}
