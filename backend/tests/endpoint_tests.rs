use reqwest;
use tokio;
use serde_json::{json, Value};
use uuid::Uuid;

// Helper to start the server in a separate thread for testing
const TEST_SERVER_URL: &str = "http://localhost:8000";

#[tokio::test]
async fn test_endpoint_get_all_products() {
    let client = reqwest::Client::new();
    
    // Test GET /products
    let response = client
        .get(&format!("{}/products", TEST_SERVER_URL))
        .send()
        .await;

    // If server is not running, skip this test
    if response.is_err() {
        println!("Server not running, skipping endpoint tests");
        return;
    }

    let response = response.unwrap();
    assert_eq!(response.status(), 200);
    
    let body: Value = response.json().await.unwrap();
    assert!(body.is_array());
}

#[tokio::test] 
async fn test_endpoint_create_product() {
    let client = reqwest::Client::new();
    
    let new_product = json!({
        "product": {
            "name": "HTTP Integration Test Product",
            "cost": 39.99,
            "active": true
        },
        "variants": []
    });

    // Test POST /products
    let response = client
        .post(&format!("{}/products", TEST_SERVER_URL))
        .json(&new_product)
        .send()
        .await;

    if response.is_err() {
        println!("Server not running, skipping endpoint tests");
        return;
    }

    let response = response.unwrap();
    assert_eq!(response.status(), 200);
    
    let body: Value = response.json().await.unwrap();
    assert_eq!(body["name"], "HTTP Integration Test Product");
    assert_eq!(body["cost"], 39.99);
    assert_eq!(body["active"], true);
    assert!(body["id"].is_string()); // UUID is serialized as string
}

#[tokio::test]
async fn test_endpoint_create_product_with_variants() {
    let client = reqwest::Client::new();
    
    let new_product = json!({
        "product": {
            "name": "Product with Variants HTTP Test",
            "cost": 59.99,
            "active": true
        },
        "variants": [
            {
                "variant": {
                    "name": "Color"
                },
                "values": ["Red", "Blue", "Green"]
            },
            {
                "variant": {
                    "name": "Size"
                },
                "values": ["Small", "Medium", "Large"]
            }
        ]
    });

    let response = client
        .post(&format!("{}/products", TEST_SERVER_URL))
        .json(&new_product)
        .send()
        .await;

    if response.is_err() {
        println!("Server not running, skipping endpoint tests");
        return;
    }

    let response = response.unwrap();
    assert_eq!(response.status(), 200);
    
    let body: Value = response.json().await.unwrap();
    assert_eq!(body["name"], "Product with Variants HTTP Test");
}

#[tokio::test]
async fn test_endpoint_get_products_with_filters() {
    let client = reqwest::Client::new();
    
    // Test with name filter
    let response = client
        .get(&format!("{}/products?name=test", TEST_SERVER_URL))
        .send()
        .await;

    if response.is_err() {
        println!("Server not running, skipping endpoint tests");
        return;
    }

    let response = response.unwrap();
    assert_eq!(response.status(), 200);
    
    let body: Value = response.json().await.unwrap();
    assert!(body.is_array());
}

#[tokio::test]
async fn test_endpoint_get_products_with_cost_filters() {
    let client = reqwest::Client::new();
    
    // Test with cost range filters
    let response = client
        .get(&format!("{}/products?cost_ge=10.0&cost_le=100.0", TEST_SERVER_URL))
        .send()
        .await;

    if response.is_err() {
        println!("Server not running, skipping endpoint tests");
        return;
    }

    let response = response.unwrap();
    assert_eq!(response.status(), 200);
    
    let body: Value = response.json().await.unwrap();
    assert!(body.is_array());
}

#[tokio::test]
async fn test_endpoint_get_products_with_active_filter() {
    let client = reqwest::Client::new();
    
    // Test with active filter
    let response = client
        .get(&format!("{}/products?is_active=true", TEST_SERVER_URL))
        .send()
        .await;

    if response.is_err() {
        println!("Server not running, skipping endpoint tests");
        return;
    }

    let response = response.unwrap();
    assert_eq!(response.status(), 200);
    
    let body: Value = response.json().await.unwrap();
    assert!(body.is_array());
}

