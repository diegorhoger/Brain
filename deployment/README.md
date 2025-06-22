# Deployment Directory

This directory contains all files related to deploying and containerizing the Brain AI system:

## Container Configuration
- `Dockerfile` - Multi-stage Docker build configuration for Brain AI
- `docker-compose.yml` - Complete service orchestration with optional components

## Features
- **Multi-stage builds** for optimized production images
- **Service orchestration** with optional Neo4j, Redis, Prometheus, and Grafana
- **Security hardening** with non-root containers
- **Health checks** and monitoring integration
- **Environment configuration** support

## Quick Start
```bash
# Build and run with Docker Compose
docker-compose up -d

# Or build Docker image manually
docker build -t brain-ai .
```

## Services Included
- **Brain AI Core** - Main application service
- **Neo4j** (optional) - Graph database for concept relationships
- **Redis** (optional) - Caching and session storage
- **Prometheus** (optional) - Metrics collection
- **Grafana** (optional) - Monitoring dashboards

## Configuration
Copy `env.example` from the root directory and customize for your deployment environment.

For detailed deployment instructions, see `../documentation/DEPLOYMENT.md`. 