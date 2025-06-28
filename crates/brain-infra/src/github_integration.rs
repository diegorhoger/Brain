//! GitHub Integration Infrastructure
//! 
//! This module provides infrastructure for learning from GitHub repositories by:
//! - Fetching repository content via GitHub API
//! - Processing different file types (code, docs, README)
//! - Extracting meaningful information for learning
//! - Understanding repository structure and relationships

use brain_types::{Result, BrainError};
use brain_core::{Priority, WorkingMemoryRepository, WorkingMemoryItem};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use std::time::Instant;
use base64::{Engine as _, engine::general_purpose};

/// GitHub API client for repository access
pub struct GitHubClient {
    base_url: String,
    token: Option<String>,
    client: reqwest::Client,
}

/// Repository information extracted from GitHub
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryInfo {
    pub name: String,
    pub full_name: String,
    pub description: Option<String>,
    pub language: Option<String>,
    pub topics: Vec<String>,
    pub stars: u32,
    pub forks: u32,
    pub size: u32,
    pub license: Option<String>,
    pub readme_content: Option<String>,
    pub files: Vec<RepositoryFile>,
}

/// Individual file in a repository
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryFile {
    pub path: String,
    pub name: String,
    pub content: String,
    pub file_type: FileType,
    pub size: usize,
    pub language: Option<String>,
}

/// Types of files we can process
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum FileType {
    Documentation,  // README, docs, etc.
    Code,          // Source code files
    Configuration, // Config files, manifests
    Data,         // JSON, YAML, etc.
    Other,
}

/// GitHub learning configuration
#[derive(Debug, Clone)]
pub struct GitHubLearningConfig {
    pub max_files: usize,
    pub max_file_size: usize,
    pub include_code: bool,
    pub include_docs: bool,
    pub include_config: bool,
    pub priority_by_type: HashMap<FileType, Priority>,
}

impl Default for GitHubLearningConfig {
    fn default() -> Self {
        let mut priority_by_type = HashMap::new();
        priority_by_type.insert(FileType::Documentation, Priority::High);
        priority_by_type.insert(FileType::Code, Priority::Medium);
        priority_by_type.insert(FileType::Configuration, Priority::Low);
        priority_by_type.insert(FileType::Data, Priority::Medium);
        priority_by_type.insert(FileType::Other, Priority::Low);

        Self {
            max_files: 100,
            max_file_size: 100_000, // 100KB
            include_code: true,
            include_docs: true,
            include_config: true,
            priority_by_type,
        }
    }
}

/// Result of learning from a GitHub repository
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubLearningResult {
    pub repository: String,
    pub files_processed: usize,
    pub total_content_size: usize,
    pub learning_time_ms: u64,
    pub concepts_discovered: usize,
    pub memory_entries_created: usize,
    pub summary: String,
    pub key_insights: Vec<String>,
}

// Detailed analysis structures
#[derive(Debug, Clone)]
pub struct DetailedDataStructure {
    pub name: String,
    pub description: String,
    pub structure_type: String, // "class", "struct", "interface", "enum", etc.
    pub fields: Vec<String>,
    pub file_location: String,
}

#[derive(Debug, Clone)]
pub struct DetailedAPIEndpoint {
    pub method: String, // GET, POST, PUT, DELETE, etc.
    pub path: String,
    pub description: String,
    pub parameters: Vec<String>,
    pub response_type: Option<String>,
    pub file_location: String,
}

#[derive(Debug, Clone)]
pub struct DetailedArchitecturalPattern {
    pub name: String,
    pub description: String,
    pub pattern_type: String, // "architectural", "design", "framework"
    pub evidence: String,
    pub implementation_details: Vec<String>,
    pub files_involved: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct DetailedDependency {
    pub name: String,
    pub version: Option<String>,
    pub purpose: String,
    pub dependency_type: String, // "runtime", "dev", "peer", etc.
    pub source_file: String,
}

impl GitHubClient {
    /// Create a new GitHub client
    pub fn new(token: Option<String>) -> Self {
        Self {
            base_url: "https://api.github.com".to_string(),
            token,
            client: reqwest::Client::new(),
        }
    }

