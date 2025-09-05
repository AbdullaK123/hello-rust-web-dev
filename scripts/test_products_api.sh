#!/bin/bash

# Product API Test Script
# Tests all CRUD operations for the products endpoint

BASE_URL="http://localhost:8000"
PRODUCTS_URL="$BASE_URL/products"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Test counter
TESTS_RUN=0
TESTS_PASSED=0

# Function to print test results
print_test_result() {
    local test_name="$1"
    local status="$2"
    local response="$3"
    
    TESTS_RUN=$((TESTS_RUN + 1))
    
    if [ "$status" = "PASS" ]; then
        echo -e "${GREEN}✓ PASS${NC} - $test_name"
        TESTS_PASSED=$((TESTS_PASSED + 1))
    else
        echo -e "${RED}✗ FAIL${NC} - $test_name"
        echo -e "${YELLOW}Response:${NC} $response"
    fi
    echo
}

# Function to check if server is running
check_server() {
    echo -e "${BLUE}Checking if server is running...${NC}"
    if curl -s "$BASE_URL" > /dev/null 2>&1; then
        echo -e "${GREEN}Server is running at $BASE_URL${NC}"
    else
        echo -e "${RED}Server is not running at $BASE_URL${NC}"
        echo "Please start the server first with: cargo run"
        exit 1
    fi
    echo
}

# Test 1: Create a simple product
test_create_simple_product() {
    echo -e "${BLUE}Test 1: Create Simple Product${NC}"
    
    local response=$(curl -s -X POST "$PRODUCTS_URL" \
        -H "Content-Type: application/json" \
        -d '{
            "product": {
                "name": "Test Shoe",
                "cost": 99.99,
                "active": true
            },
            "variants": []
        }')
    
    if echo "$response" | grep -q '"name":"Test Shoe"' && echo "$response" | grep -q '"id"'; then
        # Extract product ID (UUID) for later tests
        PRODUCT_ID=$(echo "$response" | grep -o '"id":"[a-f0-9-]*"' | grep -o '[a-f0-9-]\{36\}')
        print_test_result "Create Simple Product" "PASS" "$response"
    else
        print_test_result "Create Simple Product" "FAIL" "$response"
    fi
}

# Test 2: Create product with variants
test_create_product_with_variants() {
    echo -e "${BLUE}Test 2: Create Product with Variants${NC}"
    
    local response=$(curl -s -X POST "$PRODUCTS_URL" \
        -H "Content-Type: application/json" \
        -d '{
            "product": {
                "name": "Running Shoe",
                "cost": 129.99,
                "active": true
            },
            "variants": [
                {
                    "variant": {
                        "name": "Size"
                    },
                    "values": ["8", "9", "10"]
                },
                {
                    "variant": {
                        "name": "Color"
                    },
                    "values": ["Red", "Blue"]
                }
            ]
        }')
    
    if echo "$response" | grep -q '"name":"Running Shoe"' && echo "$response" | grep -q '"id"'; then
        # Extract product ID (UUID) for later tests
        VARIANT_PRODUCT_ID=$(echo "$response" | grep -o '"id":"[a-f0-9-]*"' | grep -o '[a-f0-9-]\{36\}')
        print_test_result "Create Product with Variants" "PASS" "$response"
    else
        print_test_result "Create Product with Variants" "FAIL" "$response"
    fi
}

# Test 3: Get all products
test_get_all_products() {
    echo -e "${BLUE}Test 3: Get All Products${NC}"
    
    local response=$(curl -s -X GET "$PRODUCTS_URL")
    
    if echo "$response" | grep -q '\[' && echo "$response" | grep -q '"Test Shoe"'; then
        print_test_result "Get All Products" "PASS" "$response"
    else
        print_test_result "Get All Products" "FAIL" "$response"
    fi
}

