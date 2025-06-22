#!/bin/bash

# Brain AI System Health Check Script
# This script performs comprehensive health checks on the Brain AI system

set -euo pipefail

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
CONFIG_FILE="${PROJECT_ROOT}/config.toml"
ENV_FILE="${PROJECT_ROOT}/.env"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Health check results
CHECKS_TOTAL=0
CHECKS_PASSED=0
CHECKS_FAILED=0
CHECKS_WARNING=0

# Logging
log() {
    echo -e "${BLUE}[$(date +'%Y-%m-%d %H:%M:%S')]${NC} $1"
}

error() {
    echo -e "${RED}[ERROR]${NC} $1" >&2
    ((CHECKS_FAILED++))
}

success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
    ((CHECKS_PASSED++))
}

warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
    ((CHECKS_WARNING++))
}

info() {
    echo -e "${CYAN}[INFO]${NC} $1"
}

# Function to increment total checks
check_start() {
    ((CHECKS_TOTAL++))
}

# Help function
show_help() {
    cat << EOF
Brain AI System Health Check Script

Usage: $0 [OPTIONS]

Options:
    -h, --help          Show this help message
    -v, --verbose       Verbose output with detailed information
    -q, --quiet         Quiet mode (only errors and summary)
    --quick             Quick health check (skip performance tests)
    --full              Full health check including performance tests
    --api-only          Check only API endpoints
    --deps-only         Check only dependencies and requirements
    --fix               Attempt to fix common issues automatically
    --json              Output results in JSON format

Examples:
    $0                  # Standard health check
    $0 --verbose        # Detailed health check
    $0 --quick          # Quick health check
    $0 --api-only       # Check only API endpoints
    $0 --fix            # Health check with automatic fixes
EOF
}

# Default options
VERBOSE=false
QUIET=false
QUICK=false
FULL=false
API_ONLY=false
DEPS_ONLY=false
FIX=false
JSON_OUTPUT=false

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -h|--help)
            show_help
            exit 0
            ;;
        -v|--verbose)
            VERBOSE=true
            shift
            ;;
        -q|--quiet)
            QUIET=true
            shift
            ;;
        --quick)
            QUICK=true
            shift
            ;;
        --full)
            FULL=true
            shift
            ;;
        --api-only)
            API_ONLY=true
            shift
            ;;
        --deps-only)
            DEPS_ONLY=true
            shift
            ;;
        --fix)
            FIX=true
            shift
            ;;
        --json)
            JSON_OUTPUT=true
            shift
            ;;
        *)
            error "Unknown option: $1"
            show_help
            exit 1
            ;;
    esac
done

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function to check if port is open
port_open() {
    local host="$1"
    local port="$2"
    local timeout="${3:-5}"
    
    if command_exists nc; then
        nc -z -w"$timeout" "$host" "$port" >/dev/null 2>&1
    elif command_exists telnet; then
        timeout "$timeout" telnet "$host" "$port" >/dev/null 2>&1
    else
        # Fallback using /dev/tcp (bash built-in)
        timeout "$timeout" bash -c "</dev/tcp/$host/$port" >/dev/null 2>&1
    fi
}

# Function to get file size in bytes
get_file_size() {
    if [[ -f "$1" ]]; then
        if command_exists stat; then
            stat -c%s "$1" 2>/dev/null || echo "0"
        else
            wc -c < "$1" 2>/dev/null || echo "0"
        fi
    else
        echo "0"
    fi
}

# Function to check disk space
check_disk_space() {
    check_start
    
    if [[ "$QUIET" == false ]]; then
        log "Checking disk space..."
    fi
    
    local required_mb=1000  # Require at least 1GB free space
    
    if command_exists df; then
        local free_space_kb=$(df "$PROJECT_ROOT" | tail -1 | awk '{print $4}')
        local free_space_mb=$((free_space_kb / 1024))
        
        if [[ "$VERBOSE" == true ]]; then
            info "Available disk space: ${free_space_mb}MB"
        fi
        
        if [[ $free_space_mb -gt $required_mb ]]; then
            success "Sufficient disk space available (${free_space_mb}MB)"
        else
            error "Insufficient disk space. Required: ${required_mb}MB, Available: ${free_space_mb}MB"
        fi
    else
        warning "Cannot check disk space (df command not available)"
    fi
}

