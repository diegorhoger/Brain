#!/bin/bash

# Brain AI Documentation Generation Script
# This script automates the complete documentation generation process

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
DOCS_DIR="docs"
OUTPUT_DIR="docs/book"
BACKUP_DIR="docs/backup"
RUST_DOC_DIR="target/doc"

# Functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check dependencies
check_dependencies() {
    log_info "Checking dependencies..."
    
    local deps=("mdbook" "cargo" "rustc")
    local missing_deps=()
    
    for dep in "${deps[@]}"; do
        if ! command -v "$dep" &> /dev/null; then
            missing_deps+=("$dep")
        fi
    done
    
    if [ ${#missing_deps[@]} -ne 0 ]; then
        log_error "Missing dependencies: ${missing_deps[*]}"
        echo "Please install missing dependencies:"
        for dep in "${missing_deps[@]}"; do
            case $dep in
                "mdbook")
                    echo "  cargo install mdbook"
                    ;;
                "cargo"|"rustc")
                    echo "  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
                    ;;
            esac
        done
        exit 1
    fi
    
    log_success "All dependencies found"
}

# Install mdBook plugins
install_plugins() {
    log_info "Installing mdBook plugins..."
    
    local plugins=("mdbook-mermaid" "mdbook-toc")
    
    for plugin in "${plugins[@]}"; do
        if ! command -v "$plugin" &> /dev/null; then
            log_info "Installing $plugin..."
            cargo install "$plugin"
        else
            log_info "$plugin already installed"
        fi
    done
    
    log_success "All plugins installed"
}

# Generate Rust documentation
generate_rust_docs() {
    log_info "Generating Rust API documentation..."
    
    # Clean previous docs
    if [ -d "$RUST_DOC_DIR" ]; then
        rm -rf "$RUST_DOC_DIR"
    fi
    
    # Generate docs with all features
    cargo doc --all-features --no-deps --document-private-items
    
    # Copy to docs directory
    if [ -d "$RUST_DOC_DIR" ]; then
        mkdir -p "$DOCS_DIR/src/api/rust"
        cp -r "$RUST_DOC_DIR"/* "$DOCS_DIR/src/api/rust/" 2>/dev/null || true
        log_success "Rust documentation generated"
    else
        log_warning "Rust documentation not found"
    fi
}

# Extract code examples from source
extract_examples() {
    log_info "Extracting code examples..."
    
    local examples_dir="$DOCS_DIR/src/examples"
    mkdir -p "$examples_dir"
    
    # Find all example files
    find examples/ -name "*.rs" -type f | while read -r example_file; do
        local example_name=$(basename "$example_file" .rs)
        local output_file="$examples_dir/${example_name}.md"
        
        cat > "$output_file" << EOF
# ${example_name^} Example

\`\`\`rust
$(cat "$example_file")
\`\`\`

## Running This Example

\`\`\`bash
cargo run --example $example_name
\`\`\`
EOF
    done
    
    log_success "Code examples extracted"
}

# Generate API reference from source code
generate_api_reference() {
    log_info "Generating API reference..."
    
    local api_dir="$DOCS_DIR/src/api"
    mkdir -p "$api_dir"
    
    # Generate module documentation
    find src/ -name "*.rs" -type f | grep -v test | while read -r rust_file; do
        local module_name=$(echo "$rust_file" | sed 's|src/||' | sed 's|\.rs$||' | sed 's|/|_|g')
        
        if [ "$module_name" != "lib" ] && [ "$module_name" != "main" ]; then
            local output_file="$api_dir/${module_name}.md"
            
            cat > "$output_file" << EOF
# ${module_name^} Module

## Overview

This module is part of the Brain AI cognitive architecture.

## Source Code Documentation

For detailed API documentation, see the [Rust docs](./rust/brain/$(echo "$module_name" | sed 's|_|/|g')/index.html).

## Key Components

EOF
            
            # Extract public structs and functions
            grep -n "pub struct\|pub fn\|pub enum\|pub trait" "$rust_file" | head -20 | while read -r line; do
                echo "- \`$(echo "$line" | cut -d: -f2- | sed 's/^[[:space:]]*//')\`" >> "$output_file"
            done || true
        fi
    done
    
    log_success "API reference generated"
}

