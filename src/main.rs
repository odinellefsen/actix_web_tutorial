use actix_web::{App, HttpServer, web};
use std::sync::{Arc, Mutex};

mod todos;
use todos::handlers::{create_todo, delete_todo, get_todos};
use todos::structs::Todo;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let shared_todos = web::Data::new(Arc::new(Mutex::new(Vec::<Todo>::new())));

    HttpServer::new(move || {
        App::new().app_data(shared_todos.clone()).service(
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
