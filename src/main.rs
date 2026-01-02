use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};
use std::sync::{Arc, Mutex};

type Todos = Arc<Mutex<Vec<String>>>;

#[get("/")]
async fn get_todos(todos: web::Data<Todos>) -> impl Responder {
    let todos_guard = todos.lock().unwrap();
    HttpResponse::Ok().json(&*todos_guard)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let todos: Todos = Arc::new(Mutex::new(Vec::new()));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(todos.clone()))
            .service(web::scope("/api").service(get_todos))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
