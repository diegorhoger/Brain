#!/bin/bash

# Brain AI System Deployment Script
# This script handles deployment of the Brain AI system to various environments

set -euo pipefail

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
DEFAULT_ENV="production"
DEFAULT_REGISTRY="ghcr.io"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Logging
log() {
    echo -e "${BLUE}[$(date +'%Y-%m-%d %H:%M:%S')]${NC} $1"
}

error() {
    echo -e "${RED}[ERROR]${NC} $1" >&2
}

success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

info() {
    echo -e "${CYAN}[INFO]${NC} $1"
}

# Help function
show_help() {
    cat << EOF
Brain AI System Deployment Script

Usage: $0 [OPTIONS] ENVIRONMENT

Arguments:
    ENVIRONMENT         Target environment (production, staging, development)

Options:
    -h, --help          Show this help message
    -v, --version TAG   Docker image version tag (default: latest)
    -r, --registry URL  Docker registry URL (default: $DEFAULT_REGISTRY)
    -n, --namespace NS  Kubernetes namespace (default: brain-ai)
    --build             Build Docker image before deployment
    --push              Push Docker image to registry
    --no-backup         Skip backup before deployment
    --dry-run           Show what would be deployed without doing it
    --force             Force deployment without confirmations
    --rollback          Rollback to previous deployment
    --health-check      Run health checks after deployment
    --logs              Show deployment logs
    --cleanup           Clean up old deployments

Deployment Types:
    docker              Deploy using Docker Compose
    kubernetes          Deploy to Kubernetes cluster
    local               Deploy locally for development

Examples:
    $0 production                           # Deploy to production
    $0 staging --build --push               # Build and deploy to staging
    $0 development --dry-run                # Preview development deployment
    $0 production --rollback                # Rollback production deployment
    $0 kubernetes production --namespace=prod  # Deploy to Kubernetes
EOF
}

# Default options
ENVIRONMENT=""
VERSION="latest"
REGISTRY="$DEFAULT_REGISTRY"
NAMESPACE="brain-ai"
BUILD=false
PUSH=false
NO_BACKUP=false
DRY_RUN=false
FORCE=false
ROLLBACK=false
HEALTH_CHECK=false
SHOW_LOGS=false
CLEANUP=false
DEPLOYMENT_TYPE="docker"

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -h|--help)
            show_help
            exit 0
            ;;
        -v|--version)
            VERSION="$2"
            shift 2
            ;;
        -r|--registry)
            REGISTRY="$2"
            shift 2
            ;;
        -n|--namespace)
            NAMESPACE="$2"
            shift 2
            ;;
        --build)
            BUILD=true
            shift
            ;;
        --push)
            PUSH=true
            shift
            ;;
        --no-backup)
            NO_BACKUP=true
            shift
            ;;
        --dry-run)
            DRY_RUN=true
            shift
            ;;
        --force)
            FORCE=true
            shift
            ;;
        --rollback)
            ROLLBACK=true
            shift
            ;;
        --health-check)
            HEALTH_CHECK=true
            shift
            ;;
        --logs)
            SHOW_LOGS=true
            shift
            ;;
        --cleanup)
            CLEANUP=true
            shift
            ;;
        docker|kubernetes|local)
            DEPLOYMENT_TYPE="$1"
            shift
            ;;
        -*)
            error "Unknown option: $1"
            show_help
            exit 1
            ;;
        *)
            if [[ -z "$ENVIRONMENT" ]]; then
                ENVIRONMENT="$1"
            else
                error "Multiple environments specified"
                exit 1
            fi
            shift
            ;;
    esac
done

# Set default environment if not specified
if [[ -z "$ENVIRONMENT" ]]; then
    ENVIRONMENT="$DEFAULT_ENV"