# Test 4: Get product by ID
test_get_product_by_id() {
    echo -e "${BLUE}Test 4: Get Product by ID${NC}"
    
    if [ -z "$PRODUCT_ID" ]; then
        print_test_result "Get Product by ID" "FAIL" "No product ID available from previous test"
        return
    fi
    
    local response=$(curl -s -X GET "$PRODUCTS_URL/$PRODUCT_ID")
    
    if echo "$response" | grep -q '"name":"Test Shoe"' && echo "$response" | grep -q "\"id\":\"$PRODUCT_ID\""; then
        print_test_result "Get Product by ID" "PASS" "$response"
    else
        print_test_result "Get Product by ID" "FAIL" "$response"
    fi
}

# Test 5: Get non-existent product
test_get_nonexistent_product() {
    echo -e "${BLUE}Test 5: Get Non-existent Product${NC}"
    
    # Generate a random UUID for testing non-existent product
    local non_existent_uuid="00000000-0000-0000-0000-000000000000"
    local response=$(curl -s -w "%{http_code}" -X GET "$PRODUCTS_URL/$non_existent_uuid")
    
    if echo "$response" | grep -q "404"; then
        print_test_result "Get Non-existent Product (404)" "PASS" "$response"
    else
        print_test_result "Get Non-existent Product (404)" "FAIL" "$response"
    fi
}

# Test 6: Filter products by name
test_filter_products_by_name() {
    echo -e "${BLUE}Test 6: Filter Products by Name${NC}"
    
    local response=$(curl -s -X GET "$PRODUCTS_URL?name=Test")
    
    if echo "$response" | grep -q '"Test Shoe"' && echo "$response" | grep -q '\['; then
        print_test_result "Filter Products by Name" "PASS" "$response"
    else
        print_test_result "Filter Products by Name" "FAIL" "$response"
    fi
}

# Test 7: Filter products by cost
test_filter_products_by_cost() {
    echo -e "${BLUE}Test 7: Filter Products by Cost${NC}"
    
    local response=$(curl -s -X GET "$PRODUCTS_URL?cost_ge=100&cost_le=150")
    
    if echo "$response" | grep -q '"Running Shoe"' && echo "$response" | grep -q '\['; then
        print_test_result "Filter Products by Cost Range" "PASS" "$response"
    else
        print_test_result "Filter Products by Cost Range" "FAIL" "$response"
    fi
}

# Test 8: Filter products by active status
test_filter_products_by_active() {
    echo -e "${BLUE}Test 8: Filter Products by Active Status${NC}"
    
    local response=$(curl -s -X GET "$PRODUCTS_URL?is_active=true")
    
    if echo "$response" | grep -q '"active":true' && echo "$response" | grep -q '\['; then
        print_test_result "Filter Products by Active Status" "PASS" "$response"
    else
        print_test_result "Filter Products by Active Status" "FAIL" "$response"
    fi
}

# Test 9: Update product
test_update_product() {
    echo -e "${BLUE}Test 9: Update Product${NC}"
    
    if [ -z "$PRODUCT_ID" ]; then
        print_test_result "Update Product" "FAIL" "No product ID available from previous test"
        return
    fi
    
    local response=$(curl -s -X PUT "$PRODUCTS_URL/$PRODUCT_ID" \
        -H "Content-Type: application/json" \
        -d '{
            "name": "Updated Test Shoe",
            "cost": 149.99,
            "active": false
        }')
    
    if echo "$response" | grep -q '"name":"Updated Test Shoe"' && echo "$response" | grep -q '"cost":149.99'; then
        print_test_result "Update Product" "PASS" "$response"
    else
        print_test_result "Update Product" "FAIL" "$response"
    fi
}

# Test 10: Partial update product
test_partial_update_product() {
    echo -e "${BLUE}Test 10: Partial Update Product${NC}"
    
    if [ -z "$PRODUCT_ID" ]; then
        print_test_result "Partial Update Product" "FAIL" "No product ID available from previous test"
        return
    fi
    
    local response=$(curl -s -X PUT "$PRODUCTS_URL/$PRODUCT_ID" \
        -H "Content-Type: application/json" \
        -d '{
            "name": "Partially Updated Shoe"
        }')
    
    if echo "$response" | grep -q '"name":"Partially Updated Shoe"'; then
        print_test_result "Partial Update Product" "PASS" "$response"
    else
        print_test_result "Partial Update Product" "FAIL" "$response"
    fi
}

