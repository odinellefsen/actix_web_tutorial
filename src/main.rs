use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(web::scope("/api").service(index)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
