# Brain AI System Deployment Guide

This guide provides comprehensive instructions for deploying the Brain AI system across different environments and platforms.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Quick Start](#quick-start)
- [Deployment Methods](#deployment-methods)
- [Environment Configuration](#environment-configuration)
- [Docker Deployment](#docker-deployment)
- [Kubernetes Deployment](#kubernetes-deployment)
- [Local Development](#local-development)
- [Production Deployment](#production-deployment)
- [Monitoring & Health Checks](#monitoring--health-checks)
- [Backup & Recovery](#backup--recovery)
- [Troubleshooting](#troubleshooting)
- [Security Considerations](#security-considerations)

## Prerequisites

### System Requirements

- **CPU**: 4+ cores recommended (2 cores minimum)
- **Memory**: 8GB+ RAM recommended (4GB minimum)
- **Storage**: 50GB+ available disk space
- **Network**: Stable internet connection for external dependencies

### Software Dependencies

#### Core Requirements
- **Rust**: 1.70+ (latest stable recommended)
- **Docker**: 20.10+ (for containerized deployment)
- **Docker Compose**: 2.0+ (for multi-service deployment)

#### Optional Components
- **Kubernetes**: 1.25+ (for cluster deployment)
- **Helm**: 3.8+ (for Kubernetes package management)
- **Neo4j**: 5.0+ (for graph database features)
- **Redis**: 6.0+ (for caching and session storage)

### Environment Setup

1. **Install Rust**:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   ```

2. **Install Docker** (Ubuntu/Debian):
   ```bash
   curl -fsSL https://get.docker.com -o get-docker.sh
   sudo sh get-docker.sh
   sudo usermod -aG docker $USER
   ```

3. **Install Docker Compose**:
   ```bash
   sudo curl -L "https://github.com/docker/compose/releases/latest/download/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
   sudo chmod +x /usr/local/bin/docker-compose
   ```

## Quick Start

### 1. Clone and Setup
```bash
git clone <repository-url> brain-ai
cd brain-ai
cp .env.example .env
```

### 2. Configure Environment
Edit `.env` file with your settings:
```bash
# Required: Anthropic API key for AI functionality
ANTHROPIC_API_KEY=your_anthropic_api_key_here

# Optional: Perplexity API key for research features
PERPLEXITY_API_KEY=your_perplexity_api_key_here

# Environment configuration
BRAIN_AI_ENVIRONMENT=production
BRAIN_AI_LOG_LEVEL=info
```

### 3. Deploy with Docker Compose
```bash
# Quick deployment
docker-compose up -d

# Or use the deployment script
./scripts/deploy.sh docker production --build --health-check
```

### 4. Verify Deployment
```bash
# Run health checks
./scripts/health_check.sh

# Check service status
docker-compose ps

# View logs
docker-compose logs -f
```

## Deployment Methods

### 1. Docker Compose (Recommended for Single-Node)
- **Best for**: Development, staging, small production deployments
- **Pros**: Simple setup, easy management, integrated services
- **Cons**: Single point of failure, limited scalability

### 2. Kubernetes (Recommended for Production)
- **Best for**: Production environments, high availability, scaling
- **Pros**: High availability, auto-scaling, rolling updates
- **Cons**: Complex setup, requires Kubernetes knowledge

### 3. Local Development
- **Best for**: Development, testing, debugging
- **Pros**: Fast iteration, full control, easy debugging
- **Cons**: Manual setup, no service orchestration

## Environment Configuration

### Configuration Files

1. **`.env`** - Environment variables
2. **`config.toml`** - Application configuration
3. **`docker-compose.yml`** - Service orchestration
4. **`Dockerfile`** - Container build instructions

### Environment Variables

#### Required Variables
```bash
# AI Service Configuration
ANTHROPIC_API_KEY=your_anthropic_api_key
MODEL=claude-3-opus-20240229
MAX_TOKENS=8192
TEMPERATURE=0.7

# System Configuration
BRAIN_AI_ENVIRONMENT=production
BRAIN_AI_LOG_LEVEL=info
RUST_LOG=info
```

#### Optional Variables
```bash
# Performance Monitoring
PERPLEXITY_API_KEY=your_perplexity_key
ENABLE_PERFORMANCE_MONITORING=true
ENABLE_HEALTH_CHECKS=true

# Database Configuration
NEO4J_URI=bolt://localhost:7687
NEO4J_USERNAME=neo4j
NEO4J_PASSWORD=your_neo4j_password

# API Configuration
API_HOST=0.0.0.0
API_PORT=8080
ENABLE_CORS=true

# Security
ENABLE_AUTHENTICATION=false
API_KEY_REQUIRED=false
```

### Configuration Profiles

#### Development
```toml
[system]
environment = "development"

[logging]
level = "debug"
enable_comprehensive = true

[performance]
enable_monitoring = false
enable_health_checks = false
```

#### Production
```toml
[system]
environment = "production"

[logging]
level = "info"
enable_file_logging = true

[performance]
enable_monitoring = true
enable_health_checks = true
max_concurrent_operations = 100
```

## Docker Deployment

### Basic Docker Compose Deployment

1. **Start Services**:
   ```bash
   docker-compose up -d
   ```

2. **View Status**:
   ```bash
   docker-compose ps
   ```

3. **View Logs**:
   ```bash
   docker-compose logs -f brain-ai
   ```

4. **Stop Services**:
   ```bash
   docker-compose down
   ```

### Advanced Docker Deployment

#### With External Services
```bash
# Start with Neo4j and Redis
docker-compose -f docker-compose.yml -f docker-compose.prod.yml up -d
```

#### Custom Build
```bash
# Build custom image
docker build -t brain-ai:custom .

# Deploy with custom image
BRAIN_AI_VERSION=custom docker-compose up -d
```

#### Resource Limits
```yaml
services:
  brain-ai:
    deploy:
      resources:
        limits:
          cpus: '2.0'
          memory: 4G
        reservations:
          cpus: '1.0'
          memory: 2G
```

### Using Deployment Script

```bash
# Basic deployment
./scripts/deploy.sh docker production

# Build and deploy
./scripts/deploy.sh docker production --build --push

# Deploy with health checks
./scripts/deploy.sh docker production --health-check

# Dry run (preview)
./scripts/deploy.sh docker production --dry-run
```

## Kubernetes Deployment

### Prerequisites

1. **Kubernetes Cluster**: Running cluster with kubectl access
2. **Container Registry**: Access to push/pull images
3. **Helm** (optional): For package management

### Basic Kubernetes Deployment

#### 1. Create Namespace
```bash
kubectl create namespace brain-ai
```

#### 2. Deploy Application
```bash
# Apply manifests
kubectl apply -f k8s/ -n brain-ai

# Or use Helm (if charts available)
helm install brain-ai ./helm/brain-ai -n brain-ai
```

#### 3. Verify Deployment
```bash
# Check pods
kubectl get pods -n brain-ai

# Check services
kubectl get services -n brain-ai

# View logs
kubectl logs -f deployment/brain-ai -n brain-ai
```

### Using Deployment Script

```bash
# Deploy to Kubernetes
./scripts/deploy.sh kubernetes production --namespace=brain-ai

# Deploy with custom image
./scripts/deploy.sh kubernetes production --version=v1.2.3 --build --push

# Rollback deployment
./scripts/deploy.sh kubernetes production --rollback
```

### Kubernetes Configuration Examples

#### Deployment Manifest
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: brain-ai
  namespace: brain-ai
spec:
  replicas: 3
  selector:
    matchLabels:
      app: brain-ai
  template:
    metadata:
      labels:
        app: brain-ai
    spec:
      containers:
      - name: brain-ai
        image: ghcr.io/brain-ai:latest
        ports:
        - containerPort: 8080
        env:
        - name: ANTHROPIC_API_KEY
          valueFrom:
            secretKeyRef:
              name: brain-ai-secrets
              key: anthropic-api-key
        resources:
          limits:
            cpu: 2000m
            memory: 4Gi
          requests:
            cpu: 1000m
            memory: 2Gi
```

#### Service Manifest
```yaml
apiVersion: v1
kind: Service
metadata:
  name: brain-ai-service
  namespace: brain-ai
spec:
  selector:
    app: brain-ai
  ports:
  - port: 80
    targetPort: 8080
  type: LoadBalancer
```

## Local Development

### Setup for Development

1. **Clone Repository**:
   ```bash
   git clone <repository-url> brain-ai
   cd brain-ai
   ```

2. **Install Dependencies**:
   ```bash
   cargo build
   ```

3. **Setup Configuration**:
   ```bash
   cp scripts/config.toml config.toml
   cp .env.example .env
   # Edit .env with your API keys
   ```

4. **Run Development Server**:
   ```bash
   # Using deployment script
   ./scripts/deploy.sh local development

   # Or manually
   cargo run
   ```

### Development Workflow

1. **Make Changes**: Edit source code
2. **Test**: Run tests with `cargo test`
3. **Build**: Build with `cargo build`
4. **Run**: Test locally with `cargo run`
5. **Deploy**: Deploy to staging with deployment script

## Production Deployment

### Pre-Deployment Checklist

- [ ] **Environment Configuration**: All required environment variables set
- [ ] **API Keys**: Valid Anthropic API key configured
- [ ] **Resources**: Sufficient CPU, memory, and storage
- [ ] **Network**: Proper firewall and network configuration
- [ ] **Backup**: Backup strategy in place
- [ ] **Monitoring**: Health checks and monitoring configured
- [ ] **Security**: Security best practices implemented

### Production Deployment Steps

1. **Backup Current System**:
   ```bash
   ./scripts/backup.sh --name "pre-production-deploy"
   ```

2. **Deploy New Version**:
   ```bash
   ./scripts/deploy.sh production --build --push --health-check
   ```

3. **Verify Deployment**:
   ```bash
   ./scripts/health_check.sh --full
   ```

4. **Monitor System**:
   ```bash
   # Check logs
   docker-compose logs -f

   # Monitor metrics (if Prometheus enabled)
   curl http://localhost:9090/metrics
   ```

### Rolling Updates

#### Docker Compose
```bash
# Update image version
export BRAIN_AI_VERSION=v1.2.3
docker-compose pull
docker-compose up -d
```

#### Kubernetes
```bash
# Update deployment
kubectl set image deployment/brain-ai brain-ai=ghcr.io/brain-ai:v1.2.3 -n brain-ai

# Monitor rollout
kubectl rollout status deployment/brain-ai -n brain-ai
```

### Rollback Procedures

#### Automatic Rollback
```bash
./scripts/deploy.sh production --rollback
```

#### Manual Rollback
```bash
# Docker Compose
docker-compose down
export BRAIN_AI_VERSION=previous-version
docker-compose up -d

# Kubernetes
kubectl rollout undo deployment/brain-ai -n brain-ai
```

## Monitoring & Health Checks

### Built-in Health Checks

The system includes comprehensive health monitoring:

```bash
# Run health checks
./scripts/health_check.sh

# Quick health check
./scripts/health_check.sh --quick

# API-only health check
./scripts/health_check.sh --api-only

# JSON output for monitoring systems
./scripts/health_check.sh --json
```

### Health Check Endpoints

- **`GET /health`**: Basic health status
- **`GET /health/detailed`**: Detailed system status
- **`GET /metrics`**: Prometheus metrics (if enabled)

### Monitoring Stack (Optional)

Enable monitoring with:
```bash
# In docker-compose.yml, uncomment monitoring services
docker-compose up -d prometheus grafana

# Access dashboards
# Prometheus: http://localhost:9090
# Grafana: http://localhost:3000 (admin/admin)
```

### Log Management

#### Log Locations
- **Container logs**: `docker-compose logs`
- **File logs**: `./logs/brain-ai.log`
- **System logs**: `/var/log/brain-ai/`

#### Log Rotation
```bash
# Configure logrotate (Linux)
sudo cp scripts/logrotate.conf /etc/logrotate.d/brain-ai
```

## Backup & Recovery

### Automated Backups

```bash
# Create backup
./scripts/backup.sh

# Create compressed backup
./scripts/backup.sh --compress

# Data-only backup
./scripts/backup.sh --data-only

# Custom backup with retention
./scripts/backup.sh --retention 7 --name "weekly-backup"
```

### Backup Strategy

#### Development
- **Frequency**: Daily
- **Retention**: 7 days
- **Type**: Data + configuration

#### Production
- **Frequency**: Every 6 hours
- **Retention**: 30 days
- **Type**: Full backup with verification

### Recovery Procedures

```bash
# List available backups
./scripts/restore.sh --list

# Restore from backup
./scripts/restore.sh /path/to/backup.tar.gz

# Dry run (preview)
./scripts/restore.sh --dry-run backup.tar.gz

# Data-only restore
./scripts/restore.sh --data-only backup.tar.gz
```

## Troubleshooting

### Common Issues

#### 1. Container Fails to Start
```bash
# Check logs
docker-compose logs brain-ai

# Check resource usage
docker stats

# Verify configuration
docker-compose config
```

#### 2. API Not Responding
```bash
# Check if service is running
curl -I http://localhost:8080/health

# Check port binding
netstat -tlnp | grep 8080

# Check firewall
sudo ufw status
```

#### 3. Database Connection Issues
```bash
# Check Neo4j status
docker-compose logs neo4j

# Test connection
nc -zv localhost 7687

# Verify credentials
docker-compose exec neo4j cypher-shell -u neo4j -p password
```

#### 4. Performance Issues
```bash
# Run performance check
./scripts/health_check.sh --full

# Check system resources
top
df -h
free -m

# Analyze logs for errors
grep -i error logs/brain-ai.log
```

### Debug Mode

Enable debug mode for troubleshooting:
```bash
# Set environment variable
export RUST_LOG=debug
export BRAIN_AI_LOG_LEVEL=debug

# Restart services
docker-compose restart brain-ai
```

### Support Information

When reporting issues, include:
1. **Environment details**: OS, Docker version, deployment method
2. **Configuration**: Sanitized config files (remove API keys)
3. **Logs**: Recent log entries showing the issue
4. **Health check output**: `./scripts/health_check.sh --verbose`
5. **System resources**: CPU, memory, disk usage

## Security Considerations

### API Security

1. **API Keys**: Store in environment variables, never in code
2. **HTTPS**: Use TLS/SSL in production
3. **Rate Limiting**: Configure appropriate rate limits
4. **CORS**: Configure CORS for web interfaces

### Container Security

1. **Image Scanning**: Scan images for vulnerabilities
2. **Non-root User**: Run containers as non-root user
3. **Resource Limits**: Set CPU and memory limits
4. **Network Isolation**: Use Docker networks for isolation

### Data Security

1. **Encryption**: Encrypt sensitive data at rest
2. **Backup Encryption**: Encrypt backup files
3. **Access Control**: Limit access to data directories
4. **Audit Logging**: Enable audit logs for data access

### Network Security

1. **Firewall**: Configure firewall rules
2. **VPN**: Use VPN for remote access
3. **Private Networks**: Deploy in private networks
4. **SSL/TLS**: Use encrypted connections

### Example Security Configuration

```yaml
# docker-compose.yml security settings
services:
  brain-ai:
    user: "1000:1000"  # Non-root user
    read_only: true     # Read-only filesystem
    tmpfs:
      - /tmp
      - /var/tmp
    cap_drop:
      - ALL
    cap_add:
      - NET_BIND_SERVICE
    security_opt:
      - no-new-privileges:true
```

---

## Additional Resources

- **Configuration Reference**: See `config.toml` for all available options
- **API Documentation**: Available at `/docs` endpoint when running
- **Development Guide**: See `DEVELOPMENT.md`
- **Troubleshooting**: See `TROUBLESHOOTING.md`
- **Security Guide**: See `SECURITY.md`

For support or questions, please refer to the project documentation or contact the development team. 