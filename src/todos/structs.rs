// src/todos.rs
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: usize,
    pub text: String,
}

#[derive(Serialize)]
pub struct TodosResponse {
    pub message: &'static str,
    pub todos: Vec<Todo>,
}

#[derive(Deserialize)]
pub struct CreateTodoRequest {
    pub todo: String,
}

#[derive(Deserialize)]
pub struct DeleteTodoRequest {
    pub todo_id: usize,
}
