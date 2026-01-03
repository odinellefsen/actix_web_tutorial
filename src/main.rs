use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use std::sync::{Arc, Mutex};

mod middleware;
mod todos;
use todos::handlers::{create_todo, delete_todo, get_todos};
use todos::structs::Todo;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let shared_todos = web::Data::new(Arc::new(Mutex::new(Vec::<Todo>::new())));

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin_fn(|origin, _req_head| {
                        origin.as_bytes().starts_with(b"http://localhost")
                    })
                    .allowed_methods(vec!["GET", "POST", "DELETE"])
                    .allowed_headers(vec![
                        actix_web::http::header::AUTHORIZATION,
                        actix_web::http::header::ACCEPT,
                    ])
                    .allowed_header(actix_web::http::header::CONTENT_TYPE)
                    .max_age(3600),
            )
            .app_data(shared_todos.clone())
            .service(
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