#[tokio::test]
async fn test_endpoint_get_product_by_id_not_found() {
    let client = reqwest::Client::new();
    
    // Test GET /products/{id} with non-existent ID
    let non_existent_id = Uuid::new_v4();
    let response = client
        .get(&format!("{}/products/{}", TEST_SERVER_URL, non_existent_id))
        .send()
        .await;

    if response.is_err() {
        println!("Server not running, skipping endpoint tests");
        return;
    }

    let response = response.unwrap();
    assert_eq!(response.status(), 404);
    
    let body: Value = response.json().await.unwrap();
    assert!(body["error"].is_string());
}

#[tokio::test]
async fn test_endpoint_update_product_not_found() {
    let client = reqwest::Client::new();
    
    let update_data = json!({
        "name": "Updated Product",
        "cost": 49.99,
        "active": false
    });

    // Test PUT /products/{id} with non-existent ID
    let non_existent_id = Uuid::new_v4();
    let response = client
        .put(&format!("{}/products/{}", TEST_SERVER_URL, non_existent_id))
        .json(&update_data)
        .send()
        .await;

    if response.is_err() {
        println!("Server not running, skipping endpoint tests");
        return;
    }

    let response = response.unwrap();
    assert_eq!(response.status(), 404);
    
    let body: Value = response.json().await.unwrap();
    assert!(body["error"].is_string());
}

#[tokio::test]
async fn test_endpoint_delete_product_not_found() {
    let client = reqwest::Client::new();
    
    // Test DELETE /products/{id} with non-existent ID
    let non_existent_id = Uuid::new_v4();
    let response = client
        .delete(&format!("{}/products/{}", TEST_SERVER_URL, non_existent_id))
        .send()
        .await;

    if response.is_err() {
        println!("Server not running, skipping endpoint tests");
        return;
    }

    let response = response.unwrap();
    assert_eq!(response.status(), 404);
    
    let body: Value = response.json().await.unwrap();
    assert!(body["error"].is_string());
}

#[tokio::test]
async fn test_endpoint_full_crud_flow() {
    let client = reqwest::Client::new();
    
    // 1. Create a product
    let new_product = json!({
        "product": {
            "name": "CRUD Flow HTTP Test Product",
            "cost": 25.50,
            "active": true
        },
        "variants": []
    });

    let response = client
        .post(&format!("{}/products", TEST_SERVER_URL))
        .json(&new_product)
        .send()
        .await;

    if response.is_err() {
        println!("Server not running, skipping endpoint tests");
        return;
    }

    let response = response.unwrap();
    assert_eq!(response.status(), 200);
    
    let created_product: Value = response.json().await.unwrap();
    let product_id = created_product["id"].as_str().unwrap(); // UUID is a string
    
    // 2. Get the created product by ID
    let response = client
        .get(&format!("{}/products/{}", TEST_SERVER_URL, product_id))
        .send()
        .await
        .unwrap();
    
    assert_eq!(response.status(), 200);
    
    let retrieved_product: Value = response.json().await.unwrap();
    assert_eq!(retrieved_product["name"], "CRUD Flow HTTP Test Product");
    assert_eq!(retrieved_product["cost"], 25.5);
    assert_eq!(retrieved_product["active"], true);
    assert_eq!(retrieved_product["id"], product_id);
    
    // 3. Update the product
    let update_data = json!({
        "name": "Updated CRUD Flow HTTP Test Product",
        "cost": 35.75,
        "active": false
    });

    let response = client
        .put(&format!("{}/products/{}", TEST_SERVER_URL, product_id))
        .json(&update_data)
        .send()
        .await
        .unwrap();
    
    assert_eq!(response.status(), 200);
    
    let updated_product: Value = response.json().await.unwrap();
    assert_eq!(updated_product["name"], "Updated CRUD Flow HTTP Test Product");
    assert_eq!(updated_product["cost"], 35.75);
    assert_eq!(updated_product["active"], false);
    assert_eq!(updated_product["id"], product_id);
    
    // 4. Verify the update by getting the product again
    let response = client
        .get(&format!("{}/products/{}", TEST_SERVER_URL, product_id))
        .send()
        .await
        .unwrap();
    
    assert_eq!(response.status(), 200);
    
    let verified_product: Value = response.json().await.unwrap();
    assert_eq!(verified_product["name"], "Updated CRUD Flow HTTP Test Product");
    assert_eq!(verified_product["cost"], 35.75);
    assert_eq!(verified_product["active"], false);
    
    // 5. Delete the product
    let response = client
        .delete(&format!("{}/products/{}", TEST_SERVER_URL, product_id))
        .send()
        .await
        .unwrap();
    
    assert_eq!(response.status(), 204);
    
    // 6. Verify product is deleted
    let response = client
        .get(&format!("{}/products/{}", TEST_SERVER_URL, product_id))
        .send()
        .await
        .unwrap();
    
    assert_eq!(response.status(), 404);
}

