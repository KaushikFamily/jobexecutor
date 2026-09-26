#!/bin/bash

# 1. Ensure the shared network exists before compose tries to attach to it
docker network inspect home-network >/dev/null 2>&1 || docker network create home-network

# 2. Build the Docker image without cache using the new Dockerfile
docker build --no-cache -t job-executor:latest .

# 3. Restart the environment
docker compose down
docker compose up --build -d