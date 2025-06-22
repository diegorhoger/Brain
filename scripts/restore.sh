#!/bin/bash

# Brain AI System Restore Script
# This script restores Brain AI system from backups

set -euo pipefail

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
BACKUP_DIR="${PROJECT_ROOT}/backups"

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
Brain AI System Restore Script

Usage: $0 [OPTIONS] BACKUP_PATH

Arguments:
    BACKUP_PATH         Path to backup file or directory to restore

Options:
    -h, --help          Show this help message
    -f, --full          Restore full backup (default)
    -d, --data-only     Restore only data files
    -c, --config-only   Restore only configuration files
    --no-backup         Don't create backup before restore
    --force             Force restore without confirmation
    --verify            Verify backup integrity before restore
    --dry-run           Show what would be restored without doing it
    --list              List available backups and exit

Examples:
    $0 --list                                    # List available backups
    $0 /path/to/backup.tar.gz                   # Restore from compressed backup
    $0 /path/to/backup_directory                # Restore from directory
    $0 --data-only backup.tar.gz                # Restore only data files
    $0 --verify --force backup.tar.gz           # Verify and force restore
    $0 --dry-run backup.tar.gz                  # Preview restore operation
EOF
}

# Default options
RESTORE_TYPE="full"
CREATE_BACKUP=true
FORCE=false
VERIFY=false
DRY_RUN=false
LIST_BACKUPS=false
BACKUP_PATH=""

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -h|--help)
            show_help
            exit 0
            ;;
        -f|--full)
            RESTORE_TYPE="full"
            shift
            ;;
        -d|--data-only)
            RESTORE_TYPE="data"
            shift
            ;;
        -c|--config-only)
            RESTORE_TYPE="config"
            shift
            ;;
        --no-backup)
            CREATE_BACKUP=false
            shift
            ;;
        --force)
            FORCE=true
            shift
            ;;
        --verify)
            VERIFY=true
            shift
            ;;
        --dry-run)
            DRY_RUN=true
            shift
            ;;
        --list)
            LIST_BACKUPS=true
            shift
            ;;
        -*)
            error "Unknown option: $1"
            show_help
            exit 1
            ;;
        *)
            if [[ -z "$BACKUP_PATH" ]]; then
                BACKUP_PATH="$1"
            else
                error "Multiple backup paths specified"
                exit 1
            fi
            shift
            ;;
    esac