# Test 11: Update non-existent product
test_update_nonexistent_product() {
    echo -e "${BLUE}Test 11: Update Non-existent Product${NC}"
    
    # Generate a random UUID for testing non-existent product
    local non_existent_uuid="00000000-0000-0000-0000-000000000000"
    local response=$(curl -s -w "%{http_code}" -X PUT "$PRODUCTS_URL/$non_existent_uuid" \
        -H "Content-Type: application/json" \
        -d '{
            "name": "Should Not Work"
        }')
    
    if echo "$response" | grep -q "404"; then
        print_test_result "Update Non-existent Product (404)" "PASS" "$response"
    else
        print_test_result "Update Non-existent Product (404)" "FAIL" "$response"
    fi
}

# Test 12: Delete product
test_delete_product() {
    echo -e "${BLUE}Test 12: Delete Product${NC}"
    
    if [ -z "$VARIANT_PRODUCT_ID" ]; then
        print_test_result "Delete Product" "FAIL" "No variant product ID available from previous test"
        return
    fi
    
    local response=$(curl -s -w "%{http_code}" -X DELETE "$PRODUCTS_URL/$VARIANT_PRODUCT_ID")
    
    if echo "$response" | grep -q "204"; then
        print_test_result "Delete Product" "PASS" "$response"
    else
        print_test_result "Delete Product" "FAIL" "$response"
    fi
}

# Test 13: Delete non-existent product
test_delete_nonexistent_product() {
    echo -e "${BLUE}Test 13: Delete Non-existent Product${NC}"
    
    # Generate a random UUID for testing non-existent product
    local non_existent_uuid="00000000-0000-0000-0000-000000000000"
    local response=$(curl -s -w "%{http_code}" -X DELETE "$PRODUCTS_URL/$non_existent_uuid")
    
    if echo "$response" | grep -q "404"; then
        print_test_result "Delete Non-existent Product (404)" "PASS" "$response"
    else
        print_test_result "Delete Non-existent Product (404)" "FAIL" "$response"
    fi
}

# Test 14: Invalid JSON
test_invalid_json() {
    echo -e "${BLUE}Test 14: Invalid JSON${NC}"
    
    local response=$(curl -s -w "%{http_code}" -X POST "$PRODUCTS_URL" \
        -H "Content-Type: application/json" \
        -d '{invalid json}')
    
    if echo "$response" | grep -q "400"; then
        print_test_result "Invalid JSON (400)" "PASS" "$response"
    else
        print_test_result "Invalid JSON (400)" "FAIL" "$response"
    fi
}

# Clean up function
cleanup() {
    echo -e "${BLUE}Cleaning up test data...${NC}"
    if [ -n "$PRODUCT_ID" ]; then
        curl -s -X DELETE "$PRODUCTS_URL/$PRODUCT_ID" > /dev/null
        echo "Deleted test product with ID: $PRODUCT_ID"
    fi
}

# Main test execution
main() {
    echo -e "${BLUE}Starting Product API Tests${NC}"
    echo "=================================="
    echo
    
    check_server
    
    test_create_simple_product
    test_create_product_with_variants
    test_get_all_products
    test_get_product_by_id
    test_get_nonexistent_product
    test_filter_products_by_name
    test_filter_products_by_cost
    test_filter_products_by_active
    test_update_product
    test_partial_update_product
    test_update_nonexistent_product
    test_delete_product
    test_delete_nonexistent_product
    test_invalid_json
    
    # Print summary
    echo "=================================="
    echo -e "${BLUE}Test Summary${NC}"
    echo "Tests run: $TESTS_RUN"
    echo -e "Tests passed: ${GREEN}$TESTS_PASSED${NC}"
    echo -e "Tests failed: ${RED}$((TESTS_RUN - TESTS_PASSED))${NC}"
    
    if [ $TESTS_PASSED -eq $TESTS_RUN ]; then
        echo -e "${GREEN}All tests passed! 🎉${NC}"
        cleanup
        exit 0
    else
        echo -e "${RED}Some tests failed. Check the output above.${NC}"
        cleanup
        exit 1
    fi
}

# Run the tests
main