    /// Parse GitHub URL to extract owner and repository name
    pub fn parse_github_url(url: &str) -> Result<(String, String)> {
        let url = url.trim_end_matches('/');
        
        // Handle different GitHub URL formats
        let parts: Vec<&str> = if url.starts_with("https://github.com/") {
            url.strip_prefix("https://github.com/")
                .ok_or_else(|| BrainError::InvalidInput("Invalid GitHub URL".to_string()))?
                .split('/')
                .collect()
        } else if url.starts_with("github.com/") {
            url.strip_prefix("github.com/")
                .ok_or_else(|| BrainError::InvalidInput("Invalid GitHub URL".to_string()))?
                .split('/')
                .collect()
        } else if url.contains('/') && !url.contains("://") {
            // Assume it's owner/repo format
            url.split('/').collect()
        } else {
            return Err(BrainError::InvalidInput(
                "URL must be in format 'https://github.com/owner/repo' or 'owner/repo'".to_string()
            ));
        };

        if parts.len() < 2 {
            return Err(BrainError::InvalidInput(
                "URL must contain both owner and repository name".to_string()
            ));
        }

        Ok((parts[0].to_string(), parts[1].to_string()))
    }

    /// Fetch repository information from GitHub API
    pub async fn fetch_repository_info(&self, owner: &str, repo: &str) -> Result<RepositoryInfo> {
        let url = format!("{}/repos/{}/{}", self.base_url, owner, repo);
        
        let mut request = self.client.get(&url);
        if let Some(token) = &self.token {
            request = request.header("Authorization", format!("Bearer {}", token));
        }
        request = request.header("User-Agent", "Brain-AI/1.0");

        let response = request.send().await
            .map_err(|e| BrainError::NetworkError(format!("Failed to fetch repository: {}", e)))?;

        if !response.status().is_success() {
            return Err(BrainError::NetworkError(
                format!("GitHub API error: {} - {}", response.status(), 
                    response.text().await.unwrap_or_default())
            ));
        }

        let repo_data: serde_json::Value = response.json().await
            .map_err(|e| BrainError::ParseError(format!("Failed to parse repository data: {}", e)))?;

        // Extract basic repository information
        let name = repo_data["name"].as_str().unwrap_or("unknown").to_string();
        let full_name = repo_data["full_name"].as_str().unwrap_or("unknown").to_string();
        let description = repo_data["description"].as_str().map(|s| s.to_string());
        let language = repo_data["language"].as_str().map(|s| s.to_string());
        let stars = repo_data["stargazers_count"].as_u64().unwrap_or(0) as u32;
        let forks = repo_data["forks_count"].as_u64().unwrap_or(0) as u32;
        let size = repo_data["size"].as_u64().unwrap_or(0) as u32;
        
        let topics: Vec<String> = repo_data["topics"]
            .as_array()
            .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
            .unwrap_or_default();

        let license = repo_data["license"]["name"].as_str().map(|s| s.to_string());

        // Fetch README content
        let readme_content = self.fetch_readme(owner, repo).await.ok();

        // Create initial repository info (files will be added later)
        Ok(RepositoryInfo {
            name,
            full_name,
            description,
            language,
            topics,
            stars,
            forks,
            size,
            license,
            readme_content,
            files: Vec::new(),
        })
    }

