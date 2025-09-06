mod config;
mod core;
mod schema;
mod services;
mod models;
mod controllers;
mod prelude;
mod traits;
mod middleware;

use actix_web::middleware::from_fn;
use actix_web::{App, web, HttpServer};
use tracing::{info, error};
use crate::config::{create_pool, get_settings};
use crate::controllers::create_product_controller;
use crate::services::ProductService;
use crate::core::init_tracing;
use crate::middleware::{cors_middleware, request_logging};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize structured logging
    init_tracing();
    
    info!("🚀 Starting application initialization");

    let settings = match get_settings() {
        Ok(settings) => {
            info!("⚙️  Configuration loaded successfully");
            settings
        }
        Err(e) => {
            error!("❌ Failed to load configuration: {}", e);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, e));
        }
    };

    let pool = create_pool(&settings);
    info!("🗄️  Database connection pool created");

    let products_service = web::Data::new(ProductService::new(pool.clone()));
    info!("🛍️  Product service initialized");

    let bind_address = "0.0.0.0:8000";
    info!("🌐 Starting HTTP server on {}", bind_address);

    HttpServer::new(move || {
        App::new()
            .wrap(tracing_actix_web::TracingLogger::default())
            .wrap(from_fn(request_logging))
            .wrap(cors_middleware())
            .app_data(products_service.clone())
            .configure(create_product_controller)
    })
    .bind(bind_address)?
    .run()
    .await
}
