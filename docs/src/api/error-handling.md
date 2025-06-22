# Error Handling

Brain AI provides comprehensive error handling mechanisms to help developers build robust applications. This document covers error types, response formats, debugging techniques, and best practices for handling errors gracefully.

## Error Response Format

All API errors follow a consistent JSON structure:

```json
{
  "error": "error_code",
  "message": "Human-readable error description",
  "details": {
    "field": "specific_field_if_applicable",
    "code": "DETAILED_ERROR_CODE",
    "suggestions": ["Possible solutions or next steps"],
    "context": {
      "request_id": "req_550e8400-e29b-41d4-a716-446655440000",
      "timestamp": "2024-01-02T10:30:00Z",
      "component": "memory_system"
    }
  },
  "request_id": "req_550e8400-e29b-41d4-a716-446655440000",
  "timestamp": "2024-01-02T10:30:00Z"
}
```

## HTTP Status Codes

Brain AI uses standard HTTP status codes with specific meanings:

### 2xx Success Codes
- **200 OK**: Request successful, data returned
- **201 Created**: Resource created successfully
- **202 Accepted**: Request accepted for processing (async operations)
- **204 No Content**: Request successful, no data to return

### 4xx Client Error Codes
- **400 Bad Request**: Invalid request format or parameters
- **401 Unauthorized**: Authentication required or invalid
- **403 Forbidden**: Insufficient permissions for the operation
- **404 Not Found**: Requested resource does not exist
- **409 Conflict**: Request conflicts with current state
- **413 Payload Too Large**: Request body exceeds size limits
- **422 Unprocessable Entity**: Valid format but semantic errors
- **429 Too Many Requests**: Rate limit exceeded

### 5xx Server Error Codes
- **500 Internal Server Error**: Unexpected server error
- **502 Bad Gateway**: Upstream service error
- **503 Service Unavailable**: Service temporarily unavailable
- **504 Gateway Timeout**: Request timeout

## Error Categories

### Authentication Errors

#### Invalid Token (401)
```json
{
  "error": "invalid_token",
  "message": "The provided authentication token is invalid or expired",
  "details": {
    "code": "TOKEN_INVALID",
    "suggestions": [
      "Check if your token is correctly formatted",
      "Refresh your token if it has expired",
      "Re-authenticate to get a new token"
    ]
  }
}
```

### Rate Limiting Errors

#### Rate Limit Exceeded (429)
```json
{
  "error": "rate_limit_exceeded",
  "message": "Too many requests. Please slow down.",
  "details": {
    "code": "RATE_LIMIT_EXCEEDED",
    "limit": 100,
    "window": "1 minute",
    "retry_after": 45,
    "suggestions": [
      "Wait 45 seconds before making another request",
      "Implement exponential backoff in your client"
    ]
  }
}
```

### Validation Errors

#### Invalid Request Parameters (400)
```json
{
  "error": "invalid_request",
  "message": "One or more request parameters are invalid",
  "details": {
    "code": "VALIDATION_ERROR",
    "validation_errors": [
      {
        "field": "text",
        "message": "Text content is required and cannot be empty",
        "code": "REQUIRED_FIELD_MISSING"
      }
    ]
  }
}
```

## Error Handling Best Practices

### JavaScript Example
```javascript
class BrainAIClient {
  async makeRequest(endpoint, options = {}) {
    try {
      const response = await fetch(`${this.baseUrl}${endpoint}`, {
        ...options,
        headers: {
          'Authorization': `Bearer ${this.token}`,
          'Content-Type': 'application/json',
          ...options.headers
        }
      });

      if (!response.ok) {
        await this.handleErrorResponse(response);
      }

      return await response.json();
    } catch (error) {
      console.error('Request failed:', error);
      throw error;
    }
  }

  async handleErrorResponse(response) {
    const errorData = await response.json();
    
    switch (response.status) {
      case 401:
        await this.handleAuthError(errorData);
        break;
      case 429:
        await this.handleRateLimitError(errorData);
        break;
      case 500:
        this.handleServerError(errorData);
        break;
      default:
        throw new Error(`API Error: ${errorData.message}`);
    }
  }
}
```