    /// Fetch README content from repository
    async fn fetch_readme(&self, owner: &str, repo: &str) -> Result<String> {
        let readme_files = ["README.md", "README.rst", "README.txt", "README"];
        
        for readme_file in &readme_files {
            let url = format!("{}/repos/{}/{}/contents/{}", self.base_url, owner, repo, readme_file);
            
            let mut request = self.client.get(&url);
            if let Some(token) = &self.token {
                request = request.header("Authorization", format!("Bearer {}", token));
            }
            request = request.header("User-Agent", "Brain-AI/1.0");

            if let Ok(response) = request.send().await {
                if response.status().is_success() {
                    if let Ok(content_data) = response.json::<serde_json::Value>().await {
                        if let Some(content_b64) = content_data["content"].as_str() {
                            let content_bytes = general_purpose::STANDARD.decode(content_b64.replace('\n', ""))
                                .map_err(|e| BrainError::ParseError(format!("Failed to decode README: {}", e)))?;
                            return String::from_utf8(content_bytes)
                                .map_err(|e| BrainError::ParseError(format!("Invalid UTF-8 in README: {}", e)));
                        }
                    }
                }
            }
        }
        
        Err(BrainError::NotFound("README file not found".to_string()))
    }

    /// Fetch repository files based on configuration
    async fn fetch_repository_files(&self, owner: &str, repo: &str, config: &GitHubLearningConfig) -> Result<Vec<RepositoryFile>> {
        let url = format!("{}/repos/{}/{}/contents", self.base_url, owner, repo);
        
        let mut request = self.client.get(&url);
        if let Some(token) = &self.token {
            request = request.header("Authorization", format!("Bearer {}", token));
        }
        request = request.header("User-Agent", "Brain-AI/1.0");

        let response = request.send().await
            .map_err(|e| BrainError::NetworkError(format!("Failed to fetch repository contents: {}", e)))?;

        if !response.status().is_success() {
            return Err(BrainError::NetworkError(
                format!("GitHub API error: {}", response.status())
            ));
        }

        let contents: Vec<serde_json::Value> = response.json().await
            .map_err(|e| BrainError::ParseError(format!("Failed to parse contents: {}", e)))?;

        let mut files = Vec::new();
        let mut processed_count = 0;

        for item in contents {
            if processed_count >= config.max_files {
                break;
            }

            if let Some(file) = self.process_file_item(&item, config).await {
                files.push(file);
                processed_count += 1;
            }
        }

        Ok(files)
    }

    /// Process a single file item from GitHub API
    async fn process_file_item(&self, item: &serde_json::Value, config: &GitHubLearningConfig) -> Option<RepositoryFile> {
        let file_type = item["type"].as_str()?;
        if file_type != "file" {
            return None;
        }

        let path = item["path"].as_str()?.to_string();
        let name = item["name"].as_str()?.to_string();
        let size = item["size"].as_u64()? as usize;

        // Skip files that are too large
        if size > config.max_file_size {
            return None;
        }

        let file_type_enum = Self::determine_file_type(&path, &name);
        
        // Check if we should include this file type
        match file_type_enum {
            FileType::Code if !config.include_code => return None,
            FileType::Documentation if !config.include_docs => return None,
            FileType::Configuration if !config.include_config => return None,
            _ => {}
        }

        let download_url = item["download_url"].as_str()?;
        
        if let Ok(content) = self.fetch_file_content(download_url).await {
            let language = Self::detect_language(&path, &name);
            
            Some(RepositoryFile {
                path,
                name,
                content,
                file_type: file_type_enum,
                size,
                language,
            })
        } else {
            None
        }
    }

    /// Fetch content of a specific file
    async fn fetch_file_content(&self, download_url: &str) -> Result<String> {
        let mut request = self.client.get(download_url);
        if let Some(token) = &self.token {
            request = request.header("Authorization", format!("Bearer {}", token));
        }
        request = request.header("User-Agent", "Brain-AI/1.0");

        let response = request.send().await
            .map_err(|e| BrainError::NetworkError(format!("Failed to fetch file content: {}", e)))?;

        let content = response.text().await
            .map_err(|e| BrainError::NetworkError(format!("Failed to read file content: {}", e)))?;

        Ok(content)
    }

