mod models;
use crate::{error::Result, routing::api::models::get_todo};
use axum::{Router, routing::get, Json, extract::{State, Path}, debug_handler};

use crate::{state::AppState, routing::api::models::{Todo, CreateTodo, create_todo}};


#[debug_handler]
async fn new_todo(State(app_state): State<AppState>,
                     Json(new_todo): Json<CreateTodo>) -> Result<Json<Todo>> {
    Ok(Json(create_todo(app_state.pool, new_todo).await?))
}

#[debug_handler]
async fn show_todo(State(app_state): State<AppState>,
                     Path(todo_id): Path<i32>) -> Result<Json<Option<Todo>>> {
    Ok(Json(get_todo(app_state.pool, todo_id).await?))
}
#[debug_handler]
async fn list_todos(State(app_state): State<AppState>) -> Result<Json<Vec<Todo>>> { 
    let todos = models::list_todos(app_state.pool.clone()).await?;
    Ok(Json(todos))
}

pub fn create_router() -> Router<AppState> {
    Router::new()
          .route("/", get(list_todos).post(new_todo))
          .route("/:id", get(show_todo))
}
