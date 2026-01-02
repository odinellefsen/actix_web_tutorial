use actix_web::{App, HttpResponse, HttpServer, Responder, delete, get, post, web};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

type Todos = Arc<Mutex<Vec<Todo>>>;

#[derive(Clone, Serialize, Deserialize)]
struct Todo {
    id: usize,
    text: String,
}

#[derive(Serialize)]
struct TodosResponse {
    message: &'static str,
    todos: Vec<Todo>,
}

#[get("")]
async fn get_todos(todos: web::Data<Todos>) -> impl Responder {
    let todos_guard = todos.lock().unwrap();

    let response = TodosResponse {
        message: "These are your todos",
        todos: todos_guard.clone(),
    };

    HttpResponse::Ok().json(response)
}

#[derive(Deserialize)]
struct CreateTodoRequest {
    todo: String,
}

#[post("")]
async fn create_todo(
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

#[derive(Deserialize)]
struct DeleteTodoRequest {
    todo_id: usize,
}

#[delete("")]
async fn delete_todo(
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let todos: Todos = Arc::new(Mutex::new(Vec::new()));

    {
        let mut t = todos.lock().unwrap();

        t.push(Todo {
            id: 100,
            text: "Read Textbook 'Database System Concepts'".to_string(),
        });
        t.push(Todo {
            id: 101,
            text: "Do 15 consecutive pushups (NO BREAK!)".to_string(),
        });
        t.push(Todo {
            id: 102,
            text: "Brush Teeth!".to_string(),
        });
    }

    HttpServer::new(move || {
        App::new().app_data(web::Data::new(todos.clone())).service(
            web::scope("/api").service(
                web::scope("/todos")
                    .service(get_todos)
                    .service(create_todo)
                    .service(delete_todo),
            ),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
