use diesel::prelude::*;
use diesel::{ExpressionMethods, RunQueryDsl};
use crate::config::{DbConnection, DbPool};
use crate::models::{NewCompleteProduct, NewProductVariant, Product, ProductFilters, ProductUpdates};
use uuid::Uuid;
use crate::schema::{product_variants, products, variants};
use anyhow::Result;
use tracing::{info, warn, error, instrument, debug};

pub struct ProductService {
    pub pool: DbPool
}

impl ProductService {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    } 

    #[instrument(
        name = "db_get_connection",
        skip(self),
        fields(service = "ProductService")
    )]
    fn get_connection(&self) -> Result<DbConnection> {
        debug!("Acquiring database connection from pool");
        match self.pool.get() {
            Ok(conn) => {
                debug!("Database connection acquired successfully");
                Ok(conn)
            }
            Err(e) => {
                error!(error = %e, "Failed to acquire database connection");
                Err(e.into())
            }
        }
    }

    #[instrument(
        name = "service_get_product_by_id",
        skip(self),
        fields(product_id = %product_id)
    )]
    pub fn get_product_by_id(&self, product_id: Uuid) -> Result<Option<Product>> {
        info!(product_id = %product_id, "🔍 Fetching product by ID from database");
        
        let mut conn = self.get_connection()?;
        
        let result = products::table
            .filter(products::id.eq(product_id))
            .select(Product::as_select())
            .first(&mut conn)
            .optional();
            
        match result {
            Ok(Some(product)) => {
                info!(
                    product_id = %product_id,
                    product_name = %product.name,
                    "Product found in database"
                );
                Ok(Some(product))
            }
            Ok(None) => {
                info!(product_id = %product_id, "Product not found in database");
                Ok(None)
            }
            Err(e) => {
                error!(
                    product_id = %product_id,
                    error = %e,
                    "Database error while fetching product"
                );
                Err(e.into())
            }
        }
    }

    #[instrument(
        name = "service_get_products",
        skip(self, filters),
        fields(
            has_filters = filters.is_some(),
            filter_name = filters.as_ref().and_then(|f| f.name.as_deref()).unwrap_or("none"),
            filter_active = filters.as_ref().and_then(|f| f.is_active)
        )
    )]
    pub fn get_products(&self, filters: Option<ProductFilters>) -> Result<Vec<Product>> {
        let has_filters = filters.is_some();
        info!(
            has_filters = has_filters,
            filter_name = filters.as_ref().and_then(|f| f.name.as_deref()).unwrap_or("none"),
            filter_active = filters.as_ref().and_then(|f| f.is_active),
            "Fetching products from database with filters"
        );
        
        let mut conn = self.get_connection()?;
        let mut query = products::table.into_boxed();
        
        if let Some(filters) = filters {
            if let Some(product_name) = filters.name {
                debug!(filter_name = %product_name, "Applying name filter");
                query = query.filter(products::name.ilike(format!("%{}%", product_name)));
            }
            
            if let Some(min_cost) = filters.cost_ge {
                debug!(min_cost = min_cost, "Applying minimum cost filter");
                query = query.filter(products::cost.ge(min_cost));
            }
            
            if let Some(max_cost) = filters.cost_le {
                debug!(max_cost = max_cost, "Applying maximum cost filter");
                query = query.filter(products::cost.le(max_cost));
            }
            
            if let Some(is_active_filter) = filters.is_active {
                debug!(is_active = is_active_filter, "Applying active status filter");
                query = query.filter(products::active.eq(is_active_filter));
            }
        }
        
        let result = query
            .select(Product::as_select())
            .load(&mut conn);
            
        match result {
            Ok(products) => {
                info!(
                    product_count = products.len(),
                    "Products fetched successfully from database"
                );
                Ok(products)
            }
            Err(e) => {
                error!(
                    error = %e,
                    "Database error while fetching products"
                );
                Err(e.into())
            }
        }
    }

    #[instrument(
        name = "service_update_product",
        skip(self, updates),
        fields(
            product_id = %product_id,
            update_name = updates.name.is_some(),
            update_cost = updates.cost.is_some(),
            update_active = updates.active.is_some()
        )
    )]
    pub fn update_product(&self, product_id: Uuid, updates: ProductUpdates) -> Result<Option<Product>> {
        info!(
            product_id = %product_id,
            update_name = updates.name.is_some(),
            update_cost = updates.cost.is_some(),
            update_active = updates.active.is_some(),
            "Updating product in database"
        );
        
        let mut conn = self.get_connection()?;
        
        let result = diesel::update(products::table.filter(products::id.eq(product_id)))
                .set(&updates)
                .returning(Product::as_select())
                .get_result(&mut conn)
                .optional();
                
        match result {
            Ok(Some(product)) => {
                info!(
                    product_id = %product_id,
                    product_name = %product.name,
                    "Product updated successfully in database"
                );
                Ok(Some(product))
            }
            Ok(None) => {
                info!(product_id = %product_id, "Product not found for update");
                Ok(None)
            }
            Err(e) => {
                error!(
                    product_id = %product_id,
                    error = %e,
                    "Database error while updating product"
                );
                Err(e.into())
            }
        }
    }
   
    #[instrument(skip(self), fields(product_name = new_complete_product.product.name))]
    pub fn create_product(&self, new_complete_product: NewCompleteProduct) -> Result<Product> {
        info!("🆕 Creating new product with {} variants", new_complete_product.variants.len());
        
        let mut conn = self.get_connection()
            .map_err(|e| {
                warn!("Failed to get database connection for product creation: {}", e);
                e
            })?;

        let NewCompleteProduct { 
            product: new_product, 
            variants: new_variants
        } = new_complete_product;

        let result = conn.transaction(|conn| {
            info!("💾 Inserting product into database");
            let product = diesel::insert_into(products::table)
                .values(new_product)
                .returning(Product::as_select())
                .get_result(conn)
                .map_err(|e| {
                    warn!("Failed to insert product: {}", e);
                    e
                })?;

            info!("Product created successfully with ID: {}", product.id);

            for (index, variant_value) in new_variants.iter().enumerate() {
                info!("Processing variant {} of {}", index + 1, new_variants.len());
                
                let variant_id = diesel::insert_into(variants::table)
                    .values(&variant_value.variant)
                    .returning(variants::id)
                    .get_result::<Uuid>(conn)
                    .map_err(|e| {
                        warn!("Failed to insert variant {}: {}", index + 1, e);
                        e
                    })?;

                // Insert each value for this variant
                for (value_index, value) in variant_value.values.iter().enumerate() {
                    if let Some(val) = value {
                        let new_product_variant = NewProductVariant {
                            variant_id,
                            product_id: product.id,
                            value: val.clone(),
                        };
                        diesel::insert_into(product_variants::table)
                            .values(new_product_variant)
                            .execute(conn)
                            .map_err(|e| {
                                warn!("Failed to insert variant value {} for variant {}: {}", value_index + 1, index + 1, e);
                                e
                            })?;
                    }
                }
                info!("Variant {} processed successfully", index + 1);
            }

            info!("All variants processed successfully for product {}", product.id);
            Ok(product)
        });

        match &result {
            Ok(product) => {
                info!("Product creation completed successfully for ID: {}", product.id);
            }
            Err(e) => {
                warn!("Product creation failed: {}", e);
            }
        }

        result
    }

    #[instrument(skip(self), fields(product_id = %product_id))]
    pub fn delete_product(&self, product_id: Uuid) -> Result<bool> {
        info!("Attempting to delete product with ID: {}", product_id);
        
        let mut conn = self.get_connection()
            .map_err(|e| {
                warn!("Failed to get database connection for product deletion: {}", e);
                e
            })?;
        
        let result = diesel::delete(products::table.filter(products::id.eq(product_id)))
            .execute(&mut conn)
            .map_err(|e| {
                warn!("Failed to execute delete query for product {}: {}", product_id, e);
                e
            })?;
        
        let deleted = result > 0;
        
        if deleted {
            info!("Product {} deleted successfully (affected {} rows)", product_id, result);
        } else {
            warn!("Product {} not found or already deleted (affected {} rows)", product_id, result);
        }
        
        Ok(deleted)
    }
}