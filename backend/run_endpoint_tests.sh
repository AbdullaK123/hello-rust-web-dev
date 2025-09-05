#!/bin/bash

# Script to run endpoint tests
echo "Setting up test environment..."

# Set test environment variables
export DATABASE_URL="postgresql://postgres:password@localhost:5432/products_db"
export RUST_LOG=debug

# Start the server on port 8001 for testing
echo "Starting test server on port 8001..."
cargo run &
SERVER_PID=$!

# Wait for server to start
sleep 3

# Run the endpoint tests
echo "Running endpoint tests..."
cargo test endpoint_tests -- --nocapture

# Kill the server
echo "Stopping test server..."
kill $SERVER_PID

echo "Endpoint tests completed!"