### Python Example
```python
import requests
import time
from typing import Dict, Any

class BrainAIError(Exception):
    def __init__(self, message: str, error_code: str = None, details: Dict = None):
        super().__init__(message)
        self.error_code = error_code
        self.details = details or {}

class BrainAIClient:
    def make_request(self, method: str, endpoint: str, **kwargs) -> Dict[str, Any]:
        headers = kwargs.get('headers', {})
        headers['Authorization'] = f'Bearer {self.token}'
        kwargs['headers'] = headers

        try:
            response = requests.request(method, f"{self.base_url}{endpoint}", **kwargs)
            
            if not response.ok:
                self._handle_error_response(response)
            
            return response.json()
        except requests.RequestException as e:
            raise BrainAIError(f"Network error: {e}")

    def _handle_error_response(self, response: requests.Response):
        try:
            error_data = response.json()
        except ValueError:
            raise BrainAIError(f"HTTP {response.status_code}: {response.text}")

        message = error_data.get('message', 'An error occurred')
        error_code = error_data.get('error', 'unknown_error')
        details = error_data.get('details', {})
        
        raise BrainAIError(message, error_code, details)
```

## Debugging Tips

### Enable Debug Mode

Add debug headers to requests for more detailed error information:

```http
X-Debug-Mode: true
X-Debug-Level: verbose
```

### Common Troubleshooting Steps

1. **Check API Status**: Verify service health at `/api/v1/health`
2. **Validate Token**: Use `/api/v1/auth/validate` to check token status
3. **Review Rate Limits**: Check response headers for rate limit information
4. **Verify Permissions**: Ensure your account has required permissions
5. **Check Payload Size**: Verify request doesn't exceed size limits
6. **Test with curl**: Isolate issues using direct HTTP requests

Brain AI provides comprehensive error handling mechanisms to help developers build robust applications. This document covers error types, response formats, debugging techniques, and best practices for handling errors gracefully.

## Error Response Format

All API errors follow a consistent JSON structure:

```json
{
  "error": "error_code",
  "message": "Human-readable error description",
  "details": {
    "field": "specific_field_if_applicable",
    "code": "DETAILED_ERROR_CODE",
    "suggestions": ["Possible solutions or next steps"],
    "context": {
      "request_id": "req_550e8400-e29b-41d4-a716-446655440000",
      "timestamp": "2024-01-02T10:30:00Z",
      "component": "memory_system"
    }
  },
  "request_id": "req_550e8400-e29b-41d4-a716-446655440000",
  "timestamp": "2024-01-02T10:30:00Z"
}
```

## HTTP Status Codes

Brain AI uses standard HTTP status codes with specific meanings:

### 2xx Success Codes
- **200 OK**: Request successful, data returned
- **201 Created**: Resource created successfully
- **202 Accepted**: Request accepted for processing (async operations)
- **204 No Content**: Request successful, no data to return

### 4xx Client Error Codes
- **400 Bad Request**: Invalid request format or parameters
- **401 Unauthorized**: Authentication required or invalid
- **403 Forbidden**: Insufficient permissions for the operation
- **404 Not Found**: Requested resource does not exist
- **409 Conflict**: Request conflicts with current state
- **413 Payload Too Large**: Request body exceeds size limits
- **422 Unprocessable Entity**: Valid format but semantic errors
- **429 Too Many Requests**: Rate limit exceeded

### 5xx Server Error Codes
- **500 Internal Server Error**: Unexpected server error
- **502 Bad Gateway**: Upstream service error
- **503 Service Unavailable**: Service temporarily unavailable
- **504 Gateway Timeout**: Request timeout

## Error Categories

### Authentication Errors

#### Invalid Token (401)
```json
{
  "error": "invalid_token",
  "message": "The provided authentication token is invalid or expired",
  "details": {
    "code": "TOKEN_INVALID",
    "suggestions": [
      "Check if your token is correctly formatted",
      "Refresh your token if it has expired",
      "Re-authenticate to get a new token"
    ],
    "token_info": {
      "expires_at": "2024-01-02T10:00:00Z",
      "issued_at": "2024-01-02T09:00:00Z",
      "status": "expired"
    }
  }
}
```

#### Insufficient Permissions (403)
```json
{
  "error": "insufficient_permissions",
  "message": "Your account does not have permission to perform this action",
  "details": {
    "code": "PERMISSION_DENIED",
    "required_permission": "admin",
    "user_permissions": ["read", "write"],
    "suggestions": [
      "Contact your administrator to request elevated permissions",
      "Use an endpoint that matches your permission level"
    ]
  }
}
```