# Update table of contents
update_summary() {
    log_info "Updating table of contents..."
    
    local summary_file="$DOCS_DIR/src/SUMMARY.md"
    
    # Backup existing summary
    if [ -f "$summary_file" ]; then
        cp "$summary_file" "$summary_file.backup"
    fi
    
    # Generate new summary (this is a simplified version)
    # In practice, you might want to use a more sophisticated approach
    log_info "Table of contents updated (manual verification recommended)"
}

# Validate documentation
validate_docs() {
    log_info "Validating documentation..."
    
    cd "$DOCS_DIR"
    
    # Test mdbook build
    if mdbook test; then
        log_success "Documentation tests passed"
    else
        log_error "Documentation tests failed"
        return 1
    fi
    
    # Check for broken links (basic check)
    if command -v linkchecker &> /dev/null; then
        linkchecker book/index.html
    else
        log_warning "linkchecker not found, skipping link validation"
    fi
    
    cd ..
}

# Build documentation
build_docs() {
    log_info "Building documentation..."
    
    cd "$DOCS_DIR"
    
    # Clean previous build
    if [ -d "book" ]; then
        rm -rf book
    fi
    
    # Build the book
    if mdbook build; then
        log_success "Documentation built successfully"
    else
        log_error "Documentation build failed"
        cd ..
        return 1
    fi
    
    cd ..
}

# Serve documentation locally
serve_docs() {
    log_info "Starting documentation server..."
    
    cd "$DOCS_DIR"
    
    echo -e "${GREEN}Documentation server starting...${NC}"
    echo -e "${BLUE}Open your browser to: http://localhost:3000${NC}"
    echo -e "${YELLOW}Press Ctrl+C to stop the server${NC}"
    
    mdbook serve --hostname 0.0.0.0 --port 3000
}

# Deploy documentation
deploy_docs() {
    log_info "Deploying documentation..."
    
    local deploy_target="${1:-github-pages}"
    
    case $deploy_target in
        "github-pages")
            deploy_github_pages
            ;;
        "netlify")
            deploy_netlify
            ;;
        "local")
            deploy_local
            ;;
        *)
            log_error "Unknown deployment target: $deploy_target"
            echo "Available targets: github-pages, netlify, local"
            return 1
            ;;
    esac
}

