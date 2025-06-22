# Authentication

Brain AI uses JWT (JSON Web Token) based authentication for secure API access. This document covers authentication methods, token management, and security best practices.

## Overview

All API endpoints (except public health checks) require authentication using Bearer tokens. The authentication system supports:

- JWT-based authentication with configurable expiration
- Role-based access control (RBAC)
- Rate limiting per authenticated user
- Token refresh capabilities
- Secure token storage recommendations

## Getting Started

### 1. User Registration

Create a new user account (if registration is enabled):

**Endpoint:** `POST /api/v1/auth/register`

**Request Body:**
```json
{
  "username": "john_doe",
  "email": "john@example.com",
  "password": "secure_password_123",
  "full_name": "John Doe",
  "organization": "Example Corp"
}
```

**Response (201 Created):**
```json
{
  "success": true,
  "message": "User registered successfully",
  "user_id": "user_550e8400-e29b-41d4-a716-446655440000",
  "requires_verification": true
}
```

### 2. User Login

Authenticate and receive access tokens:

**Endpoint:** `POST /api/v1/auth/login`

**Request Body:**
```json
{
  "username": "john_doe",
  "password": "secure_password_123",
  "remember_me": true
}
```

**Response (200 OK):**
```json
{
  "success": true,
  "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IlJlZnJlc2gifQ...",
  "token_type": "Bearer",
  "expires_in": 3600,
  "refresh_expires_in": 604800,
  "user": {
    "id": "user_550e8400-e29b-41d4-a716-446655440000",
    "username": "john_doe",
    "email": "john@example.com",
    "roles": ["user"],
    "permissions": ["read", "write"]
  }
}
```

### 3. Using Access Tokens

Include the access token in the Authorization header for all API requests:

```http
Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
```

**Example Request:**
```bash
curl -X GET "http://localhost:8080/api/v1/memory/search?query=test" \
  -H "Authorization: Bearer YOUR_ACCESS_TOKEN" \
  -H "Content-Type: application/json"
```

## Token Management

### Token Refresh

Refresh expired access tokens using the refresh token:

**Endpoint:** `POST /api/v1/auth/refresh`

**Request Body:**
```json
{
  "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IlJlZnJlc2gifQ..."
}
```

**Response (200 OK):**
```json
{
  "success": true,
  "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "token_type": "Bearer",
  "expires_in": 3600
}
```

### Token Validation

Validate token status and get user information:

**Endpoint:** `GET /api/v1/auth/validate`

**Headers:**
```http
Authorization: Bearer YOUR_ACCESS_TOKEN
```

**Response (200 OK):**
```json
{
  "valid": true,
  "user": {
    "id": "user_550e8400-e29b-41d4-a716-446655440000",
    "username": "john_doe",
    "roles": ["user"],
    "permissions": ["read", "write"]
  },
  "token_info": {
    "issued_at": "2024-01-02T10:00:00Z",
    "expires_at": "2024-01-02T11:00:00Z",
    "remaining_seconds": 2847
  }
}
```

### Logout

Invalidate tokens and end the session:

**Endpoint:** `POST /api/v1/auth/logout`

**Headers:**
```http
Authorization: Bearer YOUR_ACCESS_TOKEN
```

**Request Body:**
```json
{
  "invalidate_refresh_token": true
}
```

**Response (200 OK):**
```json
{
  "success": true,
  "message": "Logged out successfully"
}
```

## Role-Based Access Control

Brain AI implements role-based access control with the following default roles:

### User Roles

1. **Guest** (read-only access)
   - View public content
   - Basic health checks
   - Limited API access

2. **User** (standard access)
   - Learn from text
   - Query memory and concepts
   - Access personal data
   - Generate insights

3. **Power User** (advanced features)
   - Batch operations
   - Advanced analytics
   - System monitoring
   - Export capabilities

4. **Admin** (full system access)
   - User management
   - System configuration
   - Performance monitoring
   - All API endpoints

### Permission Matrix

| Endpoint Category | Guest | User | Power User | Admin |
|------------------|-------|------|------------|-------|
| Health Check     | ✅    | ✅   | ✅         | ✅    |
| Learning         | ❌    | ✅   | ✅         | ✅    |
| Memory Query     | ❌    | ✅   | ✅         | ✅    |
| Batch Operations | ❌    | ❌   | ✅         | ✅    |
| System Metrics   | ❌    | ❌   | ✅         | ✅    |
| User Management  | ❌    | ❌   | ❌         | ✅    |

## Security Best Practices

### Token Security

1. **Secure Storage:**
   ```javascript
   // Store tokens securely (avoid localStorage for sensitive apps)
   // Use httpOnly cookies or secure storage mechanisms
   const secureStorage = {
     setToken: (token) => {
       // Use secure storage implementation
       sessionStorage.setItem('brain_ai_token', token);
     },
     getToken: () => {
       return sessionStorage.getItem('brain_ai_token');
     }
   };
   ```

2. **Token Expiration Handling:**
   ```javascript
   // Implement automatic token refresh
   const apiCall = async (endpoint, options) => {
     try {
       const response = await fetch(endpoint, {
         ...options,
         headers: {
           'Authorization': `Bearer ${getToken()}`,
           ...options.headers
         }
       });
       
       if (response.status === 401) {
         // Token expired, refresh it
         await refreshToken();
         // Retry the original request
         return fetch(endpoint, options);
       }
       
       return response;
     } catch (error) {
       console.error('API call failed:', error);
       throw error;
     }
   };
   ```

### HTTPS Requirements

- **Production:** Always use HTTPS for token transmission
- **Development:** HTTPS recommended even in development
- **Token Headers:** Never include tokens in URL parameters

### Rate Limiting

Authentication includes rate limiting protection:

```http
X-RateLimit-Limit: 100
X-RateLimit-Remaining: 95
X-RateLimit-Reset: 1640995200
X-RateLimit-Category: auth
```

**Rate Limits by Role:**
- Guest: 10 requests/minute
- User: 100 requests/minute  
- Power User: 500 requests/minute
- Admin: 1000 requests/minute

## Error Handling

### Authentication Errors

**401 Unauthorized - Invalid Token:**
```json
{
  "error": "invalid_token",
  "message": "The provided token is invalid or expired",
  "details": {
    "code": "TOKEN_INVALID",
    "suggestions": ["Refresh your token", "Re-authenticate"]
  }
}
```

**403 Forbidden - Insufficient Permissions:**
```json
{
  "error": "insufficient_permissions",
  "message": "Your role does not have permission to access this resource",
  "details": {
    "required_permission": "admin",
    "user_permissions": ["read", "write"],
    "code": "PERMISSION_DENIED"
  }
}
```

**429 Too Many Requests:**
```json
{
  "error": "rate_limit_exceeded",
  "message": "Too many authentication attempts",
  "details": {
    "retry_after": 300,
    "limit": 5,
    "window": "5 minutes"
  }
}
```

## Integration Examples

### JavaScript/Node.js

```javascript
class BrainAIAuth {
  constructor(baseUrl) {
    this.baseUrl = baseUrl;
    this.accessToken = null;
    this.refreshToken = null;
  }

  async login(username, password) {
    const response = await fetch(`${this.baseUrl}/api/v1/auth/login`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ username, password })
    });

    if (response.ok) {
      const data = await response.json();
      this.accessToken = data.access_token;
      this.refreshToken = data.refresh_token;
      return data;
    }
    
    throw new Error('Login failed');
  }

  async makeAuthenticatedRequest(endpoint, options = {}) {
    const response = await fetch(`${this.baseUrl}${endpoint}`, {
      ...options,
      headers: {
        'Authorization': `Bearer ${this.accessToken}`,
        'Content-Type': 'application/json',
        ...options.headers
      }
    });

    if (response.status === 401) {
      await this.refreshAccessToken();
      return this.makeAuthenticatedRequest(endpoint, options);
    }

    return response;
  }

  async refreshAccessToken() {
    const response = await fetch(`${this.baseUrl}/api/v1/auth/refresh`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ refresh_token: this.refreshToken })
    });

    if (response.ok) {
      const data = await response.json();
      this.accessToken = data.access_token;
    } else {
      throw new Error('Token refresh failed');
    }
  }
}
```

### Python

```python
import requests
import time
from typing import Optional, Dict, Any

class BrainAIAuth:
    def __init__(self, base_url: str):
        self.base_url = base_url
        self.access_token: Optional[str] = None
        self.refresh_token: Optional[str] = None
        self.token_expires_at: Optional[float] = None

    def login(self, username: str, password: str) -> Dict[str, Any]:
        response = requests.post(
            f"{self.base_url}/api/v1/auth/login",
            json={"username": username, "password": password}
        )
        response.raise_for_status()
        
        data = response.json()
        self.access_token = data["access_token"]
        self.refresh_token = data["refresh_token"]
        self.token_expires_at = time.time() + data["expires_in"]
        
        return data

    def make_authenticated_request(self, method: str, endpoint: str, **kwargs) -> requests.Response:
        if self._token_needs_refresh():
            self.refresh_access_token()
        
        headers = kwargs.get('headers', {})
        headers['Authorization'] = f'Bearer {self.access_token}'
        kwargs['headers'] = headers
        
        response = requests.request(method, f"{self.base_url}{endpoint}", **kwargs)
        
        if response.status_code == 401:
            self.refresh_access_token()
            headers['Authorization'] = f'Bearer {self.access_token}'
            response = requests.request(method, f"{self.base_url}{endpoint}", **kwargs)
        
        return response

    def _token_needs_refresh(self) -> bool:
        if not self.token_expires_at:
            return False
        return time.time() >= (self.token_expires_at - 300)  # Refresh 5 minutes early

    def refresh_access_token(self):
        response = requests.post(
            f"{self.base_url}/api/v1/auth/refresh",
            json={"refresh_token": self.refresh_token}
        )
        response.raise_for_status()
        
        data = response.json()
        self.access_token = data["access_token"]
        self.token_expires_at = time.time() + data["expires_in"]
```

## Configuration

### Environment Variables

Configure authentication behavior using environment variables:

```bash
# JWT Configuration
JWT_SECRET=your-secret-key-here
JWT_EXPIRES_IN=3600
JWT_REFRESH_EXPIRES_IN=604800

# Security Settings
BCRYPT_ROUNDS=12
ENABLE_REGISTRATION=false
REQUIRE_EMAIL_VERIFICATION=true

# Rate Limiting
AUTH_RATE_LIMIT=5
AUTH_RATE_WINDOW=300

# CORS Settings
CORS_ORIGIN=https://your-frontend-domain.com
CORS_CREDENTIALS=true
```

### Database Schema

User authentication data is stored with the following structure:

```sql
-- Users table
CREATE TABLE users (
    id UUID PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    full_name VARCHAR(255),
    organization VARCHAR(255),
    role VARCHAR(50) DEFAULT 'user',
    is_active BOOLEAN DEFAULT true,
    email_verified BOOLEAN DEFAULT false,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

-- Refresh tokens table
CREATE TABLE refresh_tokens (
    id UUID PRIMARY KEY,
    user_id UUID REFERENCES users(id),
    token_hash VARCHAR(255) NOT NULL,
    expires_at TIMESTAMP NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);
```

This comprehensive authentication system ensures secure access to Brain AI's cognitive capabilities while providing flexibility for different integration scenarios.