### Rate Limiting Errors

#### Rate Limit Exceeded (429)
```json
{
  "error": "rate_limit_exceeded",
  "message": "Too many requests. Please slow down.",
  "details": {
    "code": "RATE_LIMIT_EXCEEDED",
    "limit": 100,
    "window": "1 minute",
    "retry_after": 45,
    "current_usage": 105,
    "suggestions": [
      "Wait 45 seconds before making another request",
      "Implement exponential backoff in your client",
      "Consider upgrading your plan for higher limits"
    ]
  }
}
```

### Validation Errors

#### Invalid Request Parameters (400)
```json
{
  "error": "invalid_request",
  "message": "One or more request parameters are invalid",
  "details": {
    "code": "VALIDATION_ERROR",
    "validation_errors": [
      {
        "field": "text",
        "message": "Text content is required and cannot be empty",
        "code": "REQUIRED_FIELD_MISSING"
      },
      {
        "field": "priority",
        "message": "Priority must be one of: low, medium, high, critical",
        "code": "INVALID_ENUM_VALUE",
        "provided_value": "urgent",
        "allowed_values": ["low", "medium", "high", "critical"]
      }
    ],
    "suggestions": [
      "Provide valid text content in the request body",
      "Use a valid priority value"
    ]
  }
}
```

#### Payload Too Large (413)
```json
{
  "error": "payload_too_large",
  "message": "Request payload exceeds maximum allowed size",
  "details": {
    "code": "PAYLOAD_TOO_LARGE",
    "max_size": "1MB",
    "provided_size": "1.5MB",
    "suggestions": [
      "Reduce the size of your text content",
      "Split large texts into smaller chunks",
      "Use batch processing for multiple items"
    ]
  }
}
```

### Resource Errors

#### Resource Not Found (404)
```json
{
  "error": "resource_not_found",
  "message": "The requested resource could not be found",
  "details": {
    "code": "RESOURCE_NOT_FOUND",
    "resource_type": "memory",
    "resource_id": "mem_nonexistent_id",
    "suggestions": [
      "Check if the resource ID is correct",
      "Verify the resource exists and you have access to it",
      "Use the search endpoint to find available resources"
    ]
  }
}
```

#### Resource Conflict (409)
```json
{
  "error": "resource_conflict",
  "message": "The request conflicts with the current state of the resource",
  "details": {
    "code": "RESOURCE_CONFLICT",
    "conflict_type": "duplicate_concept",
    "existing_resource": {
      "id": "concept_123",
      "name": "machine_learning"
    },
    "suggestions": [
      "Use a different concept name",
      "Update the existing concept instead",
      "Check for existing resources before creating new ones"
    ]
  }
}
```

### System Errors

#### Memory System Error (500)
```json
{
  "error": "memory_system_error",
  "message": "An error occurred in the memory system",
  "details": {
    "code": "MEMORY_SYSTEM_ERROR",
    "component": "memory_consolidation",
    "error_type": "storage_full",
    "suggestions": [
      "Try again in a few moments",
      "Clear old memories to free up space",
      "Contact support if the problem persists"
    ],
    "debug_info": {
      "memory_usage": "95%",
      "consolidation_status": "failed"
    }
  }
}
```

#### Service Unavailable (503)
```json
{
  "error": "service_unavailable",
  "message": "The service is temporarily unavailable",
  "details": {
    "code": "SERVICE_UNAVAILABLE",
    "reason": "scheduled_maintenance",
    "estimated_recovery": "2024-01-02T12:00:00Z",
    "suggestions": [
      "Retry your request after the estimated recovery time",
      "Check the status page for updates",
      "Implement retry logic with exponential backoff"
    ]
  }
}
```

## Error Handling Best Practices

### Client-Side Error Handling

