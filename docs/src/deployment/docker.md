# Docker Deployment

This guide covers deploying Brain AI using Docker containers, including single-node deployments, multi-container setups, and production configurations.

## Prerequisites

- Docker Engine 20.10 or later
- Docker Compose v2.0 or later
- At least 4GB RAM available for containers
- 10GB disk space for images and data

## Quick Start

### Single Container Deployment

```bash
# Pull the latest Brain AI image
docker pull brain-ai:latest

# Run with default configuration
docker run -d \
  --name brain-ai \
  -p 8080:8080 \
  -e ANTHROPIC_API_KEY=your_api_key_here \
  brain-ai:latest

# Check if it's running
docker ps
curl http://localhost:8080/api/v1/health
```

### With Environment File

Create a `.env` file for configuration:

```bash
# Create environment file
cat > .env << EOF
ANTHROPIC_API_KEY=your_api_key_here
MODEL=claude-3-opus-20240229
LOG_LEVEL=info
MEMORY_CAPACITY=100000
ENABLE_PERFORMANCE_MONITORING=true
JWT_SECRET=your-secret-key-here
EOF

# Run with environment file
docker run -d \
  --name brain-ai \
  -p 8080:8080 \
  --env-file .env \
  -v $(pwd)/data:/app/data \
  brain-ai:latest
```

## Docker Compose Deployment

### Basic Compose Setup

Create a `docker-compose.yml` file:

```yaml
version: '3.8'

services:
  brain-ai:
    image: brain-ai:latest
    container_name: brain-ai
    restart: unless-stopped
    ports:
      - "8080:8080"
    environment:
      - ANTHROPIC_API_KEY=${ANTHROPIC_API_KEY}
      - MODEL=${MODEL:-claude-3-opus-20240229}
      - LOG_LEVEL=${LOG_LEVEL:-info}
      - MEMORY_CAPACITY=${MEMORY_CAPACITY:-100000}
      - JWT_SECRET=${JWT_SECRET}
    volumes:
      - brain_data:/app/data
      - brain_logs:/app/logs
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8080/api/v1/health"]
      interval: 30s
      timeout: 10s
      retries: 3

volumes:
  brain_data:
  brain_logs:
```

Deploy with Docker Compose:

```bash
# Start the services
docker-compose up -d

# Check status
docker-compose ps

# View logs
docker-compose logs -f brain-ai

# Stop services
docker-compose down
```

## Configuration Management

### Environment Variables

Key environment variables for Docker deployment:

```bash
# Core Configuration
ANTHROPIC_API_KEY=your_api_key_here
MODEL=claude-3-opus-20240229
LOG_LEVEL=info
MEMORY_CAPACITY=1000000

# Network Configuration
HOST=0.0.0.0
PORT=8080

# Security Configuration
JWT_SECRET=your-secret-key-here
```

## Volume Management

### Data Persistence

Important directories to persist:

```yaml
volumes:
  # Application data (memories, concepts, etc.)
  - brain_data:/app/data
  
  # Application logs
  - brain_logs:/app/logs
  
  # Configuration files
  - ./config:/app/config:ro
```

## Monitoring and Security

### Health Checks

```bash
# Check container health
docker inspect --format='{{.State.Health.Status}}' brain-ai

# View logs
docker-compose logs -f brain-ai
```

### Basic Security

```dockerfile
# Use non-root user
USER brain

# Read-only root filesystem
docker run --read-only --tmpfs /tmp brain-ai:latest
```

This Docker deployment guide provides the essentials for containerized Brain AI deployment.
