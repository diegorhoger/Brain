//! HTTP Infrastructure
//! 
//! HTTP client utilities and external API integrations for the Brain AI system.

use brain_types::*;
use base64::{engine::general_purpose, Engine as _};
use reqwest::{Client, Response};
use serde_json::Value;
use std::collections::HashMap;
use std::time::Duration;

/// HTTP client manager for external API calls
pub struct HttpClient {
    client: Client,
    base_url: Option<String>,
    default_headers: HashMap<String, String>,
}

impl HttpClient {
    /// Create a new HTTP client
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            base_url: None,
            default_headers: HashMap::new(),
        }
    }

    /// Create a new HTTP client with custom timeout
    pub fn with_timeout(timeout_secs: u64) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(timeout_secs))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            base_url: None,
            default_headers: HashMap::new(),
        }
    }

    /// Set the base URL for all requests
    pub fn with_base_url(mut self, base_url: String) -> Self {
        self.base_url = Some(base_url);
        self
    }

    /// Add a default header
    pub fn with_header(mut self, key: String, value: String) -> Self {
        self.default_headers.insert(key, value);
        self
    }

    /// Make a GET request
    pub async fn get(&self, path: &str) -> Result<Response> {
        let url = self.build_url(path);
        let mut request = self.client.get(&url);

        for (key, value) in &self.default_headers {
            request = request.header(key, value);
        }

        let response = request
            .send()
            .await
            .map_err(|e| BrainError::HttpError(format!("GET request failed: {}", e)))?;

        Ok(response)
    }

    /// Make a POST request with JSON body
    pub async fn post_json(&self, path: &str, body: &Value) -> Result<Response> {
        let url = self.build_url(path);
        let mut request = self.client.post(&url).json(body);

        for (key, value) in &self.default_headers {
            request = request.header(key, value);
        }

        let response = request
            .send()
            .await
            .map_err(|e| BrainError::HttpError(format!("POST request failed: {}", e)))?;

        Ok(response)
    }

    /// Make a PUT request with JSON body
    pub async fn put_json(&self, path: &str, body: &Value) -> Result<Response> {
        let url = self.build_url(path);
        let mut request = self.client.put(&url).json(body);

        for (key, value) in &self.default_headers {
            request = request.header(key, value);
        }

        let response = request
            .send()
            .await
            .map_err(|e| BrainError::HttpError(format!("PUT request failed: {}", e)))?;

        Ok(response)
    }

    /// Make a DELETE request
    pub async fn delete(&self, path: &str) -> Result<Response> {
        let url = self.build_url(path);
        let mut request = self.client.delete(&url);

        for (key, value) in &self.default_headers {
            request = request.header(key, value);
        }

        let response = request
            .send()
            .await
            .map_err(|e| BrainError::HttpError(format!("DELETE request failed: {}", e)))?;

        Ok(response)
    }

    /// Get JSON response from a response object
    pub async fn get_json(response: Response) -> Result<Value> {
        response
            .json()
            .await
            .map_err(|e| BrainError::HttpError(format!("Failed to parse JSON response: {}", e)))
    }

    /// Get text response from a response object
    pub async fn get_text(response: Response) -> Result<String> {
        response
            .text()
            .await
            .map_err(|e| BrainError::HttpError(format!("Failed to get text response: {}", e)))
    }

    /// Build the full URL from path
    fn build_url(&self, path: &str) -> String {
        match &self.base_url {
            Some(base) => {
                if path.starts_with('/') {
                    format!("{}{}", base, path)
                } else {
                    format!("{}/{}", base, path)
                }
            }
            None => path.to_string(),
        }
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new()
    }
}

/// GitHub API client
pub struct GitHubClient {
    http_client: HttpClient,
}

impl GitHubClient {
    /// Create a new GitHub client with authentication token
    pub fn new(token: String) -> Self {
        let http_client = HttpClient::new()
            .with_base_url("https://api.github.com".to_string())
            .with_header("Authorization".to_string(), format!("token {}", token))
            .with_header("User-Agent".to_string(), "Brain-AI/1.0".to_string());

        Self { http_client }
    }

    /// Get repository information
    pub async fn get_repository(&self, owner: &str, repo: &str) -> Result<Value> {
        let path = format!("/repos/{}/{}", owner, repo);
        let response = self.http_client.get(&path).await?;
        HttpClient::get_json(response).await
    }

    /// Get repository contents
    pub async fn get_contents(&self, owner: &str, repo: &str, path: &str) -> Result<Value> {
        let api_path = format!("/repos/{}/{}/contents/{}", owner, repo, path);
        let response = self.http_client.get(&api_path).await?;
        HttpClient::get_json(response).await
    }

    /// Get file content (decoded from base64)
    pub async fn get_file_content(&self, owner: &str, repo: &str, path: &str) -> Result<String> {
        let contents = self.get_contents(owner, repo, path).await?;
        
        if let Some(content_b64) = contents.get("content").and_then(|c| c.as_str()) {
            let content_bytes = general_purpose::STANDARD.decode(content_b64.replace('\n', ""))
                .map_err(|e| BrainError::HttpError(format!("Failed to decode base64 content: {}", e)))?;
            
            String::from_utf8(content_bytes)
                .map_err(|e| BrainError::HttpError(format!("Failed to convert to UTF-8: {}", e)))
        } else {
            Err(BrainError::HttpError("No content found in response".to_string()))
        }
    }
} 