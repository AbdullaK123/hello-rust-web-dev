// Integration tests for the ProductService
// These tests require a running PostgreSQL database with the schema set up

use backend::services::ProductService;
use backend::models::{NewCompleteProduct, NewProduct, ProductFilters, ProductUpdates};
use backend::config::{create_pool, get_settings};
use uuid::Uuid;

fn create_test_service() -> ProductService {
    let settings = get_settings().unwrap();
    let pool = create_pool(&settings);
    ProductService::new(pool)
}

#[tokio::test]
async fn test_service_create_and_get_product() {
    let service = create_test_service();
    
    let new_product = NewCompleteProduct {
        product: NewProduct {
            id: None,
            name: "Test Product".to_string(),
            cost: 29.99,
            active: true,
        },
        variants: vec![],
    };
    
    // Create a product
    let result = service.create_product(new_product);
    assert!(result.is_ok(), "Failed to create product: {:?}", result.err());
    
    let created_product = result.unwrap();
    assert_eq!(created_product.name, "Test Product");
    assert_eq!(created_product.cost, 29.99);
    assert!(created_product.active);
    
    // Get the product by ID
    let get_result = service.get_product_by_id(created_product.id);
    assert!(get_result.is_ok(), "Failed to get product: {:?}", get_result.err());
    
    let retrieved = get_result.unwrap();
    assert!(retrieved.is_some());
    let retrieved = retrieved.unwrap();
    assert_eq!(retrieved.id, created_product.id);
    assert_eq!(retrieved.name, "Test Product");
    
    // Clean up: delete the product
    let _ = service.delete_product(created_product.id);
}

#[tokio::test]
async fn test_service_update_product() {
    let service = create_test_service();
    
    // Create a product first
    let new_product = NewCompleteProduct {
        product: NewProduct {
            id: None,
            name: "Update Test Product".to_string(),
            cost: 45.00,
            active: true,
        },
        variants: vec![],
    };
    
    let created = service.create_product(new_product).unwrap();
    
    // Update the product
    let updates = ProductUpdates {
        name: Some("Updated Product Name".to_string()),
        cost: Some(55.00),
        active: Some(false),
    };
    
    let update_result = service.update_product(created.id, updates);
    assert!(update_result.is_ok(), "Failed to update product: {:?}", update_result.err());
    
    let updated = update_result.unwrap();
    assert!(updated.is_some());
    let updated = updated.unwrap();
    assert_eq!(updated.name, "Updated Product Name");
    assert_eq!(updated.cost, 55.0);
    assert!(!updated.active);
    
    // Clean up: delete the product
    let _ = service.delete_product(created.id);
}

#[tokio::test]
async fn test_service_delete_product() {
    let service = create_test_service();
    
    // Create a product first
    let new_product = NewCompleteProduct {
        product: NewProduct {
            id: None,
            name: "Delete Test Product".to_string(),
            cost: 15.99,
            active: true,
        },
        variants: vec![],
    };
    
    let created = service.create_product(new_product).unwrap();
    let product_id = created.id;
    
    // Delete the product
    let delete_result = service.delete_product(product_id);
    assert!(delete_result.is_ok(), "Failed to delete product: {:?}", delete_result.err());
    assert!(delete_result.unwrap(), "Product should have been deleted");
    
    // Verify it's deleted
    let get_result = service.get_product_by_id(product_id);
    assert!(get_result.is_ok());
    assert!(get_result.unwrap().is_none(), "Product should not exist after deletion");
}

#[tokio::test]
async fn test_service_get_products_with_filters() {
    let service = create_test_service();
    
    // Create test products
    let products = vec![
        NewCompleteProduct {
            product: NewProduct {
                id: None,
                name: "Filter Test Product 1".to_string(),
                cost: 10.0,
                active: true,
            },
            variants: vec![],
        },
        NewCompleteProduct {
            product: NewProduct {
                id: None,
                name: "Filter Test Product 2".to_string(),
                cost: 50.0,
                active: false,
            },
            variants: vec![],
        },
    ];
    
    let mut created_ids = Vec::new();
    for product in products {
        let created = service.create_product(product).unwrap();
        created_ids.push(created.id);
    }
    
    // Test filtering by name
    let name_filter = ProductFilters {
        name: Some("Filter Test".to_string()),
        cost_ge: None,
        cost_le: None,
        is_active: None,
    };
    
    let filtered_result = service.get_products(Some(name_filter));
    assert!(filtered_result.is_ok(), "Failed to get filtered products: {:?}", filtered_result.err());
    let filtered_products = filtered_result.unwrap();
    assert!(!filtered_products.is_empty(), "Should find products with name filter");
    
    // Test filtering by cost range
    let cost_filter = ProductFilters {
        name: None,
        cost_ge: Some(40.0),
        cost_le: Some(60.0),
        is_active: None,
    };
    
    let cost_filtered_result = service.get_products(Some(cost_filter));
    assert!(cost_filtered_result.is_ok());
    
    // Test filtering by active status
    let active_filter = ProductFilters {
        name: None,
        cost_ge: None,
        cost_le: None,
        is_active: Some(true),
    };
    
    let active_filtered_result = service.get_products(Some(active_filter));
    assert!(active_filtered_result.is_ok());
    
    // Clean up: delete the test products
    for id in created_ids {
        let _ = service.delete_product(id);
    }
}

#[tokio::test]
async fn test_service_get_products_no_filter() {
    let service = create_test_service();
    
    let result = service.get_products(None);
    assert!(result.is_ok(), "Failed to get all products: {:?}", result.err());
    
    let products = result.unwrap();
    // We don't assert on the count since other tests might have created products
    // Just verify the call succeeds and returns a vector
    println!("Found {} products", products.len());
}

#[tokio::test]
async fn test_service_get_nonexistent_product() {
    let service = create_test_service();
    
    // Use a random UUID that doesn't exist
    let non_existent_id = Uuid::new_v4();
    let result = service.get_product_by_id(non_existent_id);
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}

#[tokio::test]
async fn test_service_update_nonexistent_product() {
    let service = create_test_service();
    
    let updates = ProductUpdates {
        name: Some("This won't work".to_string()),
        cost: Some(100.0),
        active: Some(true),
    };
    
    // Use a random UUID that doesn't exist
    let non_existent_id = Uuid::new_v4();
    let result = service.update_product(non_existent_id, updates);
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}

#[tokio::test]
async fn test_service_delete_nonexistent_product() {
    let service = create_test_service();
    
    // Use a random UUID that doesn't exist
    let non_existent_id = Uuid::new_v4();
    let result = service.delete_product(non_existent_id);
    assert!(result.is_ok());
    assert!(!result.unwrap()); // Should return false for non-existent product
}
