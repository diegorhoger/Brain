# Brain AI Scripts Directory

This directory contains essential scripts and configuration files for managing the Brain AI system deployment, maintenance, and operations.

## Scripts Overview

### 🚀 Deployment & Operations

#### `deploy.sh`
Comprehensive deployment script supporting multiple environments and platforms.

**Usage:**
```bash
# Deploy to production with Docker Compose
./deploy.sh docker production --build --health-check

# Deploy to Kubernetes with custom version
./deploy.sh kubernetes staging --version=v1.2.3 --namespace=brain-ai-staging

# Local development deployment
./deploy.sh local development

# Preview deployment (dry run)
./deploy.sh production --dry-run

# Rollback to previous version
./deploy.sh production --rollback
```

**Features:**
- Multi-platform support (Docker, Kubernetes, Local)
- Environment-specific configurations
- Automated backup before deployment
- Health checks and verification
- Rollback capabilities
- Image building and registry push

### 💾 Backup & Recovery

#### `backup.sh`
Creates comprehensive backups of system data, configuration, and source code.

**Usage:**
```bash
# Full system backup
./backup.sh

# Data-only backup
./backup.sh --data-only

# Configuration-only backup
./backup.sh --config-only

# Custom backup with verification
./backup.sh --name "pre-deployment" --verify --retention 7

# Compressed backup
./backup.sh --compress
```

**Features:**
- Full, data-only, or config-only backups
- Automatic compression and verification
- Metadata and checksums
- Configurable retention policies
- Git information capture

#### `restore.sh`
Restores system from backups with verification and safety checks.

**Usage:**
```bash
# List available backups
./restore.sh --list

# Restore from compressed backup
./restore.sh backup_20231201_120000.tar.gz

# Preview restore operation
./restore.sh --dry-run backup.tar.gz

# Force restore without confirmation
./restore.sh --force backup.tar.gz

# Restore only data files
./restore.sh --data-only backup.tar.gz
```

**Features:**
- Backup integrity verification
- Pre-restore backup creation
- Selective restoration (data/config/source)
- Dry-run capability
- Automatic extraction handling

### 🏥 Health & Monitoring

#### `health_check.sh`
Comprehensive system health monitoring and diagnostics.

**Usage:**
```bash
# Standard health check
./health_check.sh

# Verbose output with details
./health_check.sh --verbose

# Quick check (skip performance tests)
./health_check.sh --quick

# API endpoints only
./health_check.sh --api-only

# JSON output for monitoring systems
./health_check.sh --json

# Full check with automatic fixes
./health_check.sh --full --fix
```

**Health Checks Include:**
- System resources (CPU, memory, disk)
- Rust toolchain and dependencies
- Project structure and configuration
- Database connections
- API endpoint availability
- Build and test status
- Docker setup
- Log file analysis

## Configuration Files

### `config.toml`
Master configuration file with all system settings.

**Key Sections:**
- **System**: Basic system identification and metadata
- **Logging**: Comprehensive logging configuration
- **Performance**: Monitoring and performance settings
- **Memory**: Memory system configuration
- **AI Models**: Character prediction and segment discovery
- **Databases**: Database connection settings
- **Security**: Authentication and security settings
- **Experimental**: Feature flags and experimental options

**Example Configuration:**
```toml
[system]
id = "brain-ai-system"
name = "Brain AI Cognitive Architecture"
version = "1.0.0"

[performance]
enable_monitoring = true
enable_health_checks = true
max_concurrent_operations = 50

[memory]
working_memory_capacity = 1000
episodic_memory_retention_days = 365
```

### `.env.example`
Template for environment variables with comprehensive documentation.

**Required Variables:**
- `ANTHROPIC_API_KEY`: Anthropic API key for Claude
- `BRAIN_AI_ENVIRONMENT`: Deployment environment
- `RUST_LOG`: Rust logging level

**Optional Variables:**
- `PERPLEXITY_API_KEY`: For research-enhanced features
- `NEO4J_*`: Neo4j database configuration
- `PROMETHEUS_*`: Monitoring configuration

## Docker Configuration

### `Dockerfile`
Multi-stage Docker build with optimization for production deployment.

**Build Stages:**
1. **Builder**: Rust compilation with dependency caching
2. **Runtime**: Minimal runtime image with security hardening

**Features:**
- Dependency layer caching for faster builds
- Non-root user execution
- Health check integration
- Optimized binary size

### `docker-compose.yml`
Complete service orchestration with optional components.

**Services:**
- **brain-ai**: Main application service
- **neo4j**: Graph database (optional)
- **redis**: Caching and session storage (optional)
- **prometheus**: Metrics collection (optional)
- **grafana**: Monitoring dashboards (optional)

**Usage:**
```bash
# Start core services
docker-compose up -d brain-ai

# Start with all optional services
docker-compose --profile full up -d

# Production deployment
docker-compose -f docker-compose.yml -f docker-compose.prod.yml up -d
```

## Usage Patterns

### Development Workflow

1. **Setup Environment**:
   ```bash
   cp .env.example .env
   # Edit .env with your API keys
   ```

2. **Health Check**:
   ```bash
   ./health_check.sh --verbose
   ```