# Function to check memory usage
check_memory() {
    check_start
    
    if [[ "$QUIET" == false ]]; then
        log "Checking memory usage..."
    fi
    
    if command_exists free; then
        local total_mem=$(free -m | awk 'NR==2{print $2}')
        local used_mem=$(free -m | awk 'NR==2{print $3}')
        local usage_percent=$((used_mem * 100 / total_mem))
        
        if [[ "$VERBOSE" == true ]]; then
            info "Memory usage: ${used_mem}MB / ${total_mem}MB (${usage_percent}%)"
        fi
        
        if [[ $usage_percent -lt 90 ]]; then
            success "Memory usage is acceptable (${usage_percent}%)"
        elif [[ $usage_percent -lt 95 ]]; then
            warning "High memory usage (${usage_percent}%)"
        else
            error "Critical memory usage (${usage_percent}%)"
        fi
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        # macOS specific memory check
        local memory_info=$(vm_stat)
        if [[ "$VERBOSE" == true ]]; then
            info "Memory statistics available via vm_stat"
        fi
        success "Memory check completed (macOS)"
    else
        warning "Cannot check memory usage (free command not available)"
    fi
}

# Function to check Rust installation
check_rust() {
    check_start
    
    if [[ "$QUIET" == false ]]; then
        log "Checking Rust installation..."
    fi
    
    if command_exists rustc; then
        local rust_version=$(rustc --version 2>/dev/null || echo "unknown")
        if [[ "$VERBOSE" == true ]]; then
            info "Rust version: $rust_version"
        fi
        success "Rust compiler available"
    else
        error "Rust compiler not found"
        if [[ "$FIX" == true ]]; then
            info "Attempting to install Rust..."
            curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
            source "$HOME/.cargo/env"
            if command_exists rustc; then
                success "Rust installed successfully"
            else
                error "Failed to install Rust"
            fi
        fi
    fi
    
    if command_exists cargo; then
        local cargo_version=$(cargo --version 2>/dev/null || echo "unknown")
        if [[ "$VERBOSE" == true ]]; then
            info "Cargo version: $cargo_version"
        fi
        success "Cargo package manager available"
    else
        error "Cargo package manager not found"
    fi
}

