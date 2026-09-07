use actix_web::{App, HttpResponse, HttpServer, Responder, post};

#[post("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[actix_web::main] // Starts the Actix async runtime
async fn main() -> std::io::Result<()> {
    // Start the HTTP server
    HttpServer::new(|| {
        App::new().service(hello) // Register the macro-routed handlers
    })
    .bind(("127.0.0.1", 8080))? // Bind to port 8080
    .run()
    .await
}