3. **Local Deployment**:
   ```bash
   ./deploy.sh local development
   ```

4. **Run Tests**:
   ```bash
   cargo test
   ```

### Production Deployment

1. **Pre-Deployment Backup**:
   ```bash
   ./backup.sh --name "pre-production-$(date +%Y%m%d)"
   ```

2. **Deploy**:
   ```bash
   ./deploy.sh docker production --build --push --health-check
   ```

3. **Verify**:
   ```bash
   ./health_check.sh --full
   ```

4. **Monitor**:
   ```bash
   docker-compose logs -f brain-ai
   ```

### Maintenance Tasks

#### Weekly Backup
```bash
# Automated weekly backup
./backup.sh --name "weekly-$(date +%Y%m%d)" --compress --verify
```

#### Health Monitoring
```bash
# Daily health check with JSON output for monitoring
./health_check.sh --json > /var/log/brain-ai-health.json
```

#### System Cleanup
```bash
# Clean up old Docker images and containers
docker system prune -f

# Clean up old backups (automatic with retention policy)
./backup.sh --retention 30
```

## Script Dependencies

### Required Commands
- `bash` (4.0+)
- `docker` and `docker-compose` (for container deployment)
- `kubectl` and `helm` (for Kubernetes deployment)
- `curl` (for API health checks)
- `jq` (for JSON processing, optional but recommended)

### Optional Commands
- `nc` or `telnet` (for port connectivity tests)
- `sha256sum` or `shasum` (for backup verification)
- `tar` and `gzip` (for backup compression)
- `bc` (for performance calculations)

## Environment Variables

### Script Configuration
- `BRAIN_AI_ENVIRONMENT`: Target environment (development/staging/production)
- `BRAIN_AI_VERSION`: Docker image version tag
- `BRAIN_AI_REGISTRY`: Docker registry URL
- `DEBUG`: Enable debug output in scripts

### System Configuration
- `ANTHROPIC_API_KEY`: Required for AI functionality
- `PERPLEXITY_API_KEY`: Optional for research features
- `RUST_LOG`: Rust logging configuration
- `NEO4J_*`: Neo4j database settings

## Security Considerations

### Script Security
- All scripts use `set -euo pipefail` for error handling
- Sensitive information is never logged or displayed
- Temporary files are cleaned up automatically
- User confirmation required for destructive operations

### Deployment Security
- API keys stored in environment variables only
- Docker images run as non-root user
- Network isolation using Docker networks
- Resource limits configured for all services

### Backup Security
- Checksums generated for all backup files
- Backup verification before restoration
- Configurable retention policies
- Support for encrypted backup storage

## Troubleshooting

### Common Issues

#### Permission Errors
```bash
# Make scripts executable
chmod +x scripts/*.sh

# Fix Docker permissions
sudo usermod -aG docker $USER
newgrp docker
```

#### Missing Dependencies
```bash
# Install required tools (Ubuntu/Debian)
sudo apt-get update
sudo apt-get install curl jq netcat-openbsd

# Install Docker
curl -fsSL https://get.docker.com | sh
```

#### Configuration Issues
```bash
# Validate configuration
./health_check.sh --config-only

# Check environment variables
env | grep BRAIN_AI
```

### Debug Mode

Enable debug output for all scripts:
```bash
export DEBUG=true
./script_name.sh
```

### Log Locations
- Script logs: `./logs/scripts/`
- System logs: `./logs/brain-ai.log`
- Docker logs: `docker-compose logs`

## Contributing

### Adding New Scripts

1. **Follow naming convention**: `action_noun.sh` (e.g., `backup.sh`, `deploy.sh`)
2. **Include help function**: Use `--help` flag for usage information
3. **Error handling**: Use `set -euo pipefail` and proper error checking
4. **Logging**: Use consistent logging functions (log, error, success, warning)
5. **Documentation**: Update this README with new script information

### Script Template

```bash
#!/bin/bash
set -euo pipefail

# Colors and logging functions
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

log() { echo -e "${BLUE}[$(date +'%Y-%m-%d %H:%M:%S')]${NC} $1"; }
error() { echo -e "${RED}[ERROR]${NC} $1" >&2; }
success() { echo -e "${GREEN}[SUCCESS]${NC} $1"; }
warning() { echo -e "${YELLOW}[WARNING]${NC} $1"; }

# Help function
show_help() {
    cat << EOF
Script Description

Usage: $0 [OPTIONS]

Options:
    -h, --help          Show this help message
    
Examples:
    $0                  # Basic usage
EOF
}

# Main function
main() {
    log "Starting script..."
    # Script logic here
    success "Script completed!"
}

# Parse arguments and run
while [[ $# -gt 0 ]]; do
    case $1 in
        -h|--help) show_help; exit 0 ;;
        *) error "Unknown option: $1"; exit 1 ;;
    esac
done

main
```

## Support

For issues with scripts or deployment:

1. **Check logs**: Review script output and system logs
2. **Run health check**: Use `./health_check.sh --verbose`
3. **Verify configuration**: Ensure all required environment variables are set
4. **Check dependencies**: Verify all required tools are installed
5. **Consult documentation**: Review `DEPLOYMENT.md` for detailed guidance

---

*Last updated: December 2024* 