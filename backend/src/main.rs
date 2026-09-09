pub mod db;
pub mod routes;
pub mod utils;

use actix_web::{App, HttpResponse, HttpServer, Responder, post, web};
use sqlx::{PgPool, postgres::PgPoolOptions};
use tracing_actix_web::TracingLogger;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

use crate::routes::login::login;

#[derive(Clone)]
pub struct AppState {
        db: PgPool,
}

#[post("/")]
async fn hello() -> impl Responder {
        HttpResponse::Ok().body("Hello World!")
}

#[actix_web::main] // Starts the Actix async runtime
async fn main() -> std::io::Result<()> {
        // Initialize dotenv
        dotenvy::dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        // Initialize logging system
        tracing_subscriber::registry()
                .with(
                        EnvFilter::try_from_default_env()
                                .unwrap_or_else(|_| EnvFilter::new("info")),
                )
                .with(tracing_subscriber::fmt::layer().json().flatten_event(true))
                .init();

        // Initialize PostgreSQL connection
        let pool = PgPoolOptions::new()
                .max_connections(5)
                .connect(&database_url)
                .await
                .expect("Failed to create pool");

        // Start the HTTP server
        HttpServer::new(move || {
                let app_state: AppState = AppState { db: pool.clone() };
                App::new()
                        .wrap(TracingLogger::default())
                        .app_data(web::Data::new(app_state.clone()))
                        .service(hello)
                        .service(login)
        })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
