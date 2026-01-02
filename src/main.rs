use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};
use serde::Serialize;
use std::sync::{Arc, Mutex};

type Todos = Arc<Mutex<Vec<String>>>;

#[derive(Serialize)]
struct GetTodosEndpointResponse {
    message: String,
    todos: Vec<String>,
}

#[get("/")]
async fn get_todos(todos: web::Data<Todos>) -> impl Responder {
    let todos_guard = todos.lock().unwrap();

    let response = GetTodosEndpointResponse {
        message: "These are your todos".to_string(),
        todos: (*todos_guard).clone(),
    };

    HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let todos: Todos = Arc::new(Mutex::new(Vec::new()));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(todos.clone()))
            .service(web::scope("/api").service(web::scope("/todos").service(get_todos)))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
