mod config;
mod core;
mod schema;
mod services;
mod models;
mod controllers;
mod prelude;
mod traits;

use actix_web::{App, web, HttpServer};
use actix_web::middleware::Logger;
use crate::config::{create_pool, get_settings};
use crate::controllers::create_product_controller;
use crate::services::ProductService;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let settings = get_settings().unwrap();
    let pool = create_pool(&settings);
    let products_service = web::Data::new(ProductService::new(pool.clone()));

    HttpServer::new( move || {
        App::new()
            .wrap(Logger::default())
            .app_data(products_service.clone())
            .configure(create_product_controller)
    })
        .bind("0.0.0.0:8000")?
        .run()
        .await
}
