use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use dotenvy::dotenv;
use std::env;
use std::sync::{Arc, Mutex};

mod middleware;
mod todos;
use middleware::SayHi;
use todos::handlers::{create_todo, delete_todo, get_todos};
use todos::structs::Todo;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load .env file
    dotenv().ok();

    // Get database URL from environment
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Create connection pool
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    println!("✅ Successfully connected to PostgreSQL database!");

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
            .wrap(SayHi)
            .app_data(shared_todos.clone())
            .app_data(web::Data::new(pool.clone()))
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