fi

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function to check dependencies
check_dependencies() {
    local missing_deps=()
    
    case "$DEPLOYMENT_TYPE" in
        docker)
            if ! command_exists docker; then
                missing_deps+=("docker")
            fi
            if ! command_exists docker-compose; then
                missing_deps+=("docker-compose")
            fi
            ;;
        kubernetes)
            if ! command_exists kubectl; then
                missing_deps+=("kubectl")
            fi
            if ! command_exists helm; then
                missing_deps+=("helm")
            fi
            ;;
    esac
    
    if [[ ${#missing_deps[@]} -gt 0 ]]; then
        error "Missing required dependencies: ${missing_deps[*]}"
        exit 1
    fi
}

# Function to validate environment
validate_environment() {
    local valid_envs=("production" "staging" "development" "dev" "prod" "stage")
    local env_valid=false
    
    for valid_env in "${valid_envs[@]}"; do
        if [[ "$ENVIRONMENT" == "$valid_env" ]]; then
            env_valid=true
            break
        fi
    done
    
    if [[ "$env_valid" == false ]]; then
        error "Invalid environment: $ENVIRONMENT"
        error "Valid environments: ${valid_envs[*]}"
        exit 1
    fi
}

# Function to create backup before deployment
create_backup() {
    if [[ "$NO_BACKUP" == true ]]; then
        return 0
    fi
    
    log "Creating backup before deployment..."
    
    local backup_script="${SCRIPT_DIR}/backup.sh"
    if [[ -x "$backup_script" ]]; then
        local timestamp=$(date +"%Y%m%d_%H%M%S")
        if [[ "$DRY_RUN" == true ]]; then
            info "[DRY RUN] Would create backup: pre-deploy-${ENVIRONMENT}-${timestamp}"
        else
            "$backup_script" --name "pre-deploy-${ENVIRONMENT}-${timestamp}" --compress
            success "Pre-deployment backup created"
        fi
    else
        warning "Backup script not found, skipping backup"
    fi
}

# Function to build Docker image
build_image() {
    if [[ "$BUILD" == false ]]; then
        return 0
    fi
    
    log "Building Docker image..."
    
    local image_name="brain-ai"
    local full_image_name="${REGISTRY}/${image_name}:${VERSION}"
    
    cd "$PROJECT_ROOT"
    
    if [[ "$DRY_RUN" == true ]]; then
        info "[DRY RUN] Would build: $full_image_name"
    else
        docker build -t "$full_image_name" .
        success "Docker image built: $full_image_name"
    fi
    
    # Tag as latest if this is a release
    if [[ "$VERSION" != "latest" && "$ENVIRONMENT" == "production" ]]; then
        local latest_tag="${REGISTRY}/${image_name}:latest"
        if [[ "$DRY_RUN" == true ]]; then
            info "[DRY RUN] Would tag as latest: $latest_tag"
        else
            docker tag "$full_image_name" "$latest_tag"
            info "Tagged as latest: $latest_tag"
        fi
    fi
}

# Function to push Docker image
push_image() {
    if [[ "$PUSH" == false ]]; then
        return 0
    fi
    
    log "Pushing Docker image to registry..."
    
    local image_name="brain-ai"
    local full_image_name="${REGISTRY}/${image_name}:${VERSION}"
    
    if [[ "$DRY_RUN" == true ]]; then
        info "[DRY RUN] Would push: $full_image_name"
    else
        docker push "$full_image_name"
        success "Docker image pushed: $full_image_name"
        
        # Push latest tag if it exists
        if docker images "${REGISTRY}/${image_name}:latest" --format "table {{.Repository}}" | grep -q "${REGISTRY}/${image_name}"; then
            docker push "${REGISTRY}/${image_name}:latest"
            info "Latest tag pushed"
        fi
    fi
}

# Function to deploy with Docker Compose
deploy_docker() {
    log "Deploying with Docker Compose..."
    
    cd "$PROJECT_ROOT"
    
    # Check if docker-compose.yml exists
    if [[ ! -f "docker-compose.yml" ]]; then
        error "docker-compose.yml not found"
        exit 1
    fi
    
    # Set environment variables
    export BRAIN_AI_VERSION="$VERSION"
    export BRAIN_AI_ENVIRONMENT="$ENVIRONMENT"
    export BRAIN_AI_REGISTRY="$REGISTRY"
    
    # Create environment-specific override file
    local compose_override="docker-compose.${ENVIRONMENT}.yml"
    if [[ -f "$compose_override" ]]; then
        info "Using environment-specific override: $compose_override"
        export COMPOSE_FILE="docker-compose.yml:$compose_override"
    fi
    
    if [[ "$DRY_RUN" == true ]]; then
        info "[DRY RUN] Would run: docker-compose up -d"
        docker-compose config
    else
        # Pull latest images
        docker-compose pull
        
        # Deploy
        docker-compose up -d
        success "Docker Compose deployment completed"
        
        # Show status
        docker-compose ps
    fi
}

# Function to deploy to Kubernetes
deploy_kubernetes() {
    log "Deploying to Kubernetes..."
    
    # Check if kubectl is configured
    if ! kubectl cluster-info >/dev/null 2>&1; then
        error "kubectl is not configured or cluster is not accessible"
        exit 1
    fi
    
    # Create namespace if it doesn't exist
    if [[ "$DRY_RUN" == true ]]; then
        info "[DRY RUN] Would create namespace: $NAMESPACE"
    else
        kubectl create namespace "$NAMESPACE" --dry-run=client -o yaml | kubectl apply -f -
        info "Namespace ensured: $NAMESPACE"
    fi
    
    # Deploy using Helm if charts exist
    local helm_chart_dir="${PROJECT_ROOT}/helm/brain-ai"
    if [[ -d "$helm_chart_dir" ]]; then
        deploy_helm
    else
        deploy_kubectl
    fi
}

# Function to deploy with Helm
deploy_helm() {
    log "Deploying with Helm..."
    
    local chart_dir="${PROJECT_ROOT}/helm/brain-ai"
    local release_name="brain-ai-${ENVIRONMENT}"
    
    # Create values file for environment
    local values_file="${chart_dir}/values-${ENVIRONMENT}.yaml"
    if [[ ! -f "$values_file" ]]; then
        values_file="${chart_dir}/values.yaml"
    fi
    
    local helm_args=(
        "upgrade" "--install"
        "$release_name" "$chart_dir"
        "--namespace" "$NAMESPACE"
        "--values" "$values_file"
        "--set" "image.tag=${VERSION}"
        "--set" "environment=${ENVIRONMENT}"
    )
    
    if [[ "$DRY_RUN" == true ]]; then
        helm_args+=("--dry-run")
        info "[DRY RUN] Helm deployment preview:"
    fi
    
    helm "${helm_args[@]}"
    
    if [[ "$DRY_RUN" == false ]]; then
        success "Helm deployment completed"
        
        # Wait for deployment to be ready
        kubectl rollout status deployment/brain-ai -n "$NAMESPACE" --timeout=300s
    fi
}

# Function to deploy with kubectl
deploy_kubectl() {
    log "Deploying with kubectl..."
    
    local k8s_dir="${PROJECT_ROOT}/k8s"
    if [[ ! -d "$k8s_dir" ]]; then
        error "Kubernetes manifests directory not found: $k8s_dir"
        exit 1
    fi
    
    # Apply manifests
    local manifest_files=("$k8s_dir"/*.yaml "$k8s_dir"/*.yml)
    
    for manifest in "${manifest_files[@]}"; do
        if [[ -f "$manifest" ]]; then
            if [[ "$DRY_RUN" == true ]]; then
                info "[DRY RUN] Would apply: $(basename "$manifest")"
                kubectl apply -f "$manifest" --dry-run=client
            else
                kubectl apply -f "$manifest" -n "$NAMESPACE"
                info "Applied: $(basename "$manifest")"
            fi
        fi
    done
    
    if [[ "$DRY_RUN" == false ]]; then
        success "kubectl deployment completed"
    fi
}

# Function to deploy locally
deploy_local() {
    log "Deploying locally for development..."
    
    cd "$PROJECT_ROOT"
    
    # Build the project
    if [[ "$DRY_RUN" == true ]]; then
        info "[DRY RUN] Would run: cargo build --release"
    else
        cargo build --release
        success "Project built successfully"
    fi
    
    # Create necessary directories
    local dirs=("data" "logs" "tmp")
    for dir in "${dirs[@]}"; do
        if [[ ! -d "$dir" ]]; then
            if [[ "$DRY_RUN" == true ]]; then
                info "[DRY RUN] Would create directory: $dir"
            else
                mkdir -p "$dir"
                info "Created directory: $dir"
            fi
        fi
    done
    
    # Copy configuration files
    if [[ -f "scripts/config.toml" && ! -f "config.toml" ]]; then
        if [[ "$DRY_RUN" == true ]]; then
            info "[DRY RUN] Would copy: scripts/config.toml -> config.toml"
        else
            cp "scripts/config.toml" "config.toml"
            info "Configuration file copied"
        fi
    fi
    
    if [[ "$DRY_RUN" == false ]]; then
        success "Local deployment completed"
        info "Run './target/release/brain-ai' to start the system"
    fi
}

# Function to rollback deployment
rollback_deployment() {
    log "Rolling back deployment..."
    
    case "$DEPLOYMENT_TYPE" in
        docker)
            # Find previous version from docker images
            local previous_version=$(docker images --format "table {{.Tag}}" brain-ai | grep -v "latest" | grep -v "TAG" | head -2 | tail -1)
            if [[ -n "$previous_version" ]]; then
                info "Rolling back to version: $previous_version"
                VERSION="$previous_version"
                deploy_docker
            else
                error "No previous version found for rollback"
                exit 1
            fi
            ;;
        kubernetes)
            if command_exists helm; then
                local release_name="brain-ai-${ENVIRONMENT}"
                if [[ "$DRY_RUN" == true ]]; then
                    info "[DRY RUN] Would rollback Helm release: $release_name"
                else
                    helm rollback "$release_name" -n "$NAMESPACE"
                    success "Helm rollback completed"
                fi
            else
                if [[ "$DRY_RUN" == true ]]; then
                    info "[DRY RUN] Would rollback kubectl deployment"
                else
                    kubectl rollout undo deployment/brain-ai -n "$NAMESPACE"
                    success "kubectl rollback completed"
                fi
            fi
            ;;
        *)
            error "Rollback not supported for deployment type: $DEPLOYMENT_TYPE"
            exit 1
            ;;
    esac
}

# Function to run health checks
run_health_checks() {
    if [[ "$HEALTH_CHECK" == false ]]; then
        return 0
    fi
    
    log "Running post-deployment health checks..."
    
    local health_script="${SCRIPT_DIR}/health_check.sh"
    if [[ -x "$health_script" ]]; then
        if [[ "$DRY_RUN" == true ]]; then
            info "[DRY RUN] Would run health checks"
        else
            sleep 30  # Wait for services to start
            if "$health_script" --api-only; then
                success "Health checks passed"
            else
                error "Health checks failed"
                exit 1
            fi
        fi
    else
        warning "Health check script not found"
    fi
}

# Function to show deployment logs
show_deployment_logs() {
    if [[ "$SHOW_LOGS" == false ]]; then
        return 0
    fi
    
    log "Showing deployment logs..."
    
    case "$DEPLOYMENT_TYPE" in
        docker)
            docker-compose logs -f --tail=50
            ;;
        kubernetes)
            kubectl logs -f deployment/brain-ai -n "$NAMESPACE" --tail=50
            ;;
        local)
            if [[ -f "${PROJECT_ROOT}/logs/brain-ai.log" ]]; then
                tail -f "${PROJECT_ROOT}/logs/brain-ai.log"
            else
                info "No log file found yet"
            fi
            ;;
    esac
}

# Function to cleanup old deployments
cleanup_old_deployments() {
    if [[ "$CLEANUP" == false ]]; then
        return 0
    fi
    
    log "Cleaning up old deployments..."
    
    case "$DEPLOYMENT_TYPE" in
        docker)
            # Remove unused images
            if [[ "$DRY_RUN" == true ]]; then
                info "[DRY RUN] Would clean up Docker images"
            else
                docker image prune -f
                info "Docker cleanup completed"
            fi
            ;;
        kubernetes)
            # Clean up old replica sets
            if [[ "$DRY_RUN" == true ]]; then
                info "[DRY RUN] Would clean up old Kubernetes resources"
            else
                kubectl delete replicaset --field-selector=status.replicas=0 -n "$NAMESPACE"
                info "Kubernetes cleanup completed"
            fi
            ;;
    esac
}

# Function to confirm deployment
confirm_deployment() {
    if [[ "$FORCE" == true || "$DRY_RUN" == true ]]; then
        return 0
    fi
    
    echo
    warning "You are about to deploy to: $ENVIRONMENT"
    echo "Deployment type: $DEPLOYMENT_TYPE"
    echo "Version: $VERSION"
    echo "Registry: $REGISTRY"
    if [[ "$DEPLOYMENT_TYPE" == "kubernetes" ]]; then
        echo "Namespace: $NAMESPACE"
    fi
    echo
    
    read -p "Are you sure you want to continue? (y/N): " -n 1 -r
    echo
    
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        log "Deployment cancelled by user"
        exit 0
    fi
}

# Function to display deployment summary
show_summary() {
    echo
    echo "=================================="
    echo "    DEPLOYMENT SUMMARY"
    echo "=================================="
    echo "Environment:     $ENVIRONMENT"
    echo "Type:            $DEPLOYMENT_TYPE"
    echo "Version:         $VERSION"
    echo "Registry:        $REGISTRY"
    if [[ "$DEPLOYMENT_TYPE" == "kubernetes" ]]; then
        echo "Namespace:       $NAMESPACE"
    fi
    echo "Dry Run:         $DRY_RUN"
    echo "Timestamp:       $(date)"
    echo "=================================="
    echo
}

# Main deployment function
main() {
    log "Starting Brain AI system deployment..."
    
    # Validate inputs
    validate_environment
    check_dependencies
    
    # Show what will be deployed
    if [[ "$DRY_RUN" == false ]]; then
        confirm_deployment
    fi
    
    # Handle rollback
    if [[ "$ROLLBACK" == true ]]; then
        rollback_deployment
        run_health_checks
        show_summary
        return 0
    fi
    
    # Create backup
    create_backup
    
    # Build and push image if requested
    build_image
    push_image
    
    # Deploy based on type
    case "$DEPLOYMENT_TYPE" in
        docker)
            deploy_docker
            ;;
        kubernetes)
            deploy_kubernetes
            ;;
        local)
            deploy_local
            ;;
        *)
            error "Unknown deployment type: $DEPLOYMENT_TYPE"
            exit 1
            ;;
    esac
    
    # Post-deployment tasks
    run_health_checks
    cleanup_old_deployments
    show_deployment_logs &  # Run in background
    
    # Show summary
    show_summary
    
    if [[ "$DRY_RUN" == true ]]; then
        success "Dry run completed successfully!"
    else
        success "Deployment completed successfully!"
    fi
}

# Run main function
main 