# Function to check project structure
check_project_structure() {
    check_start
    
    if [[ "$QUIET" == false ]]; then
        log "Checking project structure..."
    fi
    
    local required_files=(
        "Cargo.toml"
        "src/lib.rs"
        "src/main.rs"
    )
    
    local required_dirs=(
        "src"
        "scripts"
    )
    
    local missing_files=()
    local missing_dirs=()
    
    # Check required files
    for file in "${required_files[@]}"; do
        if [[ ! -f "${PROJECT_ROOT}/${file}" ]]; then
            missing_files+=("$file")
        fi
    done
    
    # Check required directories
    for dir in "${required_dirs[@]}"; do
        if [[ ! -d "${PROJECT_ROOT}/${dir}" ]]; then
            missing_dirs+=("$dir")
        fi
    done
    
    if [[ ${#missing_files[@]} -eq 0 && ${#missing_dirs[@]} -eq 0 ]]; then
        success "Project structure is complete"
    else
        if [[ ${#missing_files[@]} -gt 0 ]]; then
            error "Missing files: ${missing_files[*]}"
        fi
        if [[ ${#missing_dirs[@]} -gt 0 ]]; then
            error "Missing directories: ${missing_dirs[*]}"
        fi
        
        if [[ "$FIX" == true ]]; then
            info "Attempting to create missing directories..."
            for dir in "${missing_dirs[@]}"; do
                mkdir -p "${PROJECT_ROOT}/${dir}"
                success "Created directory: $dir"
            done
        fi
    fi
}

# Function to check configuration files
check_configuration() {
    check_start
    
    if [[ "$QUIET" == false ]]; then
        log "Checking configuration files..."
    fi
    
    # Check main config file
    if [[ -f "$CONFIG_FILE" ]]; then
        if [[ "$VERBOSE" == true ]]; then
            local config_size=$(get_file_size "$CONFIG_FILE")
            info "Configuration file size: ${config_size} bytes"
        fi
        success "Main configuration file exists"
        
        # Validate TOML syntax if toml command is available
        if command_exists toml; then
            if toml get "$CONFIG_FILE" . >/dev/null 2>&1; then
                success "Configuration file syntax is valid"
            else
                error "Configuration file has invalid TOML syntax"
            fi
        fi
    else
        warning "Main configuration file not found: $CONFIG_FILE"
        if [[ "$FIX" == true ]]; then
            info "Creating default configuration file..."
            cp "${SCRIPT_DIR}/config.toml" "$CONFIG_FILE" 2>/dev/null || true
            if [[ -f "$CONFIG_FILE" ]]; then
                success "Created default configuration file"
            fi
        fi
    fi
    
    # Check environment file
    if [[ -f "$ENV_FILE" ]]; then
        success "Environment file exists"
    else
        warning "Environment file not found: $ENV_FILE"
        if [[ -f "${PROJECT_ROOT}/.env.example" ]]; then
            info "Example environment file available"
            if [[ "$FIX" == true ]]; then
                cp "${PROJECT_ROOT}/.env.example" "$ENV_FILE"
                success "Created environment file from example"
            fi
        fi
    fi
}

# Function to check dependencies
check_dependencies() {
    check_start
    
    if [[ "$QUIET" == false ]]; then
        log "Checking project dependencies..."
    fi
    
    if [[ -f "${PROJECT_ROOT}/Cargo.toml" ]]; then
        cd "$PROJECT_ROOT"
        
        # Check if Cargo.lock exists
        if [[ -f "Cargo.lock" ]]; then
            success "Dependency lock file exists"
        else
            warning "Dependency lock file missing (run 'cargo build' to generate)"
            if [[ "$FIX" == true ]]; then
                info "Generating dependency lock file..."
                cargo check >/dev/null 2>&1 && success "Dependencies resolved"
            fi
        fi
        
        # Check for outdated dependencies
        if command_exists cargo-outdated; then
            local outdated=$(cargo outdated --format json 2>/dev/null | jq '.dependencies | length' 2>/dev/null || echo "0")
            if [[ "$outdated" == "0" ]]; then
                success "All dependencies are up to date"
            else
                warning "$outdated dependencies are outdated"
            fi
        fi
    else
        error "Cargo.toml not found"
    fi
}

# Function to check build status
check_build() {
    check_start
    
    if [[ "$QUIET" == false ]]; then
        log "Checking build status..."
    fi
    
    cd "$PROJECT_ROOT"
    
    # Check if project compiles
    if cargo check --quiet >/dev/null 2>&1; then
        success "Project compiles without errors"
    else
        error "Project has compilation errors"
        if [[ "$VERBOSE" == true ]]; then
            info "Running cargo check for details..."
            cargo check 2>&1 | head -20
        fi
    fi
    
    # Check if tests compile
    if cargo test --no-run --quiet >/dev/null 2>&1; then
        success "Tests compile without errors"
    else
        warning "Tests have compilation issues"
    fi
}

# Function to run tests
check_tests() {
    check_start
    
    if [[ "$QUIET" == false ]]; then
        log "Running tests..."
    fi
    
    cd "$PROJECT_ROOT"
    
    # Run tests with timeout
    if timeout 300 cargo test --quiet >/dev/null 2>&1; then
        success "All tests pass"
    else
        local exit_code=$?
        if [[ $exit_code -eq 124 ]]; then
            warning "Tests timed out after 5 minutes"
        else
            error "Some tests failed"
            if [[ "$VERBOSE" == true ]]; then
                info "Running tests with output..."
                cargo test 2>&1 | tail -20
            fi
        fi
    fi
}

# Function to check database connections
check_databases() {
    check_start
    
    if [[ "$QUIET" == false ]]; then
        log "Checking database connections..."
    fi
    
    local databases_ok=true
    
    # Check data directory
    if [[ -d "${PROJECT_ROOT}/data" ]]; then
        success "Data directory exists"
        
        # Check for database files
        local db_files=(
            "${PROJECT_ROOT}/data/memory.db"
            "${PROJECT_ROOT}/data/meta_memory.db"
            "${PROJECT_ROOT}/data/curiosity.db"
        )
        
        for db_file in "${db_files[@]}"; do
            if [[ -f "$db_file" ]]; then
                local size=$(get_file_size "$db_file")
                if [[ "$VERBOSE" == true ]]; then
                    info "Database file: $(basename "$db_file") (${size} bytes)"
                fi
            fi
        done
    else
        warning "Data directory does not exist"
        if [[ "$FIX" == true ]]; then
            mkdir -p "${PROJECT_ROOT}/data"
            success "Created data directory"
        fi
    fi
    
    # Check Neo4j connection if configured
    if command_exists neo4j; then
        if port_open "localhost" "7687" 2; then
            success "Neo4j database is accessible"
        else
            warning "Neo4j database is not accessible on port 7687"
        fi
    fi
}

# Function to check API endpoints
check_api() {
    check_start
    
    if [[ "$QUIET" == false ]]; then
        log "Checking API endpoints..."
    fi
    
    local api_port="8080"
    local api_host="localhost"
    
    # Check if API port is available or in use
    if port_open "$api_host" "$api_port" 2; then
        success "API port $api_port is accessible"
        
        # Try to make a simple HTTP request
        if command_exists curl; then
            local response=$(curl -s -o /dev/null -w "%{http_code}" "http://$api_host:$api_port/health" 2>/dev/null || echo "000")
            if [[ "$response" == "200" ]]; then
                success "API health endpoint responds correctly"
            elif [[ "$response" != "000" ]]; then
                warning "API health endpoint returned HTTP $response"
            else
                warning "API health endpoint is not responding"
            fi
        fi
    else
        info "API is not currently running on port $api_port"
    fi
}

# Function to check log files
check_logs() {
    check_start
    
    if [[ "$QUIET" == false ]]; then
        log "Checking log files..."
    fi
    
    local log_dir="${PROJECT_ROOT}/logs"
    
    if [[ -d "$log_dir" ]]; then
        local log_count=$(find "$log_dir" -name "*.log" -type f | wc -l)
        local log_size=$(du -sh "$log_dir" 2>/dev/null | cut -f1 || echo "0B")
        
        if [[ "$VERBOSE" == true ]]; then
            info "Log files: $log_count, Total size: $log_size"
        fi
        
        if [[ $log_count -gt 0 ]]; then
            success "Log files are present"
            
            # Check for recent error logs
            local recent_errors=$(find "$log_dir" -name "*.log" -type f -mtime -1 -exec grep -l "ERROR\|FATAL" {} \; 2>/dev/null | wc -l)
            if [[ $recent_errors -gt 0 ]]; then
                warning "$recent_errors log files contain recent errors"
            fi
        else
            info "No log files found (system may not have run yet)"
        fi
    else
        info "Log directory does not exist yet"
    fi
}

# Function to check performance
check_performance() {
    if [[ "$QUICK" == true ]]; then
        return 0
    fi
    
    check_start
    
    if [[ "$QUIET" == false ]]; then
        log "Checking system performance..."
    fi
    
    # CPU load check
    if command_exists uptime; then
        local load_avg=$(uptime | awk -F'load average:' '{print $2}' | awk '{print $1}' | sed 's/,//')
        if [[ "$VERBOSE" == true ]]; then
            info "System load average: $load_avg"
        fi
        
        # Simple load check (very basic)
        if (( $(echo "$load_avg < 2.0" | bc -l 2>/dev/null || echo "1") )); then
            success "System load is acceptable"
        else
            warning "High system load: $load_avg"
        fi
    fi
    
    # Quick compile time test
    if [[ "$FULL" == true ]]; then
        cd "$PROJECT_ROOT"
        local start_time=$(date +%s)
        if cargo check --quiet >/dev/null 2>&1; then
            local end_time=$(date +%s)
            local compile_time=$((end_time - start_time))
            if [[ "$VERBOSE" == true ]]; then
                info "Compile check time: ${compile_time}s"
            fi
            
            if [[ $compile_time -lt 30 ]]; then
                success "Compilation performance is good"
            elif [[ $compile_time -lt 60 ]]; then
                warning "Compilation is slower than expected"
            else
                warning "Compilation is very slow (${compile_time}s)"
            fi
        fi
    fi
}

# Function to check Docker setup
check_docker() {
    check_start
    
    if [[ "$QUIET" == false ]]; then
        log "Checking Docker setup..."
    fi
    
    if command_exists docker; then
        if docker info >/dev/null 2>&1; then
            success "Docker is running and accessible"
            
            if [[ -f "${PROJECT_ROOT}/Dockerfile" ]]; then
                success "Dockerfile exists"
            else
                warning "Dockerfile not found"
            fi
            
            if [[ -f "${PROJECT_ROOT}/docker-compose.yml" ]]; then
                success "Docker Compose configuration exists"
            else
                warning "Docker Compose configuration not found"
            fi
        else
            warning "Docker is installed but not running"
        fi
    else
        info "Docker is not installed"
    fi
}

# Function to generate JSON output
generate_json_output() {
    local status="healthy"
    if [[ $CHECKS_FAILED -gt 0 ]]; then
        status="unhealthy"
    elif [[ $CHECKS_WARNING -gt 0 ]]; then
        status="warning"
    fi
    
    cat << EOF
{
    "timestamp": "$(date -Iseconds)",
    "status": "$status",
    "summary": {
        "total_checks": $CHECKS_TOTAL,
        "passed": $CHECKS_PASSED,
        "failed": $CHECKS_FAILED,
        "warnings": $CHECKS_WARNING
    },
    "system_info": {
        "hostname": "$(hostname)",
        "os": "$(uname -s)",
        "arch": "$(uname -m)",
        "project_root": "$PROJECT_ROOT"
    }
}
EOF
}

# Function to display summary
show_summary() {
    echo
    echo "=================================="
    echo "    HEALTH CHECK SUMMARY"
    echo "=================================="
    echo "Total Checks:    $CHECKS_TOTAL"
    echo "Passed:          $CHECKS_PASSED"
    echo "Failed:          $CHECKS_FAILED"
    echo "Warnings:        $CHECKS_WARNING"
    echo
    
    if [[ $CHECKS_FAILED -eq 0 && $CHECKS_WARNING -eq 0 ]]; then
        success "System is healthy!"
    elif [[ $CHECKS_FAILED -eq 0 ]]; then
        warning "System is mostly healthy with some warnings"
    else
        error "System has issues that need attention"
    fi
    
    echo "=================================="
    echo
}

# Main health check function
main() {
    if [[ "$QUIET" == false ]]; then
        log "Starting Brain AI system health check..."
    fi
    
    # Run checks based on options
    if [[ "$API_ONLY" == true ]]; then
        check_api
    elif [[ "$DEPS_ONLY" == true ]]; then
        check_rust
        check_dependencies
    else
        # Standard health checks
        check_disk_space
        check_memory
        check_rust
        check_project_structure
        check_configuration
        check_dependencies
        check_build
        check_databases
        check_logs
        check_docker
        
        if [[ "$QUICK" == false ]]; then
            check_tests
            check_performance
        fi
        
        check_api
    fi
    
    # Output results
    if [[ "$JSON_OUTPUT" == true ]]; then
        generate_json_output
    else
        show_summary
    fi
    
    # Exit with appropriate code
    if [[ $CHECKS_FAILED -gt 0 ]]; then
        exit 1
    else
        exit 0
    fi
}

# Check for required commands
if ! command_exists bc && [[ "$FULL" == true ]]; then
    if [[ "$QUIET" == false ]]; then
        warning "bc calculator not found, some performance checks will be skipped"
    fi
fi

# Run main function
main 