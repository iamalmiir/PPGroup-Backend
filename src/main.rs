use actix_web::{middleware::Logger, web, App, HttpServer};

mod actors;
mod database;
mod routes;
mod services;

use dotenv::dotenv;
use sea_orm::Database;
use services::realtor::realtors_services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log::info!("Starting database...");
    let db = Database::connect(std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
        .await
        .expect("Failed to connect to the database");
    log::info!("Starting server...");
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(db.clone()))
            .service(web::scope("/api/v1").service(realtors_services()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
