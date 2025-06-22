#!/bin/bash

# Brain AI System Backup Script
# This script creates comprehensive backups of all system data and configurations

set -euo pipefail

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
BACKUP_DIR="${PROJECT_ROOT}/backups"
TIMESTAMP=$(date +"%Y%m%d_%H%M%S")
BACKUP_NAME="brain_ai_backup_${TIMESTAMP}"
BACKUP_PATH="${BACKUP_DIR}/${BACKUP_NAME}"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
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

# Help function
show_help() {
    cat << EOF
Brain AI System Backup Script

Usage: $0 [OPTIONS]

Options:
    -h, --help          Show this help message
    -f, --full          Create full backup (default)
    -d, --data-only     Backup only data files
    -c, --config-only   Backup only configuration files
    -n, --name NAME     Custom backup name (without timestamp)
    -o, --output DIR    Custom backup directory
    --compress          Compress backup (default: true)
    --no-compress       Don't compress backup
    --retention DAYS    Set retention period in days (default: 30)
    --verify            Verify backup integrity after creation

Examples:
    $0                          # Full backup with default settings
    $0 --data-only              # Backup only data files
    $0 --name "pre-deployment"  # Custom backup name
    $0 --retention 7            # Keep backups for 7 days
    $0 --verify                 # Create backup and verify integrity
EOF
}

# Default options
BACKUP_TYPE="full"
COMPRESS=true
RETENTION_DAYS=30
VERIFY=false
CUSTOM_NAME=""
CUSTOM_OUTPUT=""

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -h|--help)
            show_help
            exit 0
            ;;
        -f|--full)
            BACKUP_TYPE="full"
            shift
            ;;
        -d|--data-only)
            BACKUP_TYPE="data"
            shift
            ;;
        -c|--config-only)
            BACKUP_TYPE="config"
            shift
            ;;
        -n|--name)
            CUSTOM_NAME="$2"
            shift 2
            ;;
        -o|--output)
            CUSTOM_OUTPUT="$2"
            shift 2
            ;;
        --compress)
            COMPRESS=true
            shift
            ;;
        --no-compress)
            COMPRESS=false
            shift
            ;;
        --retention)
            RETENTION_DAYS="$2"
            shift 2
            ;;
        --verify)
            VERIFY=true
            shift
            ;;
        *)
            error "Unknown option: $1"
            show_help
            exit 1
            ;;
    esac
done

# Update backup path with custom settings
if [[ -n "$CUSTOM_OUTPUT" ]]; then
    BACKUP_DIR="$CUSTOM_OUTPUT"
fi

if [[ -n "$CUSTOM_NAME" ]]; then
    BACKUP_NAME="${CUSTOM_NAME}_${TIMESTAMP}"
else
    BACKUP_NAME="brain_ai_${BACKUP_TYPE}_backup_${TIMESTAMP}"
fi

BACKUP_PATH="${BACKUP_DIR}/${BACKUP_NAME}"

# Create backup directory
mkdir -p "$BACKUP_DIR"

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function to get directory size
get_size() {
    if [[ -d "$1" ]]; then
        du -sh "$1" 2>/dev/null | cut -f1 || echo "0B"
    else
        echo "0B"
    fi
}