#### JavaScript/Node.js Example
```javascript
class BrainAIClient {
  async makeRequest(endpoint, options = {}) {
    try {
      const response = await fetch(`${this.baseUrl}${endpoint}`, {
        ...options,
        headers: {
          'Authorization': `Bearer ${this.token}`,
          'Content-Type': 'application/json',
          ...options.headers
        }
      });

      if (!response.ok) {
        await this.handleErrorResponse(response);
      }

      return await response.json();
    } catch (error) {
      console.error('Request failed:', error);
      throw error;
    }
  }

  async handleErrorResponse(response) {
    const errorData = await response.json();
    
    switch (response.status) {
      case 401:
        await this.handleAuthError(errorData);
        break;
      case 429:
        await this.handleRateLimitError(errorData);
        break;
      case 500:
        this.handleServerError(errorData);
        break;
      default:
        throw new Error(`API Error: ${errorData.message}`);
    }
  }

  async handleAuthError(errorData) {
    if (errorData.details?.code === 'TOKEN_INVALID') {
      // Try to refresh token
      await this.refreshToken();
      throw new Error('TOKEN_REFRESHED'); // Signal to retry
    }
    throw new Error('Authentication failed');
  }

  async handleRateLimitError(errorData) {
    const retryAfter = errorData.details?.retry_after || 60;
    console.log(`Rate limited. Retrying after ${retryAfter} seconds`);
    await new Promise(resolve => setTimeout(resolve, retryAfter * 1000));
    throw new Error('RATE_LIMITED'); // Signal to retry
  }

  handleServerError(errorData) {
    console.error('Server error:', errorData);
    // Log to monitoring service
    this.logError(errorData);
    throw new Error('Server error occurred');
  }
}
```

#### Python Example
```python
import requests
import time
from typing import Dict, Any
import logging

class BrainAIError(Exception):
    def __init__(self, message: str, error_code: str = None, details: Dict = None):
        super().__init__(message)
        self.error_code = error_code
        self.details = details or {}

class BrainAIClient:
    def __init__(self, base_url: str, token: str):
        self.base_url = base_url
        self.token = token
        self.logger = logging.getLogger(__name__)

    def make_request(self, method: str, endpoint: str, **kwargs) -> Dict[str, Any]:
        headers = kwargs.get('headers', {})
        headers['Authorization'] = f'Bearer {self.token}'
        kwargs['headers'] = headers

        try:
            response = requests.request(method, f"{self.base_url}{endpoint}", **kwargs)
            
            if not response.ok:
                self._handle_error_response(response)
            
            return response.json()
        except requests.RequestException as e:
            self.logger.error(f"Request failed: {e}")
            raise BrainAIError(f"Network error: {e}")

    def _handle_error_response(self, response: requests.Response):
        try:
            error_data = response.json()
        except ValueError:
            raise BrainAIError(f"HTTP {response.status_code}: {response.text}")

        error_code = error_data.get('error', 'unknown_error')
        message = error_data.get('message', 'An error occurred')
        details = error_data.get('details', {})

        if response.status_code == 401:
            self._handle_auth_error(error_data)
        elif response.status_code == 429:
            self._handle_rate_limit_error(error_data)
        elif response.status_code >= 500:
            self._handle_server_error(error_data)
        
        raise BrainAIError(message, error_code, details)

    def _handle_auth_error(self, error_data: Dict):
        if error_data.get('details', {}).get('code') == 'TOKEN_INVALID':
            # Attempt token refresh
            self.logger.info("Attempting to refresh token")
            # Implement token refresh logic here
        
        raise BrainAIError("Authentication failed", "auth_error", error_data.get('details', {}))

    def _handle_rate_limit_error(self, error_data: Dict):
        retry_after = error_data.get('details', {}).get('retry_after', 60)
        self.logger.warning(f"Rate limited. Waiting {retry_after} seconds")
        time.sleep(retry_after)
        raise BrainAIError("Rate limited", "rate_limit", error_data.get('details', {}))

    def _handle_server_error(self, error_data: Dict):
        self.logger.error(f"Server error: {error_data}")
        # Log to monitoring service
        raise BrainAIError("Server error", "server_error", error_data.get('details', {}))
```

### Retry Logic with Exponential Backoff

