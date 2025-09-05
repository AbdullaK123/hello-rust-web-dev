use diesel::prelude::*;
use diesel::{ExpressionMethods, RunQueryDsl};
use crate::config::{DbConnection, DbPool};
use crate::models::{NewCompleteProduct, NewProductVariant, Product, ProductFilters, ProductUpdates};
use crate::schema::{product_variants, products, variants};
use anyhow::Result;

pub struct ProductService {
    pub pool: DbPool
}

impl ProductService {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    } 

    fn get_connection(&self) -> Result<DbConnection> {
        Ok(self.pool.get()?)
    }

    pub fn get_product_by_id(&self, product_id: i32) -> Result<Option<Product>> {
        let mut conn = self.get_connection()?;
        
        let product = products::table
            .filter(products::id.eq(product_id))
            .select(Product::as_select())
            .first(&mut conn)
            .optional()?;
        Ok(product)
    }

    pub fn get_products(&self, filters: Option<ProductFilters>) -> Result<Vec<Product>> {
        let mut conn = self.get_connection()?;
        let mut query = products::table.into_boxed();
        
        if let Some(filters) = filters {
            if let Some(product_name) = filters.name {
                query = query.filter(products::name.ilike(format!("%{}%", product_name)));
            }
            
            if let Some(min_cost) = filters.cost_ge {
                query = query.filter(products::cost.ge(min_cost));
            }
            
            if let Some(max_cost) = filters.cost_le {
                query = query.filter(products::cost.le(max_cost));
            }
            
            if let Some(is_active_filter) = filters.is_active {
                query = query.filter(products::active.eq(is_active_filter));
            }
        }
        
        let result = query
            .select(Product::as_select())
            .load(&mut conn)?;
        Ok(result)
    }

    pub fn update_product(&self, product_id: i32, updates: ProductUpdates) -> Result<Option<Product>> {
        let mut conn = self.get_connection()?;
        
        let product = diesel::update(products::table.filter(products::id.eq(product_id)))
            .set(&updates)
            .returning(Product::as_select())
            .get_result(&mut conn)
            .optional()?;
        Ok(product)
    }
   
    pub fn create_product(&self, new_complete_product: NewCompleteProduct) -> Result<Product> {
        let mut conn = self.get_connection()?;

        let NewCompleteProduct { 
            product: new_product, 
            variants: new_variants
        } = new_complete_product;

        conn.transaction(|conn| {
            let product = diesel::insert_into(products::table)
                .values(new_product)
                .returning(Product::as_select())
                .get_result(conn)?;

            for variant_value in new_variants {
                let variant_id = diesel::insert_into(variants::table)
                    .values(variant_value.variant)
                    .returning(variants::id)
                    .get_result::<i32>(conn)?;
                
                // Insert each value for this variant
                for value in variant_value.values {
                    if let Some(val) = value {
                        let new_product_variant = NewProductVariant {
                            variant_id,
                            product_id: product.id,
                            value: val,
                        };
                        
                        diesel::insert_into(product_variants::table)
                            .values(new_product_variant)
                            .execute(conn)?;
                    }
                }
            }

            Ok(product)
        })
    }

    pub fn delete_product(&self, product_id: i32) -> Result<bool> {
        let mut conn = self.get_connection()?;
        
        let result = diesel::delete(products::table.filter(products::id.eq(product_id)))
            .execute(&mut conn)?;
        Ok(result > 0)
    }
}