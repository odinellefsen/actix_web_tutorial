use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

type Todos = Arc<Mutex<Vec<String>>>;

#[derive(Serialize)]
struct TodosResponse {
    message: &'static str,
    todos: Vec<String>,
}

#[get("")]
async fn get_todos(todos: web::Data<Todos>) -> impl Responder {
    let todos_guard = todos.lock().unwrap();

    let response = TodosResponse {
        message: "These are your todos",
        todos: (*todos_guard).clone(),
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

    todos.push(payload.todo.clone());

    let response = TodosResponse {
        message: "Todo Created!",
        todos: (*todos).clone(),
    };

    HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let todos: Todos = Arc::new(Mutex::new(Vec::new()));

    HttpServer::new(move || {
        App::new().app_data(web::Data::new(todos.clone())).service(
            web::scope("/api")
                .service(web::scope("/todos").service(get_todos).service(create_todo)),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
