# Configuration Management

Brain AI provides flexible configuration options through environment variables, TOML configuration files, and command-line arguments.

## Configuration Methods

Brain AI supports multiple configuration methods with the following precedence order:

1. **Command-line arguments** (highest priority)
2. **Environment variables**
3. **Configuration files** (TOML)
4. **Default values** (lowest priority)

## Environment Variables

### Core Configuration

```bash
# API Keys and External Services
ANTHROPIC_API_KEY=your_anthropic_api_key_here
PERPLEXITY_API_KEY=your_perplexity_api_key_here

# Model Configuration
MODEL=claude-3-opus-20240229
MAX_TOKENS=8192
TEMPERATURE=0.7

# System Configuration
LOG_LEVEL=info                    # debug, info, warn, error
DEBUG=false                       # Enable debug mode
MEMORY_CAPACITY=1000000          # Maximum number of memories
```

### Network and Security

```bash
# Server Configuration
HOST=0.0.0.0                     # Bind address
PORT=8080                        # Server port
CORS_ORIGIN=*                    # CORS allowed origins

# Authentication
JWT_SECRET=your-secret-key-here
JWT_EXPIRES_IN=3600              # Token expiration in seconds
BCRYPT_ROUNDS=12                 # Password hashing rounds
```

## TOML Configuration Files

### Main Configuration File

Create `config/brain.toml`:

```toml
[system]
project_name = "brain-ai"
log_level = "info"
debug = false

[api]
host = "0.0.0.0"
port = 8080
cors_origin = "*"

[auth]
jwt_expires_in = 3600
bcrypt_rounds = 12

[memory]
capacity = 1000000
default_priority = "medium"
consolidation_threshold = 0.8

[performance]
enable_monitoring = true
metrics_interval = 60
```

## Environment-Specific Configurations

### Development Configuration

```toml
[system]
log_level = "debug"
debug = true

[memory]
capacity = 10000

[auth]
jwt_expires_in = 86400  # 24 hours for development
```

### Production Configuration

```toml
[system]
log_level = "info"
debug = false

[memory]
capacity = 10000000

[auth]
jwt_expires_in = 3600   # 1 hour
```

## Security Considerations

### Sensitive Data

Never store sensitive data in configuration files:

```bash
# ❌ Don't do this
jwt_secret = "my-secret-key"

# ✅ Use environment variables
jwt_secret = "${JWT_SECRET}"
```

### File Permissions

```bash
# Configuration files should be readable only by the application user
chmod 600 config/*.toml
chown brain:brain config/*.toml
```

This configuration guide provides the essentials for properly configuring Brain AI across different environments.