    /// Determine file type based on path and name
    fn determine_file_type(path: &str, name: &str) -> FileType {
        let lower_name = name.to_lowercase();
        let lower_path = path.to_lowercase();

        // Documentation files
        if lower_name.starts_with("readme") 
            || lower_name.ends_with(".md") 
            || lower_name.ends_with(".rst") 
            || lower_name.ends_with(".txt")
            || lower_path.contains("/docs/")
            || lower_path.contains("/doc/")
            || lower_path.contains("documentation") {
            return FileType::Documentation;
        }

        // Configuration files
        if lower_name.ends_with(".json")
            || lower_name.ends_with(".yaml") 
            || lower_name.ends_with(".yml")
            || lower_name.ends_with(".toml")
            || lower_name.ends_with(".ini")
            || lower_name.ends_with(".cfg")
            || lower_name.ends_with(".conf")
            || lower_name == "dockerfile"
            || lower_name == "makefile"
            || lower_name == "cargo.toml"
            || lower_name == "package.json"
            || lower_name == "requirements.txt"
            || lower_name == "pom.xml" {
            return FileType::Configuration;
        }

        // Code files
        if lower_name.ends_with(".rs")
            || lower_name.ends_with(".py")
            || lower_name.ends_with(".js")
            || lower_name.ends_with(".ts")
            || lower_name.ends_with(".java")
            || lower_name.ends_with(".cpp")
            || lower_name.ends_with(".c")
            || lower_name.ends_with(".h")
            || lower_name.ends_with(".go")
            || lower_name.ends_with(".php")
            || lower_name.ends_with(".rb")
            || lower_name.ends_with(".swift")
            || lower_name.ends_with(".kt") {
            return FileType::Code;
        }

        // Data files
        if lower_name.ends_with(".xml")
            || lower_name.ends_with(".csv")
            || lower_name.ends_with(".sql") {
            return FileType::Data;
        }

        FileType::Other
    }

    /// Detect programming language from file extension
    fn detect_language(_path: &str, name: &str) -> Option<String> {
        let lower_name = name.to_lowercase();
        
        if lower_name.ends_with(".rs") { Some("Rust".to_string()) }
        else if lower_name.ends_with(".py") { Some("Python".to_string()) }
        else if lower_name.ends_with(".js") { Some("JavaScript".to_string()) }
        else if lower_name.ends_with(".ts") { Some("TypeScript".to_string()) }
        else if lower_name.ends_with(".java") { Some("Java".to_string()) }
        else if lower_name.ends_with(".cpp") || lower_name.ends_with(".cc") { Some("C++".to_string()) }
        else if lower_name.ends_with(".c") { Some("C".to_string()) }
        else if lower_name.ends_with(".go") { Some("Go".to_string()) }
        else if lower_name.ends_with(".php") { Some("PHP".to_string()) }
        else if lower_name.ends_with(".rb") { Some("Ruby".to_string()) }
        else if lower_name.ends_with(".swift") { Some("Swift".to_string()) }
        else if lower_name.ends_with(".kt") { Some("Kotlin".to_string()) }
        else { None }
    }
}

/// GitHub learning engine that orchestrates the learning process
pub struct GitHubLearningEngine {
    client: GitHubClient,
    config: GitHubLearningConfig,
}

impl GitHubLearningEngine {
    /// Create a new GitHub learning engine
    pub fn new(github_token: Option<String>, config: Option<GitHubLearningConfig>) -> Self {
        Self {
            client: GitHubClient::new(github_token),
            config: config.unwrap_or_default(),
        }
    }