```javascript
class RetryableClient {
  async makeRequestWithRetry(endpoint, options = {}, maxRetries = 3) {
    let lastError;
    
    for (let attempt = 0; attempt <= maxRetries; attempt++) {
      try {
        return await this.makeRequest(endpoint, options);
      } catch (error) {
        lastError = error;
        
        if (attempt === maxRetries) {
          break; // Don't retry on last attempt
        }
        
        if (this.shouldRetry(error)) {
          const delay = this.calculateBackoffDelay(attempt);
          console.log(`Attempt ${attempt + 1} failed. Retrying in ${delay}ms`);
          await new Promise(resolve => setTimeout(resolve, delay));
        } else {
          throw error; // Don't retry certain errors
        }
      }
    }
    
    throw lastError;
  }

  shouldRetry(error) {
    // Retry on network errors, 5xx errors, and rate limits
    return error.message.includes('RATE_LIMITED') ||
           error.message.includes('TOKEN_REFRESHED') ||
           error.status >= 500;
  }

  calculateBackoffDelay(attempt) {
    // Exponential backoff with jitter
    const baseDelay = 1000; // 1 second
    const backoffFactor = 2;
    const maxDelay = 30000; // 30 seconds
    
    const delay = Math.min(baseDelay * Math.pow(backoffFactor, attempt), maxDelay);
    const jitter = Math.random() * 0.1 * delay; // 10% jitter
    
    return delay + jitter;
  }
}
```

### Error Monitoring and Logging

```javascript
class ErrorMonitor {
  constructor(apiKey) {
    this.apiKey = apiKey;
    this.errorCounts = new Map();
  }

  logError(error, context = {}) {
    const errorKey = `${error.error_code}_${error.status}`;
    const count = this.errorCounts.get(errorKey) || 0;
    this.errorCounts.set(errorKey, count + 1);

    // Log to external monitoring service
    this.sendToMonitoring({
      error_code: error.error_code,
      message: error.message,
      status: error.status,
      context: context,
      count: count + 1,
      timestamp: new Date().toISOString()
    });

    // Alert if error rate is high
    if (count > 10) {
      this.sendAlert(`High error rate for ${errorKey}: ${count} occurrences`);
    }
  }

  sendToMonitoring(data) {
    // Implementation for your monitoring service
    console.log('Monitoring:', data);
  }

  sendAlert(message) {
    // Implementation for alerting
    console.warn('ALERT:', message);
  }
}
```

## Debugging Tips

### Enable Debug Mode

Add debug headers to requests for more detailed error information:

```http
X-Debug-Mode: true
X-Debug-Level: verbose
```

Response will include additional debug information:

```json
{
  "error": "memory_system_error",
  "message": "Memory consolidation failed",
  "details": {
    "code": "CONSOLIDATION_FAILED",
    "debug_info": {
      "memory_usage": "95%",
      "consolidation_queue_size": 1247,
      "last_successful_consolidation": "2024-01-02T09:30:00Z",
      "error_stack": "ConsolidationError: Insufficient space..."
    }
  }
}
```

### Request Tracing

Use request IDs to trace errors across system components:

```bash
curl -X POST "http://localhost:8080/api/v1/learn" \
  -H "Authorization: Bearer YOUR_TOKEN" \
  -H "X-Request-ID: trace-12345" \
  -H "Content-Type: application/json" \
  -d '{"text": "Debug this request"}'
```

### Common Troubleshooting Steps

1. **Check API Status**: Verify service health at `/api/v1/health`
2. **Validate Token**: Use `/api/v1/auth/validate` to check token status
3. **Review Rate Limits**: Check response headers for rate limit information
4. **Verify Permissions**: Ensure your account has required permissions
5. **Check Payload Size**: Verify request doesn't exceed size limits
6. **Test with curl**: Isolate issues using direct HTTP requests

### Error Recovery Strategies

#### Graceful Degradation
```javascript
class ResilientBrainAI {
  async learn(text, options = {}) {
    try {
      return await this.client.learn(text, options);
    } catch (error) {
      if (error.status === 503) {
        // Service unavailable - use local caching
        return this.cacheForLater(text, options);
      } else if (error.status === 429) {
        // Rate limited - queue for later
        return this.queueForLater(text, options);
      }
      throw error;
    }
  }

  async cacheForLater(text, options) {
    this.localCache.push({ text, options, timestamp: Date.now() });
    return { success: false, cached: true, message: "Cached for later processing" };
  }

  async queueForLater(text, options) {
    this.processingQueue.push({ text, options, timestamp: Date.now() });
    return { success: false, queued: true, message: "Queued for later processing" };
  }
}
```

This comprehensive error handling guide ensures robust integration with Brain AI's API, providing clear guidance for handling various error scenarios gracefully.