done

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function to list available backups
list_backups() {
    log "Available backups in $BACKUP_DIR:"
    echo
    
    if [[ ! -d "$BACKUP_DIR" ]]; then
        warning "Backup directory does not exist: $BACKUP_DIR"
        return 1
    fi
    
    local found_backups=false
    
    # List compressed backups
    if ls "$BACKUP_DIR"/*.tar.gz >/dev/null 2>&1; then
        echo "Compressed backups:"
        for backup in "$BACKUP_DIR"/*.tar.gz; do
            if [[ -f "$backup" ]]; then
                local size=$(du -sh "$backup" 2>/dev/null | cut -f1 || echo "unknown")
                local date=$(date -r "$backup" '+%Y-%m-%d %H:%M:%S' 2>/dev/null || echo "unknown")
                printf "  %-40s %10s  %s\n" "$(basename "$backup")" "$size" "$date"
                found_backups=true
            fi
        done
        echo
    fi
    
    # List directory backups
    if ls -d "$BACKUP_DIR"/brain_ai_*backup_* >/dev/null 2>&1; then
        echo "Directory backups:"
        for backup in "$BACKUP_DIR"/brain_ai_*backup_*; do
            if [[ -d "$backup" ]]; then
                local size=$(du -sh "$backup" 2>/dev/null | cut -f1 || echo "unknown")
                local date=$(date -r "$backup" '+%Y-%m-%d %H:%M:%S' 2>/dev/null || echo "unknown")
                printf "  %-40s %10s  %s\n" "$(basename "$backup")" "$size" "$date"
                found_backups=true
            fi
        done
        echo
    fi
    
    if [[ "$found_backups" == false ]]; then
        warning "No backups found in $BACKUP_DIR"
        return 1
    fi
    
    return 0
}

# Function to verify backup integrity
verify_backup() {
    local backup_path="$1"
    
    log "Verifying backup integrity..."
    
    if [[ -f "$backup_path" ]]; then
        # Compressed backup
        if [[ "$backup_path" == *.tar.gz ]]; then
            if command_exists tar; then
                if tar -tzf "$backup_path" >/dev/null 2>&1; then
                    success "Compressed backup integrity verified"
                    return 0
                else
                    error "Compressed backup is corrupted!"
                    return 1
                fi
            else
                warning "tar not available, skipping compressed backup verification"
                return 0
            fi
        else
            warning "Unknown backup file format, skipping verification"
            return 0
        fi
    elif [[ -d "$backup_path" ]]; then
        # Directory backup
        if [[ -f "$backup_path/checksums.sha256" ]]; then
            cd "$backup_path"
            if command_exists sha256sum; then
                if sha256sum -c checksums.sha256 >/dev/null 2>&1; then
                    success "Backup checksums verified"
                    return 0
                else
                    error "Checksum verification failed!"
                    return 1
                fi
            elif command_exists shasum; then
                if shasum -a 256 -c checksums.sha256 >/dev/null 2>&1; then
                    success "Backup checksums verified"
                    return 0
                else
                    error "Checksum verification failed!"
                    return 1
                fi
            else
                warning "No checksum utility available, skipping verification"
                return 0
            fi
        else
            warning "No checksums found, skipping verification"
            return 0
        fi
    else
        error "Backup path does not exist: $backup_path"
        return 1
    fi
}

# Function to extract backup if compressed
extract_backup() {
    local backup_path="$1"
    local extract_dir="$2"
    
    if [[ -f "$backup_path" && "$backup_path" == *.tar.gz ]]; then
        log "Extracting compressed backup..."
        
        if command_exists tar; then
            mkdir -p "$extract_dir"
            tar -xzf "$backup_path" -C "$extract_dir" --strip-components=1
            success "Backup extracted to $extract_dir"
            return 0
        else
            error "tar not available, cannot extract compressed backup"
            return 1
        fi
    elif [[ -d "$backup_path" ]]; then
        # Already a directory, just use it
        return 0
    else
        error "Invalid backup format: $backup_path"
        return 1
    fi
}

# Function to create backup before restore
create_pre_restore_backup() {
    if [[ "$CREATE_BACKUP" == true ]]; then
        log "Creating backup before restore..."
        
        local backup_script="${SCRIPT_DIR}/backup.sh"
        if [[ -x "$backup_script" ]]; then
            local timestamp=$(date +"%Y%m%d_%H%M%S")
            "$backup_script" --name "pre-restore-${timestamp}" --no-compress
            success "Pre-restore backup created"
        else
            warning "Backup script not found or not executable, skipping pre-restore backup"
        fi
    fi
}

# Function to restore data files
restore_data() {
    local backup_source="$1"
    
    log "Restoring data files..."
    
    if [[ -d "$backup_source/data" ]]; then
        log "  - Database files"
        mkdir -p "${PROJECT_ROOT}/data"
        if [[ "$DRY_RUN" == true ]]; then
            log "    [DRY RUN] Would copy: $backup_source/data/* -> ${PROJECT_ROOT}/data/"
        else
            cp -r "$backup_source/data"/* "${PROJECT_ROOT}/data/" 2>/dev/null || true
        fi
    fi
    
    if [[ -d "$backup_source/logs" ]]; then
        log "  - Log files"
        mkdir -p "${PROJECT_ROOT}/logs"
        if [[ "$DRY_RUN" == true ]]; then
            log "    [DRY RUN] Would copy: $backup_source/logs/* -> ${PROJECT_ROOT}/logs/"
        else
            cp -r "$backup_source/logs"/* "${PROJECT_ROOT}/logs/" 2>/dev/null || true
        fi
    fi
    
    if [[ -d "$backup_source/models" ]]; then
        log "  - Model files"
        mkdir -p "${PROJECT_ROOT}/models"
        if [[ "$DRY_RUN" == true ]]; then
            log "    [DRY RUN] Would copy: $backup_source/models/* -> ${PROJECT_ROOT}/models/"
        else
            cp -r "$backup_source/models"/* "${PROJECT_ROOT}/models/" 2>/dev/null || true
        fi
    fi
    
    success "Data restore completed"
}

# Function to restore configuration files
restore_config() {
    local backup_source="$1"
    
    log "Restoring configuration files..."
    
    if [[ -d "$backup_source/config" ]]; then
        if [[ -f "$backup_source/config/config.toml" ]]; then
            log "  - Main configuration"
            if [[ "$DRY_RUN" == true ]]; then
                log "    [DRY RUN] Would copy: $backup_source/config/config.toml -> ${PROJECT_ROOT}/config.toml"
            else
                cp "$backup_source/config/config.toml" "${PROJECT_ROOT}/"
            fi
        fi
        
        if [[ -f "$backup_source/config/.env" ]]; then
            log "  - Environment configuration"
            if [[ "$DRY_RUN" == true ]]; then
                log "    [DRY RUN] Would copy: $backup_source/config/.env -> ${PROJECT_ROOT}/.env"
            else
                cp "$backup_source/config/.env" "${PROJECT_ROOT}/"
            fi
        fi
        
        if [[ -f "$backup_source/config/Dockerfile" ]]; then
            log "  - Docker configuration"
            if [[ "$DRY_RUN" == true ]]; then
                log "    [DRY RUN] Would copy: $backup_source/config/Dockerfile -> ${PROJECT_ROOT}/Dockerfile"
            else
                cp "$backup_source/config/Dockerfile" "${PROJECT_ROOT}/"
            fi
        fi
        
        if [[ -f "$backup_source/config/docker-compose.yml" ]]; then
            if [[ "$DRY_RUN" == true ]]; then
                log "    [DRY RUN] Would copy: $backup_source/config/docker-compose.yml -> ${PROJECT_ROOT}/docker-compose.yml"
            else
                cp "$backup_source/config/docker-compose.yml" "${PROJECT_ROOT}/"
            fi
        fi
        
        if [[ -f "$backup_source/config/Cargo.toml" ]]; then
            log "  - Cargo configuration"
            if [[ "$DRY_RUN" == true ]]; then
                log "    [DRY RUN] Would copy: $backup_source/config/Cargo.toml -> ${PROJECT_ROOT}/Cargo.toml"
            else
                cp "$backup_source/config/Cargo.toml" "${PROJECT_ROOT}/"
            fi
        fi
        
        if [[ -d "$backup_source/scripts" ]]; then
            log "  - Scripts and tools"
            if [[ "$DRY_RUN" == true ]]; then
                log "    [DRY RUN] Would copy: $backup_source/scripts/* -> ${SCRIPT_DIR}/"
            else
                cp -r "$backup_source/scripts"/* "${SCRIPT_DIR}/" 2>/dev/null || true
            fi
        fi
    fi
    
    success "Configuration restore completed"
}

# Function to restore source code
restore_source() {
    local backup_source="$1"
    
    log "Restoring source code..."
    
    if [[ -d "$backup_source/src" ]]; then
        log "  - Rust source code"
        mkdir -p "${PROJECT_ROOT}/src"
        if [[ "$DRY_RUN" == true ]]; then
            log "    [DRY RUN] Would copy: $backup_source/src/* -> ${PROJECT_ROOT}/src/"
        else
            cp -r "$backup_source/src"/* "${PROJECT_ROOT}/src/" 2>/dev/null || true
        fi
    fi
    
    if [[ -d "$backup_source/tests" ]]; then
        log "  - Test files"
        mkdir -p "${PROJECT_ROOT}/tests"
        if [[ "$DRY_RUN" == true ]]; then
            log "    [DRY RUN] Would copy: $backup_source/tests/* -> ${PROJECT_ROOT}/tests/"
        else
            cp -r "$backup_source/tests"/* "${PROJECT_ROOT}/tests/" 2>/dev/null || true
        fi
    fi
    
    if [[ -d "$backup_source/examples" ]]; then
        log "  - Example files"
        mkdir -p "${PROJECT_ROOT}/examples"
        if [[ "$DRY_RUN" == true ]]; then
            log "    [DRY RUN] Would copy: $backup_source/examples/* -> ${PROJECT_ROOT}/examples/"
        else
            cp -r "$backup_source/examples"/* "${PROJECT_ROOT}/examples/" 2>/dev/null || true
        fi
    fi
    
    if [[ -d "$backup_source/docs" ]]; then
        log "  - Documentation"
        mkdir -p "${PROJECT_ROOT}/docs"
        if [[ "$DRY_RUN" == true ]]; then
            log "    [DRY RUN] Would copy: $backup_source/docs/* -> ${PROJECT_ROOT}/docs/"
        else
            cp -r "$backup_source/docs"/* "${PROJECT_ROOT}/docs/" 2>/dev/null || true
        fi
    fi
    
    # Restore root files
    for file in README.md CHANGELOG.md STATUS.md LICENSE; do
        if [[ -f "$backup_source/$file" ]]; then
            log "  - $file"
            if [[ "$DRY_RUN" == true ]]; then
                log "    [DRY RUN] Would copy: $backup_source/$file -> ${PROJECT_ROOT}/$file"
            else
                cp "$backup_source/$file" "${PROJECT_ROOT}/"
            fi
        fi
    done
    
    success "Source code restore completed"
}

# Function to show backup information
show_backup_info() {
    local backup_source="$1"
    
    if [[ -f "$backup_source/backup_info.json" ]]; then
        log "Backup information:"
        
        if command_exists jq; then
            jq . "$backup_source/backup_info.json"
        else
            cat "$backup_source/backup_info.json"
        fi
        echo
    else
        warning "No backup metadata found"
    fi
}

# Function to confirm restore operation
confirm_restore() {
    if [[ "$FORCE" == true ]]; then
        return 0
    fi
    
    echo
    warning "This will overwrite existing files in the project directory!"
    echo "Project root: $PROJECT_ROOT"
    echo "Restore type: $RESTORE_TYPE"
    echo "Backup path: $BACKUP_PATH"
    echo
    
    read -p "Are you sure you want to continue? (y/N): " -n 1 -r
    echo
    
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        log "Restore cancelled by user"
        exit 0
    fi
}

# Function to display restore summary
show_summary() {
    echo
    echo "=================================="
    echo "    RESTORE SUMMARY"
    echo "=================================="
    echo "Restore Type:    $RESTORE_TYPE"
    echo "Backup Path:     $BACKUP_PATH"
    echo "Project Root:    $PROJECT_ROOT"
    echo "Dry Run:         $DRY_RUN"
    echo "Verified:        $VERIFY"
    echo "Timestamp:       $(date)"
    echo "=================================="
    echo
}

# Main restore function
main() {
    local temp_extract_dir=""
    local backup_source="$BACKUP_PATH"
    
    # Handle list option
    if [[ "$LIST_BACKUPS" == true ]]; then
        list_backups
        exit $?
    fi
    
    # Validate backup path
    if [[ -z "$BACKUP_PATH" ]]; then
        error "No backup path specified"
        show_help
        exit 1
    fi
    
    if [[ ! -e "$BACKUP_PATH" ]]; then
        error "Backup path does not exist: $BACKUP_PATH"
        exit 1
    fi
    
    log "Starting Brain AI system restore..."
    log "Restore type: $RESTORE_TYPE"
    log "Backup path: $BACKUP_PATH"
    
    # Verify backup if requested
    if [[ "$VERIFY" == true ]]; then
        if ! verify_backup "$BACKUP_PATH"; then
            error "Backup verification failed"
            exit 1
        fi
    fi
    
    # Extract backup if compressed
    if [[ -f "$BACKUP_PATH" && "$BACKUP_PATH" == *.tar.gz ]]; then
        temp_extract_dir=$(mktemp -d)
        if ! extract_backup "$BACKUP_PATH" "$temp_extract_dir"; then
            error "Failed to extract backup"
            exit 1
        fi
        backup_source="$temp_extract_dir"
    fi
    
    # Show backup information
    show_backup_info "$backup_source"
    
    # Confirm restore operation
    confirm_restore
    
    # Create pre-restore backup
    if [[ "$DRY_RUN" == false ]]; then
        create_pre_restore_backup
    fi
    
    # Perform restore based on type
    case "$RESTORE_TYPE" in
        "full")
            restore_data "$backup_source"
            restore_config "$backup_source"
            restore_source "$backup_source"
            ;;
        "data")
            restore_data "$backup_source"
            ;;
        "config")
            restore_config "$backup_source"
            ;;
        *)
            error "Unknown restore type: $RESTORE_TYPE"
            exit 1
            ;;
    esac
    
    # Cleanup temporary extraction directory
    if [[ -n "$temp_extract_dir" && -d "$temp_extract_dir" ]]; then
        rm -rf "$temp_extract_dir"
    fi
    
    # Show summary
    show_summary
    
    if [[ "$DRY_RUN" == true ]]; then
        success "Dry run completed successfully!"
        log "No files were actually restored"
    else
        success "Restore completed successfully!"
        log "You may need to rebuild the project: cargo build"
    fi
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
    
    if [[ ${#missing_deps[@]} -gt 0 ]]; then
        error "Missing required dependencies: ${missing_deps[*]}"
        exit 1
    fi
}

# Trap to cleanup on exit
cleanup_on_exit() {
    if [[ -n "${temp_extract_dir:-}" && -d "$temp_extract_dir" ]]; then
        rm -rf "$temp_extract_dir"
    fi
}

trap cleanup_on_exit EXIT

# Run main function
check_dependencies
main 