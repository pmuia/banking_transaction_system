use actix_web::{App, HttpServer, web};
use sqlx::postgres::PgPool; // Instead of `sqlx::PgPool`
use dotenv::dotenv;

mod domain;
mod repository;
mod application;
mod presentation;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
    // Create a shared database connection pool
    let pool = PgPool::connect(&db_url).await.expect("Failed to connect to DB");
    let pool_data = web::Data::new(pool); // Wrap in `web::Data`

    HttpServer::new(move || {
        App::new()
            .app_data(pool_data.clone()) // Clone the reference
            .configure(presentation::routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
