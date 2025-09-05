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
async fn test_service_create_product() {
    let service = create_test_service();
    
    let new_product = NewCompleteProduct {
        product: NewProduct {
            id: None,
            name: "Service Test Product".to_string(),
            cost: 15.99,
            active: true,
        },
        variants: vec![],
    };
    
    let result = service.create_product(new_product);
    assert!(result.is_ok());
    
    let created_product = result.unwrap();
    assert_eq!(created_product.name, "Service Test Product");
    assert_eq!(created_product.cost, 15.99);
    assert!(created_product.active);
}

#[tokio::test]
async fn test_service_get_products() {
    let service = create_test_service();
    
    // Test without filters
    let result = service.get_products(None);
    assert!(result.is_ok());
    
    // Test with filters
    let filters = ProductFilters {
        name: Some("test".to_string()),
        cost_ge: Some(10.0),
        cost_le: Some(50.0),
        is_active: Some(true),
    };
    
    let result = service.get_products(Some(filters));
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_service_get_product_by_id_not_found() {
    let service = create_test_service();
    
    let result = service.get_product_by_id(Uuid::new_v4());
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}

#[tokio::test]
async fn test_service_update_product_not_found() {
    let service = create_test_service();
    
    let updates = ProductUpdates {
        name: Some("Updated Name".to_string()),
        cost: Some(99.99),
        active: Some(false),
    };
    
    let result = service.update_product(Uuid::new_v4(), updates);
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}

#[tokio::test]
async fn test_service_delete_product_not_found() {
    let service = create_test_service();
    
    let result = service.delete_product(Uuid::new_v4());
    assert!(result.is_ok());
    assert!(!result.unwrap()); // Should return false for non-existent product
}

#[tokio::test]
async fn test_service_full_flow() {
    let service = create_test_service();
    
    // 1. Create a product
    let new_product = NewCompleteProduct {
        product: NewProduct {
            id: None,
            name: "Service Flow Test".to_string(),
            cost: 45.00,
            active: true,
        },
        variants: vec![],
    };
    
    let created = service.create_product(new_product).unwrap();
    let product_id = created.id;
    
    // 2. Get the product by ID
    let retrieved = service.get_product_by_id(product_id).unwrap();
    assert!(retrieved.is_some());
    let retrieved = retrieved.unwrap();
    assert_eq!(retrieved.name, "Service Flow Test");
    
    // 3. Update the product
    let updates = ProductUpdates {
        name: Some("Updated Service Flow Test".to_string()),
        cost: Some(55.00),
        active: Some(false),
    };
    
    let updated = service.update_product(product_id, updates).unwrap();
    assert!(updated.is_some());
    let updated = updated.unwrap();
    assert_eq!(updated.name, "Updated Service Flow Test");
    assert_eq!(updated.cost, 55.0);
    assert!(!updated.active);
    
    // 4. Delete the product
    let deleted = service.delete_product(product_id).unwrap();
    assert!(deleted);
    
    // 5. Verify deletion
    let not_found = service.get_product_by_id(product_id).unwrap();
    assert!(not_found.is_none());
}
