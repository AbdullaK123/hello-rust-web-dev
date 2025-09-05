mod config;
mod core;
mod schema;
mod services;
mod models;
mod controllers;
mod prelude;
mod traits;
mod middleware;

use actix_web::{App, web, HttpServer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tracing::{info, error};
use crate::config::{create_pool, get_settings};
use crate::controllers::create_product_controller;
use crate::services::ProductService;
use crate::middleware::{RequestLogging, ErrorLogging};

fn init_tracing() {
    // Determine if we're in production or development
    let is_production = std::env::var("RUST_ENV")
        .unwrap_or_else(|_| "development".to_string())
        .to_lowercase() == "production";

    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "info,backend=debug,sqlx=info,actix_web=info".into());

    if is_production {
        // Production: JSON structured logging
        tracing_subscriber::registry()
            .with(env_filter)
            .with(
                tracing_subscriber::fmt::layer()
                    .with_target(true)
                    .with_thread_ids(true)
                    .with_thread_names(true)
                    .with_file(true)
                    .with_line_number(true)
                    .json()
            )
            .init();
        
        info!("Tracing initialized with structured JSON logging (Production Mode)");
    } else {
        // Development: Beautiful colored logging
        tracing_subscriber::registry()
            .with(env_filter)
            .with(
                tracing_subscriber::fmt::layer()
                    .with_target(true)
                    .with_thread_ids(false)  // Less noise in dev
                    .with_thread_names(false)
                    .with_file(true)
                    .with_line_number(true)
                    .with_ansi(true)  // Enable colors
                    .pretty()  // Pretty formatting
                    .with_timer(tracing_subscriber::fmt::time::ChronoUtc::rfc_3339())
            )
            .init();
        
        info!("🎨 Tracing initialized with beautiful colored logging (Development Mode)");
    }
}

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
            // Add comprehensive logging middleware
            .wrap(RequestLogging)
            .wrap(ErrorLogging)
            // Add tracing integration for actix-web
            .wrap(tracing_actix_web::TracingLogger::default())
            .app_data(products_service.clone())
            .configure(create_product_controller)
    })
    .bind(bind_address)?
    .run()
    .await
}