# Function to backup data files
backup_data() {
    log "Backing up data files..."
    
    # Create data backup directory
    mkdir -p "${BACKUP_PATH}/data"
    
    # Backup database files
    if [[ -d "${PROJECT_ROOT}/data" ]]; then
        log "  - Database files"
        cp -r "${PROJECT_ROOT}/data"/* "${BACKUP_PATH}/data/" 2>/dev/null || true
    fi
    
    # Backup logs
    if [[ -d "${PROJECT_ROOT}/logs" ]]; then
        log "  - Log files"
        mkdir -p "${BACKUP_PATH}/logs"
        cp -r "${PROJECT_ROOT}/logs"/* "${BACKUP_PATH}/logs/" 2>/dev/null || true
    fi
    
    # Backup any temporary data
    if [[ -d "${PROJECT_ROOT}/tmp" ]]; then
        log "  - Temporary data"
        mkdir -p "${BACKUP_PATH}/tmp"
        cp -r "${PROJECT_ROOT}/tmp"/* "${BACKUP_PATH}/tmp/" 2>/dev/null || true
    fi
    
    # Backup model files if they exist
    if [[ -d "${PROJECT_ROOT}/models" ]]; then
        log "  - Model files"
        mkdir -p "${BACKUP_PATH}/models"
        cp -r "${PROJECT_ROOT}/models"/* "${BACKUP_PATH}/models/" 2>/dev/null || true
    fi
    
    success "Data backup completed"
}

# Function to backup configuration files
backup_config() {
    log "Backing up configuration files..."
    
    # Create config backup directory
    mkdir -p "${BACKUP_PATH}/config"
    
    # Backup main configuration
    if [[ -f "${PROJECT_ROOT}/config.toml" ]]; then
        log "  - Main configuration"
        cp "${PROJECT_ROOT}/config.toml" "${BACKUP_PATH}/config/"
    fi
    
    if [[ -f "${SCRIPT_DIR}/config.toml" ]]; then
        log "  - Scripts configuration"
        cp "${SCRIPT_DIR}/config.toml" "${BACKUP_PATH}/config/"
    fi
    
    # Backup environment files
    if [[ -f "${PROJECT_ROOT}/.env" ]]; then
        log "  - Environment configuration"
        cp "${PROJECT_ROOT}/.env" "${BACKUP_PATH}/config/"
    fi
    
    if [[ -f "${PROJECT_ROOT}/.env.example" ]]; then
        cp "${PROJECT_ROOT}/.env.example" "${BACKUP_PATH}/config/"
    fi
    
    # Backup Docker configuration
    if [[ -f "${PROJECT_ROOT}/Dockerfile" ]]; then
        log "  - Docker configuration"
        cp "${PROJECT_ROOT}/Dockerfile" "${BACKUP_PATH}/config/"
    fi
    
    if [[ -f "${PROJECT_ROOT}/docker-compose.yml" ]]; then
        cp "${PROJECT_ROOT}/docker-compose.yml" "${BACKUP_PATH}/config/"
    fi
    
    # Backup Cargo configuration
    if [[ -f "${PROJECT_ROOT}/Cargo.toml" ]]; then
        log "  - Cargo configuration"
        cp "${PROJECT_ROOT}/Cargo.toml" "${BACKUP_PATH}/config/"
    fi
    
    if [[ -f "${PROJECT_ROOT}/Cargo.lock" ]]; then
        cp "${PROJECT_ROOT}/Cargo.lock" "${BACKUP_PATH}/config/"
    fi
    
    # Backup scripts
    if [[ -d "${SCRIPT_DIR}" ]]; then
        log "  - Scripts and tools"
        mkdir -p "${BACKUP_PATH}/scripts"
        cp -r "${SCRIPT_DIR}"/* "${BACKUP_PATH}/scripts/" 2>/dev/null || true
    fi
    
    success "Configuration backup completed"
}

# Function to backup source code (for full backup)
backup_source() {
    log "Backing up source code..."
    
    # Create source backup directory
    mkdir -p "${BACKUP_PATH}/src"
    
    # Backup Rust source code
    if [[ -d "${PROJECT_ROOT}/src" ]]; then
        log "  - Rust source code"
        cp -r "${PROJECT_ROOT}/src"/* "${BACKUP_PATH}/src/" 2>/dev/null || true
    fi
    
    # Backup tests
    if [[ -d "${PROJECT_ROOT}/tests" ]]; then
        log "  - Test files"
        mkdir -p "${BACKUP_PATH}/tests"
        cp -r "${PROJECT_ROOT}/tests"/* "${BACKUP_PATH}/tests/" 2>/dev/null || true
    fi
    
    # Backup examples
    if [[ -d "${PROJECT_ROOT}/examples" ]]; then
        log "  - Example files"
        mkdir -p "${BACKUP_PATH}/examples"
        cp -r "${PROJECT_ROOT}/examples"/* "${BACKUP_PATH}/examples/" 2>/dev/null || true
    fi
    
    # Backup documentation
    if [[ -d "${PROJECT_ROOT}/docs" ]]; then
        log "  - Documentation"
        mkdir -p "${BACKUP_PATH}/docs"
        cp -r "${PROJECT_ROOT}/docs"/* "${BACKUP_PATH}/docs/" 2>/dev/null || true
    fi
    
    # Backup README and other root files
    for file in README.md CHANGELOG.md STATUS.md LICENSE; do
        if [[ -f "${PROJECT_ROOT}/${file}" ]]; then
            log "  - ${file}"
            cp "${PROJECT_ROOT}/${file}" "${BACKUP_PATH}/"
        fi
    done
    
    success "Source code backup completed"
}

# Function to create backup metadata
create_metadata() {
    log "Creating backup metadata..."
    
    cat > "${BACKUP_PATH}/backup_info.json" << EOF
{
    "backup_name": "${BACKUP_NAME}",
    "backup_type": "${BACKUP_TYPE}",
    "timestamp": "${TIMESTAMP}",
    "date": "$(date -Iseconds)",
    "hostname": "$(hostname)",
    "user": "$(whoami)",
    "project_root": "${PROJECT_ROOT}",
    "backup_path": "${BACKUP_PATH}",
    "compressed": ${COMPRESS},
    "retention_days": ${RETENTION_DAYS},
    "git_commit": "$(cd "${PROJECT_ROOT}" && git rev-parse HEAD 2>/dev/null || echo 'unknown')",
    "git_branch": "$(cd "${PROJECT_ROOT}" && git branch --show-current 2>/dev/null || echo 'unknown')",
    "system_info": {
        "os": "$(uname -s)",
        "arch": "$(uname -m)",
        "kernel": "$(uname -r)"
    }
}
EOF
    
    # Create checksums
    log "Creating checksums..."
    if command_exists sha256sum; then
        find "${BACKUP_PATH}" -type f -not -name "checksums.sha256" -exec sha256sum {} \; > "${BACKUP_PATH}/checksums.sha256"
    elif command_exists shasum; then
        find "${BACKUP_PATH}" -type f -not -name "checksums.sha256" -exec shasum -a 256 {} \; > "${BACKUP_PATH}/checksums.sha256"
    else
        warning "No checksum utility found, skipping checksum creation"
    fi
    
    success "Metadata created"
}

# Function to compress backup
compress_backup() {
    if [[ "$COMPRESS" == true ]]; then
        log "Compressing backup..."
        
        cd "$BACKUP_DIR"
        
        if command_exists tar; then
            tar -czf "${BACKUP_NAME}.tar.gz" "$BACKUP_NAME"
            rm -rf "$BACKUP_NAME"
            BACKUP_PATH="${BACKUP_DIR}/${BACKUP_NAME}.tar.gz"
            success "Backup compressed to ${BACKUP_NAME}.tar.gz"
        else
            warning "tar not found, backup will remain uncompressed"
        fi
    fi
}

# Function to verify backup
verify_backup() {
    if [[ "$VERIFY" == true ]]; then
        log "Verifying backup integrity..."
        
        if [[ "$COMPRESS" == true ]]; then
            # Verify compressed backup
            if command_exists tar; then
                if tar -tzf "$BACKUP_PATH" >/dev/null 2>&1; then
                    success "Compressed backup integrity verified"
                else
                    error "Backup verification failed!"
                    exit 1
                fi
            fi
        else
            # Verify uncompressed backup
            if [[ -f "${BACKUP_PATH}/checksums.sha256" ]]; then
                cd "$BACKUP_PATH"
                if command_exists sha256sum; then
                    if sha256sum -c checksums.sha256 >/dev/null 2>&1; then
                        success "Backup checksums verified"
                    else
                        error "Checksum verification failed!"
                        exit 1
                    fi
                elif command_exists shasum; then
                    if shasum -a 256 -c checksums.sha256 >/dev/null 2>&1; then
                        success "Backup checksums verified"
                    else
                        error "Checksum verification failed!"
                        exit 1
                    fi
                fi
            fi
        fi
    fi
}

# Function to clean old backups
cleanup_old_backups() {
    log "Cleaning up old backups (older than ${RETENTION_DAYS} days)..."
    
    if command_exists find; then
        local deleted_count=0
        while IFS= read -r -d '' file; do
            log "  - Removing old backup: $(basename "$file")"
            rm -f "$file"
            ((deleted_count++))
        done < <(find "$BACKUP_DIR" -name "brain_ai_*backup_*.tar.gz" -o -name "brain_ai_*backup_*" -type f -mtime +"$RETENTION_DAYS" -print0 2>/dev/null)
        
        if [[ $deleted_count -gt 0 ]]; then
            success "Removed $deleted_count old backup(s)"
        else
            log "No old backups to remove"
        fi
    fi
}

# Function to display backup summary
show_summary() {
    local backup_size
    if [[ -f "$BACKUP_PATH" ]]; then
        backup_size=$(get_size "$BACKUP_PATH")
    elif [[ -d "$BACKUP_PATH" ]]; then
        backup_size=$(get_size "$BACKUP_PATH")
    else
        backup_size="unknown"
    fi
    
    echo
    echo "=================================="
    echo "    BACKUP SUMMARY"
    echo "=================================="
    echo "Backup Type:     $BACKUP_TYPE"
    echo "Backup Name:     $BACKUP_NAME"
    echo "Backup Path:     $BACKUP_PATH"
    echo "Backup Size:     $backup_size"
    echo "Compressed:      $COMPRESS"
    echo "Verified:        $VERIFY"
    echo "Retention:       $RETENTION_DAYS days"
    echo "Timestamp:       $(date)"
    echo "=================================="
    echo
}

# Main backup function
main() {
    log "Starting Brain AI system backup..."
    log "Backup type: $BACKUP_TYPE"
    log "Backup path: $BACKUP_PATH"
    
    # Create backup directory
    mkdir -p "$BACKUP_PATH"
    
    # Perform backup based on type
    case "$BACKUP_TYPE" in
        "full")
            backup_data
            backup_config
            backup_source
            ;;
        "data")
            backup_data
            ;;
        "config")
            backup_config
            ;;
        *)
            error "Unknown backup type: $BACKUP_TYPE"
            exit 1
            ;;
    esac
    
    # Create metadata
    create_metadata
    
    # Compress if requested
    compress_backup
    
    # Verify if requested
    verify_backup
    
    # Clean up old backups
    cleanup_old_backups
    
    # Show summary
    show_summary
    
    success "Backup completed successfully!"
}

# Check dependencies
check_dependencies() {
    local missing_deps=()
    
    if ! command_exists cp; then
        missing_deps+=("cp")
    fi
    
    if ! command_exists mkdir; then
        missing_deps+=("mkdir")
    fi
    
    if ! command_exists find; then
        missing_deps+=("find")
    fi
    
    if [[ ${#missing_deps[@]} -gt 0 ]]; then
        error "Missing required dependencies: ${missing_deps[*]}"
        exit 1
    fi
}

# Trap to cleanup on exit
cleanup_on_exit() {
    if [[ -d "$BACKUP_PATH" && ! -f "${BACKUP_PATH}/backup_info.json" ]]; then
        warning "Cleaning up incomplete backup..."
        rm -rf "$BACKUP_PATH"
    fi
}

trap cleanup_on_exit EXIT

# Run main function
check_dependencies
main 