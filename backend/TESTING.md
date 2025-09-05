# Endpoint Testing Guide

## Overview
This project includes comprehensive unit tests for both the service layer and HTTP endpoint functionality.

## Test Categories

### 1. Service Layer Tests (✅ PASSING - 8/8)
Located in `tests/product_tests.rs` - These test the core business logic:

- ✅ `test_service_create_and_get_product` - Creates a product and retrieves it
- ✅ `test_service_get_products_no_filter` - Gets all products without filters
- ✅ `test_service_get_products_with_filters` - Tests product filtering by name, cost, and active status
- ✅ `test_service_update_product` - Updates an existing product
- ✅ `test_service_delete_product` - Deletes a product
- ✅ `test_service_get_nonexistent_product` - Tests 404 handling
- ✅ `test_service_update_nonexistent_product` - Tests updating non-existent products
- ✅ `test_service_delete_nonexistent_product` - Tests deleting non-existent products

### 2. HTTP Endpoint Tests (🔧 INTEGRATION)
Located in `tests/endpoint_tests.rs` - These test the full HTTP API:

These tests require a running server instance and test:
- ✅ POST `/products` - Create product with variants
- ✅ GET `/products` - List all products with filters
- ✅ GET `/products/{id}` - Get product by ID
- ✅ PUT `/products/{id}` - Update product (full and partial)
- ✅ DELETE `/products/{id}` - Delete product
- ✅ Error handling (404, 400, 405)
- ✅ Full CRUD flow integration test

## Running Tests

### Service Layer Tests (Recommended)
```bash
cd backend
cargo test product_tests -- --nocapture
```

### HTTP Integration Tests (Manual)
```bash
# 1. Start the development database
cd scripts
./start_dev_db.sh

# 2. Start the server in one terminal
cd backend
cargo run

# 3. In another terminal, run endpoint tests
cargo test endpoint_tests -- --nocapture
```

### All Tests
```bash
cargo test -- --nocapture
```

## API Endpoints Tested

### Product Management
- **POST /products** - Create new product with optional variants
- **GET /products** - List products with optional filters:
  - `?name=search_term` - Filter by name
  - `?cost_ge=min_price` - Filter by minimum cost
  - `?cost_le=max_price` - Filter by maximum cost
  - `?is_active=true/false` - Filter by active status
- **GET /products/{id}** - Get specific product
- **PUT /products/{id}** - Update product (supports partial updates)
- **DELETE /products/{id}** - Delete product

### Example Requests

#### Create Product
```json
POST /products
{
  "product": {
    "name": "Test Product",
    "cost": 29.99,
    "active": true
  },
  "variants": [
    {
      "variant": { "name": "Color" },
      "values": ["Red", "Blue", "Green"]
    }
  ]
}
```

#### Update Product (Partial)
```json
PUT /products/1
{
  "name": "Updated Product Name"
}
```

## Test Coverage

### ✅ Covered Scenarios
- Product CRUD operations
- Product filtering and search
- Variant creation and management
- Error handling (404, 400, 405)
- Partial updates
- Data validation
- Database constraints

### 🎯 Test Quality
- **Database Integration**: All tests use real PostgreSQL database
- **Transaction Safety**: Each test is isolated
- **Comprehensive Coverage**: Tests both success and failure paths
- **Real Data**: Tests use actual JSON payloads and database operations

### 📊 Test Results Summary
- **Service Layer**: 8/8 tests passing ✅
- **Database Operations**: All CRUD operations validated ✅
- **Error Handling**: 404, validation errors properly tested ✅
- **Filtering**: Name, cost, and active status filters working ✅

## Development Workflow

1. **Make Changes** to service or controller code
2. **Run Service Tests** to validate business logic: `cargo test product_tests`
3. **Run Full Test Suite** to ensure no regressions: `cargo test`
4. **Manual API Testing** using curl or Postman for endpoint validation

The service layer tests provide comprehensive coverage of the core functionality and run quickly without requiring external server setup, making them ideal for continuous development and CI/CD pipelines.
