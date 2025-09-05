#!/bin/bash

CONTAINER_NAME="shoestore_db"

# Check if container exists and is running
if docker ps -q -f name=$CONTAINER_NAME | grep -q .; then
    echo "Container $CONTAINER_NAME is already running"
    exit 0
fi

# Check if container exists but is stopped
if docker ps -aq -f name=$CONTAINER_NAME | grep -q .; then
    echo "Container $CONTAINER_NAME exists but is stopped. Starting it..."
    docker start $CONTAINER_NAME
else
    echo "Container $CONTAINER_NAME doesn't exist. Creating it..."
    docker run --name $CONTAINER_NAME -e POSTGRES_PASSWORD=password -p 5432:5432 -d postgres
fi

echo "Container $CONTAINER_NAME is now running"