use crate::prelude::*;
use crate::models::{NewCompleteProduct, ProductFilters, ProductUpdates};
use crate::services::ProductService;

pub async fn create_product(
    service: web::Data<ProductService>,
    payload: web::Json<NewCompleteProduct>
) -> ActixResult<HttpResponse> {
    service.create_product(payload.into_inner()).to_response()
}

pub async fn get_product_by_id(
    service: web::Data<ProductService>,
    id: web::Path<i32>
) -> ActixResult<HttpResponse> {
    service.get_product_by_id(id.into_inner()).to_response()
}

pub async fn get_products(
    service: web::Data<ProductService>,
    filters: web::Query<ProductFilters>
) -> ActixResult<HttpResponse> {
    let filters = if filters.is_empty() { None } else { Some(filters.into_inner()) };
    service.get_products(filters).to_response()
}

pub async fn update_product(
    service: web::Data<ProductService>,
    id: web::Path<i32>,
    updates: web::Json<ProductUpdates> 
) -> ActixResult<HttpResponse> {
    service.update_product(id.into_inner(), updates.into_inner()).to_response()
}

pub async fn delete_product(
    service: web::Data<ProductService>,
    id: web::Path<i32>
) -> ActixResult<HttpResponse> {
    match service.delete_product(id.into_inner()) {
        Ok(true) => Ok(HttpResponse::NoContent().finish()),
        Ok(false) => Ok(HttpResponse::NotFound().json(ErrorResponse {
            error: "Product not found".to_string()
        })),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ErrorResponse {
            error: err.to_string()
        }))
    }
}

// Orchestrate the posts controller
pub fn create_product_controller(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/products")
        .route("", web::post().to(create_product))
        .route("", web::get().to(get_products))
        .route("/{id}", web::put().to(update_product))
        .route("/{id}", web::delete().to(delete_product))
        .route("/{id}", web::get().to(get_product_by_id))
    );
}