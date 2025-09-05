use crate::prelude::*;
use crate::models::{NewCompleteProduct, ProductFilters, ProductUpdates};
use crate::services::ProductService;
use uuid::Uuid;
use tracing::{info, warn, error, instrument};

#[instrument(
    name = "create_product_handler",
    skip(service, payload),
    fields(
        product_name = %payload.product.name,
        product_cost = %payload.product.cost,
        variants_count = payload.variants.len()
    )
)]
pub async fn create_product(
    service: web::Data<ProductService>,
    payload: web::Json<NewCompleteProduct>,
) -> ActixResult<HttpResponse> {
    let product_data = payload.into_inner();
    
    info!(
        product_name = %product_data.product.name,
        product_cost = %product_data.product.cost,
        variants_count = product_data.variants.len(),
        "🆕 Creating new product"
    );

    let result = service.create_product(product_data).to_response();
    
    match &result {
        Ok(response) if response.status().is_success() => {
            info!(
                status = response.status().as_u16(),
                "🎉 Product created successfully"
            );
        }
        Ok(response) => {
            warn!(
                status = response.status().as_u16(),
                "Product creation failed with client error"
            );
        }
        Err(e) => {
            error!(
                error = %e,
                "Product creation failed with server error"
            );
        }
    }
    
    result
}

#[instrument(
    name = "get_product_by_id_handler",
    skip(service),
    fields(
        product_id = %id.as_ref()
    )
)]
pub async fn get_product_by_id(
    service: web::Data<ProductService>,
    id: web::Path<Uuid>,
) -> ActixResult<HttpResponse> {
    let product_id = id.into_inner();
    
    info!(
        product_id = %product_id,
        "Fetching product by ID"
    );

    let result = service.get_product_by_id(product_id).to_response();
    
    match &result {
        Ok(response) if response.status().is_success() => {
            info!(
                product_id = %product_id,
                status = response.status().as_u16(),
                "Product retrieved successfully"
            );
        }
        Ok(response) if response.status() == 404 => {
            info!(
                product_id = %product_id,
                status = response.status().as_u16(),
                "Product not found"
            );
        }
        Ok(response) => {
            warn!(
                product_id = %product_id,
                status = response.status().as_u16(),
                "Product retrieval failed"
            );
        }
        Err(e) => {
            error!(
                product_id = %product_id,
                error = %e,
                "Product retrieval failed with server error"
            );
        }
    }
    
    result
}

#[instrument(
    name = "get_products_handler",
    skip(service, filters),
    fields(
        has_filters = !filters.is_empty(),
        filter_name = filters.name.as_deref().unwrap_or("none"),
        filter_active = filters.is_active
    )
)]
pub async fn get_products(
    service: web::Data<ProductService>,
    filters: web::Query<ProductFilters>
) -> ActixResult<HttpResponse> {
    let has_filters = !filters.is_empty();
    
    info!(
        has_filters = has_filters,
        filter_name = filters.name.as_deref().unwrap_or("none"),
        filter_active = filters.is_active,
        "🔍 Fetching products with filters"
    );

    let filters = if filters.is_empty() { None } else { Some(filters.into_inner()) };
    let result = service.get_products(filters).to_response();
    
    match &result {
        Ok(response) if response.status().is_success() => {
            info!(
                status = response.status().as_u16(),
                "📦 Products retrieved successfully"
            );
        }
        Ok(response) => {
            warn!(
                status = response.status().as_u16(),
                "Products retrieval failed"
            );
        }
        Err(e) => {
            error!(
                error = %e,
                "Products retrieval failed with server error"
            );
        }
    }
    
    result
}

#[instrument(
    name = "update_product_handler",
    skip(service, updates),
    fields(
        product_id = %id.as_ref(),
        update_name = updates.name.is_some(),
        update_cost = updates.cost.is_some(),
        update_active = updates.active.is_some()
    )
)]
pub async fn update_product(
    service: web::Data<ProductService>,
    id: web::Path<Uuid>,
    updates: web::Json<ProductUpdates> 
) -> ActixResult<HttpResponse> {
    let product_id = id.into_inner();
    let update_data = updates.into_inner();
    
    info!(
        product_id = %product_id,
        update_name = update_data.name.is_some(),
        update_cost = update_data.cost.is_some(),
        update_active = update_data.active.is_some(),
        "Updating product"
    );

    let result = service.update_product(product_id, update_data).to_response();
    
    match &result {
        Ok(response) if response.status().is_success() => {
            info!(
                product_id = %product_id,
                status = response.status().as_u16(),
                "Product updated successfully"
            );
        }
        Ok(response) if response.status() == 404 => {
            info!(
                product_id = %product_id,
                status = response.status().as_u16(),
                "Product not found for update"
            );
        }
        Ok(response) => {
            warn!(
                product_id = %product_id,
                status = response.status().as_u16(),
                "Product update failed"
            );
        }
        Err(e) => {
            error!(
                product_id = %product_id,
                error = %e,
                "Product update failed with server error"
            );
        }
    }
    
    result
}

#[instrument(
    name = "delete_product_handler",
    skip(service),
    fields(
        product_id = %id.as_ref()
    )
)]
pub async fn delete_product(
    service: web::Data<ProductService>,
    id: web::Path<Uuid>
) -> ActixResult<HttpResponse> {
    let product_id = id.into_inner();
    
    info!(
        product_id = %product_id,
        "Deleting product"
    );

    match service.delete_product(product_id) {
        Ok(true) => {
            info!(
                product_id = %product_id,
                "Product deleted successfully"
            );
            Ok(HttpResponse::NoContent().finish())
        },
        Ok(false) => {
            info!(
                product_id = %product_id,
                "Product not found for deletion"
            );
            Ok(HttpResponse::NotFound().json(ErrorResponse {
                error: "Product not found".to_string()
            }))
        },
        Err(err) => {
            error!(
                product_id = %product_id,
                error = %err,
                "Product deletion failed with server error"
            );
            Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                error: err.to_string()
            }))
        }
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