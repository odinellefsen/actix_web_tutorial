// src/handlers.rs
use actix_web::{HttpResponse, Responder, delete, get, post, web};
use rand::Rng;

use crate::todos::structs::{CreateTodoRequest, DeleteTodoRequest, Todo, TodosResponse};

type Todos = std::sync::Arc<std::sync::Mutex<Vec<Todo>>>;

#[get("")]
pub async fn get_todos(todos: web::Data<Todos>) -> impl Responder {
    let todos_guard = todos.lock().unwrap();

    let response = TodosResponse {
        message: "These are your todos",
        todos: todos_guard.clone(),
    };

    HttpResponse::Ok().json(response)
}

#[post("")]
pub async fn create_todo(
    payload: web::Json<CreateTodoRequest>,
    todos: web::Data<Todos>,
) -> impl Responder {
    let mut todos = todos.lock().unwrap();

    let mut rng = rand::thread_rng();
    let id = rng.gen_range(100..=999);

    let new_todo = Todo {
        id,
        text: payload.todo.clone(),
    };

    todos.push(new_todo);

    let response = TodosResponse {
        message: "Todo Created!",
        todos: todos.clone(),
    };

    HttpResponse::Ok().json(response)
}

#[delete("")]
pub async fn delete_todo(
    payload: web::Json<DeleteTodoRequest>,
    todos: web::Data<Todos>,
) -> impl Responder {
    let mut todos = todos.lock().unwrap();

    match todos.iter().position(|t| t.id == payload.todo_id) {
        Some(index) => {
            todos.remove(index);
            let response = TodosResponse {
                message: "Todo deleted successfully",
                todos: todos.clone(),
            };
            HttpResponse::Ok().json(response)
        }
        None => HttpResponse::NotFound().json("Todo not found"),
    }
}