    /// Learn from a GitHub repository
    pub async fn learn_from_repository<T: WorkingMemoryRepository>(
        &self,
        memory_repository: &mut T,
        github_url: &str,
    ) -> Result<GitHubLearningResult> {
        let start_time = Instant::now();
        
        // Parse the GitHub URL
        let (owner, repo) = GitHubClient::parse_github_url(github_url)?;
        
        // Fetch repository information
        let mut repo_info = self.client.fetch_repository_info(&owner, &repo).await?;
        
        // Fetch repository files
        let files = self.client.fetch_repository_files(&owner, &repo, &self.config).await?;
        repo_info.files = files;
        
        // Learn from repository content
        let mut memory_entries_created = 0;
        let mut total_content_size = 0;
        let mut concepts_discovered = 0;

        // Learn from repository overview
        let repo_summary = self.create_repository_summary(&repo_info);
        let repo_item = WorkingMemoryItem::new(repo_summary.clone(), Priority::High);
        memory_repository.store_item(repo_item).await?;
        memory_entries_created += 1;

        // Learn from individual files
        for file in &repo_info.files {
            let file_content = self.create_file_learning_content(&repo_info, file);
            let priority = self.config.priority_by_type.get(&file.file_type).copied().unwrap_or(Priority::Medium);
            
            let file_item = WorkingMemoryItem::new(file_content, priority);
            memory_repository.store_item(file_item).await?;
            memory_entries_created += 1;
            total_content_size += file.size;
            concepts_discovered += self.extract_code_concepts(&file.content);
        }

        let learning_time_ms = start_time.elapsed().as_millis() as u64;
        let key_insights = self.generate_key_insights(&repo_info);

        Ok(GitHubLearningResult {
            repository: repo_info.full_name.clone(),
            files_processed: repo_info.files.len(),
            total_content_size,
            learning_time_ms,
            concepts_discovered,
            memory_entries_created,
            summary: repo_summary,
            key_insights,
        })
    }

    /// Create a summary of the repository for learning
    fn create_repository_summary(&self, repo_info: &RepositoryInfo) -> String {
        let mut summary = format!("Repository: {}\n", repo_info.full_name);
        
        if let Some(description) = &repo_info.description {
            summary.push_str(&format!("Description: {}\n", description));
        }
        
        if let Some(language) = &repo_info.language {
            summary.push_str(&format!("Primary Language: {}\n", language));
        }
        
        summary.push_str(&format!("Stars: {}, Forks: {}\n", repo_info.stars, repo_info.forks));
        
        if !repo_info.topics.is_empty() {
            summary.push_str(&format!("Topics: {}\n", repo_info.topics.join(", ")));
        }
        
        if let Some(readme) = &repo_info.readme_content {
            summary.push_str("\nREADME Content:\n");
            summary.push_str(&self.extract_key_points(readme));
        }
        
        summary
    }

    /// Create learning content for a specific file
    fn create_file_learning_content(&self, repo_info: &RepositoryInfo, file: &RepositoryFile) -> String {
        let mut content = format!("File: {} ({})\n", file.path, repo_info.full_name);
        content.push_str(&format!("Type: {:?}\n", file.file_type));
        
        if let Some(language) = &file.language {
            content.push_str(&format!("Language: {}\n", language));
        }
        
        content.push_str(&format!("Size: {} bytes\n\n", file.size));
        
        // Add processed content based on file type
        match file.file_type {
            FileType::Documentation => {
                content.push_str("Documentation Content:\n");
                content.push_str(&self.extract_key_points(&file.content));
            },
            FileType::Code => {
                content.push_str("Code Analysis:\n");
                content.push_str(&self.extract_key_points(&file.content));
            },
            FileType::Configuration => {
                content.push_str("Configuration:\n");
                content.push_str(&self.extract_key_points(&file.content));
            },
            _ => {
                content.push_str("Content:\n");
                content.push_str(&self.extract_key_points(&file.content));
            }
        }
        
        content
    }

    /// Extract key points from content for learning
    fn extract_key_points(&self, content: &str) -> String {
        let lines: Vec<&str> = content.lines().collect();
        let mut key_points = Vec::new();
        
        // Extract meaningful lines (non-empty, not just whitespace/comments)
        for line in lines.iter().take(50) { // Limit to first 50 lines
            let trimmed = line.trim();
            if !trimmed.is_empty() 
                && !trimmed.starts_with("//") 
                && !trimmed.starts_with('#') 
                && !trimmed.starts_with("/*")
                && trimmed.len() > 10 {
                key_points.push(trimmed);
            }
        }
        
        // If we have too many points, take a sample
        if key_points.len() > 20 {
            let step_size = key_points.len() / 20;
            key_points = key_points.into_iter().step_by(step_size).collect();
        }
        
        key_points.join("\n")
    }