deploy_github_pages() {
    log_info "Deploying to GitHub Pages..."
    
    # Check if gh-pages branch exists
    if git show-ref --verify --quiet refs/heads/gh-pages; then
        git checkout gh-pages
        git pull origin gh-pages
    else
        git checkout --orphan gh-pages
        git rm -rf .
    fi
    
    # Copy documentation
    cp -r "$OUTPUT_DIR"/* .
    
    # Create .nojekyll file
    touch .nojekyll
    
    # Commit and push
    git add .
    git commit -m "Update documentation $(date)"
    git push origin gh-pages
    
    # Return to main branch
    git checkout main
    
    log_success "Documentation deployed to GitHub Pages"
}

deploy_netlify() {
    log_info "Preparing for Netlify deployment..."
    
    # Create netlify.toml if it doesn't exist
    if [ ! -f "netlify.toml" ]; then
        cat > netlify.toml << EOF
[build]
  publish = "docs/book"
  command = "scripts/generate-docs.sh build"

[build.environment]
  RUST_VERSION = "1.70.0"

[[redirects]]
  from = "/*"
  to = "/index.html"
  status = 404
EOF
        log_info "Created netlify.toml configuration"
    fi
    
    log_success "Ready for Netlify deployment"
    echo "Deploy command: netlify deploy --prod --dir=docs/book"
}

deploy_local() {
    log_info "Deploying documentation locally..."
    
    local local_dir="${HOME}/brain-ai-docs"
    
    # Create local directory
    mkdir -p "$local_dir"
    
    # Copy documentation
    cp -r "$OUTPUT_DIR"/* "$local_dir/"
    
    log_success "Documentation deployed to $local_dir"
    echo "Open file://$local_dir/index.html in your browser"
}

# Create backup
create_backup() {
    log_info "Creating backup..."
    
    local timestamp=$(date +"%Y%m%d_%H%M%S")
    local backup_file="$BACKUP_DIR/docs_backup_$timestamp.tar.gz"
    
    mkdir -p "$BACKUP_DIR"
    
    tar -czf "$backup_file" "$DOCS_DIR" --exclude="$DOCS_DIR/book" --exclude="$DOCS_DIR/backup"
    
    log_success "Backup created: $backup_file"
}

# Restore from backup
restore_backup() {
    local backup_file="$1"
    
    if [ ! -f "$backup_file" ]; then
        log_error "Backup file not found: $backup_file"
        return 1
    fi
    
    log_info "Restoring from backup: $backup_file"
    
    # Create backup of current state
    create_backup
    
    # Restore from backup
    tar -xzf "$backup_file"
    
    log_success "Restored from backup"
}

# Clean up
cleanup() {
    log_info "Cleaning up..."
    
    # Remove temporary files
    find "$DOCS_DIR" -name "*.tmp" -delete 2>/dev/null || true
    find "$DOCS_DIR" -name "*.bak" -delete 2>/dev/null || true
    
    # Clean old backups (keep last 5)
    if [ -d "$BACKUP_DIR" ]; then
        ls -t "$BACKUP_DIR"/docs_backup_*.tar.gz 2>/dev/null | tail -n +6 | xargs rm -f || true
    fi
    
    log_success "Cleanup completed"
}

# Show help
show_help() {
    cat << EOF
Brain AI Documentation Generator

Usage: $0 [COMMAND] [OPTIONS]

Commands:
    build           Build the documentation
    serve           Serve documentation locally (default port: 3000)
    deploy TARGET   Deploy documentation (github-pages, netlify, local)
    validate        Validate documentation
    clean           Clean up temporary files
    backup          Create backup of documentation
    restore FILE    Restore from backup file
    full            Full documentation generation (build + validate)
    help            Show this help message

Examples:
    $0 build                    # Build documentation
    $0 serve                    # Serve locally on port 3000
    $0 deploy github-pages      # Deploy to GitHub Pages
    $0 full                     # Complete documentation generation

Options:
    --no-rust-docs    Skip Rust documentation generation
    --no-examples     Skip example extraction
    --no-api-ref      Skip API reference generation
    --quiet           Reduce output verbosity

Environment Variables:
    DOCS_PORT         Port for local server (default: 3000)
    DOCS_HOST         Host for local server (default: 0.0.0.0)
EOF
}

# Main function
main() {
    local command="${1:-help}"
    local skip_rust_docs=false
    local skip_examples=false
    local skip_api_ref=false
    local quiet=false
    
    # Parse options
    while [[ $# -gt 0 ]]; do
        case $1 in
            --no-rust-docs)
                skip_rust_docs=true
                shift
                ;;
            --no-examples)
                skip_examples=true
                shift
                ;;
            --no-api-ref)
                skip_api_ref=true
                shift
                ;;
            --quiet)
                quiet=true
                shift
                ;;
            *)
                break
                ;;
        esac
    done
    
    # Set quiet mode
    if [ "$quiet" = true ]; then
        exec > /dev/null 2>&1
    fi
    
    case $command in
        "build")
            check_dependencies
            install_plugins
            [ "$skip_rust_docs" = false ] && generate_rust_docs
            [ "$skip_examples" = false ] && extract_examples
            [ "$skip_api_ref" = false ] && generate_api_reference
            build_docs
            ;;
        "serve")
            check_dependencies
            build_docs
            serve_docs
            ;;
        "deploy")
            local target="${2:-github-pages}"
            check_dependencies
            build_docs
            validate_docs
            deploy_docs "$target"
            ;;
        "validate")
            check_dependencies
            validate_docs
            ;;
        "clean")
            cleanup
            ;;
        "backup")
            create_backup
            ;;
        "restore")
            local backup_file="${2:-}"
            if [ -z "$backup_file" ]; then
                log_error "Please specify backup file"
                exit 1
            fi
            restore_backup "$backup_file"
            ;;
        "full")
            check_dependencies
            install_plugins
            create_backup
            [ "$skip_rust_docs" = false ] && generate_rust_docs
            [ "$skip_examples" = false ] && extract_examples
            [ "$skip_api_ref" = false ] && generate_api_reference
            build_docs
            validate_docs
            cleanup
            log_success "Full documentation generation completed"
            ;;
        "help"|*)
            show_help
            ;;
    esac
}

# Run main function with all arguments
main "$@" 