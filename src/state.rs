use std::sync::{Mutex, Arc};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Todo {
    pub title: String,
    pub completed: bool
}

#[derive(Debug, Clone, Default)]
pub struct AppState {
    pub todos: Arc<Mutex<Vec<Todo>>>
}

impl AppState {
    pub fn new() -> AppState {
        return AppState {
            todos: Arc::new(Mutex::new(Vec::new()))
        };
    }
}