    /// Extract code concepts from content
    fn extract_code_concepts(&self, content: &str) -> usize {
        let mut concepts = 0;
        let lines: Vec<&str> = content.lines().collect();
        
        for line in lines {
            let trimmed = line.trim();
            
            // Count function definitions
            if trimmed.contains("fn ") || trimmed.contains("function ") || trimmed.contains("def ") {
                concepts += 1;
            }
            
            // Count class/struct definitions
            if trimmed.contains("class ") || trimmed.contains("struct ") || trimmed.contains("interface ") {
                concepts += 1;
            }
            
            // Count imports/includes
            if trimmed.starts_with("import ") || trimmed.starts_with("use ") || trimmed.starts_with("#include") {
                concepts += 1;
            }
        }
        
        concepts
    }

    /// Generate key insights about the repository
    fn generate_key_insights(&self, repo_info: &RepositoryInfo) -> Vec<String> {
        let mut insights = Vec::new();
        
        // Language insights
        if let Some(language) = &repo_info.language {
            insights.push(format!("Primary programming language: {}", language));
        }
        
        // Popularity insights
        if repo_info.stars > 1000 {
            insights.push("High-popularity repository with significant community interest".to_string());
        } else if repo_info.stars > 100 {
            insights.push("Moderately popular repository".to_string());
        }
        
        // Size insights
        if repo_info.size > 10000 {
            insights.push("Large codebase indicating complex project".to_string());
        }
        
        // Topic insights
        if !repo_info.topics.is_empty() {
            insights.push(format!("Project domains: {}", repo_info.topics.join(", ")));
        }
        
        // File type distribution
        let mut file_types: HashMap<FileType, usize> = HashMap::new();
        for file in &repo_info.files {
            *file_types.entry(file.file_type.clone()).or_insert(0) += 1;
        }
        
        if let Some(code_count) = file_types.get(&FileType::Code) {
            insights.push(format!("Contains {} code files", code_count));
        }
        
        if let Some(doc_count) = file_types.get(&FileType::Documentation) {
            insights.push(format!("Contains {} documentation files", doc_count));
        }
        
        insights
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_github_url_parsing() {
        assert_eq!(
            GitHubClient::parse_github_url("https://github.com/owner/repo").unwrap(),
            ("owner".to_string(), "repo".to_string())
        );
        
        assert_eq!(
            GitHubClient::parse_github_url("owner/repo").unwrap(),
            ("owner".to_string(), "repo".to_string())
        );
        
        assert!(GitHubClient::parse_github_url("invalid").is_err());
    }

    #[test]
    fn test_file_type_detection() {
        assert_eq!(GitHubClient::determine_file_type("src/main.rs", "main.rs"), FileType::Code);
        assert_eq!(GitHubClient::determine_file_type("README.md", "README.md"), FileType::Documentation);
        assert_eq!(GitHubClient::determine_file_type("Cargo.toml", "Cargo.toml"), FileType::Configuration);
        assert_eq!(GitHubClient::determine_file_type("data.json", "data.json"), FileType::Configuration);
    }

    #[test]
    fn test_language_detection() {
        assert_eq!(GitHubClient::detect_language("", "main.rs"), Some("Rust".to_string()));
        assert_eq!(GitHubClient::detect_language("", "script.py"), Some("Python".to_string()));
        assert_eq!(GitHubClient::detect_language("", "app.js"), Some("JavaScript".to_string()));
        assert_eq!(GitHubClient::detect_language("", "unknown.xyz"), None);
    }

    #[test]
    fn test_github_learning_config_default() {
        let config = GitHubLearningConfig::default();
        assert_eq!(config.max_files, 100);
        assert_eq!(config.max_file_size, 100_000);
        assert!(config.include_code);
        assert!(config.include_docs);
        assert!(config.include_config);
    }

    #[test]
    fn test_github_learning_engine_creation() {
        let engine = GitHubLearningEngine::new(None, None);
        assert_eq!(engine.config.max_files, 100);
    }
} 