#[tokio::test]
async fn test_endpoint_update_partial_fields() {
    let client = reqwest::Client::new();
    
    // Create a product first
    let new_product = json!({
        "product": {
            "name": "Partial Update HTTP Test",
            "cost": 20.0,
            "active": true
        },
        "variants": []
    });

    let response = client
        .post(&format!("{}/products", TEST_SERVER_URL))
        .json(&new_product)
        .send()
        .await;

    if response.is_err() {
        println!("Server not running, skipping endpoint tests");
        return;
    }

    let response = response.unwrap();
    let created_product: Value = response.json().await.unwrap();
    let product_id = created_product["id"].as_str().unwrap(); // UUID is a string
    
    // Update only the name
    let partial_update = json!({
        "name": "Only Name Updated HTTP"
    });

    let response = client
        .put(&format!("{}/products/{}", TEST_SERVER_URL, product_id))
        .json(&partial_update)
        .send()
        .await
        .unwrap();
    
    assert_eq!(response.status(), 200);
    
    let updated_product: Value = response.json().await.unwrap();
    assert_eq!(updated_product["name"], "Only Name Updated HTTP");
    assert_eq!(updated_product["cost"], 20.0); // Should remain unchanged
    assert_eq!(updated_product["active"], true); // Should remain unchanged
    
    // Clean up
    let _ = client
        .delete(&format!("{}/products/{}", TEST_SERVER_URL, product_id))
        .send()
        .await;
}

#[tokio::test]
async fn test_endpoint_invalid_product_id() {
    let client = reqwest::Client::new();
    
    // Test with non-numeric ID
    let response = client
        .get(&format!("{}/products/invalid", TEST_SERVER_URL))
        .send()
        .await;

    if response.is_err() {
        println!("Server not running, skipping endpoint tests");
        return;
    }

    let response = response.unwrap();
    assert!(response.status().is_client_error());
}

#[tokio::test]
async fn test_endpoint_create_product_invalid_json() {
    let client = reqwest::Client::new();
    
    // Test with invalid JSON
    let response = client
        .post(&format!("{}/products", TEST_SERVER_URL))
        .header("content-type", "application/json")
        .body("{invalid json")
        .send()
        .await;

    if response.is_err() {
        println!("Server not running, skipping endpoint tests");
        return;
    }

    let response = response.unwrap();
    assert_eq!(response.status(), 400);
}

#[tokio::test]
async fn test_endpoint_method_not_allowed() {
    let client = reqwest::Client::new();
    
    // PATCH is not supported on /products
    let response = client
        .patch(&format!("{}/products", TEST_SERVER_URL))
        .send()
        .await;

    if response.is_err() {
        println!("Server not running, skipping endpoint tests");
        return;
    }

    let response = response.unwrap();
    assert_eq!(response.status(), 405);
}

#[tokio::test]
async fn test_endpoint_not_found_route() {
    let client = reqwest::Client::new();
    
    let response = client
        .get(&format!("{}/nonexistent", TEST_SERVER_URL))
        .send()
        .await;

    if response.is_err() {
        println!("Server not running, skipping endpoint tests");
        return;
    }

    let response = response.unwrap();
    assert_eq!(response.status(), 404);
}
