//! GitHub Integration Module
//! 
//! This module provides functionality to learn from GitHub repositories by:
//! - Fetching repository content via GitHub API
//! - Processing different file types (code, docs, README)
//! - Extracting meaningful information for learning
//! - Understanding repository structure and relationships

use crate::{Result, BrainError};
use crate::memory::{MemorySystem, Priority};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
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
        let mut repo_info = RepositoryInfo {
            name: repo_data["name"].as_str().unwrap_or(repo).to_string(),
            full_name: repo_data["full_name"].as_str().unwrap_or(&format!("{}/{}", owner, repo)).to_string(),
            description: repo_data["description"].as_str().map(|s| s.to_string()),
            language: repo_data["language"].as_str().map(|s| s.to_string()),
            topics: repo_data["topics"].as_array()
                .unwrap_or(&Vec::new())
                .iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect(),
            stars: repo_data["stargazers_count"].as_u64().unwrap_or(0) as u32,
            forks: repo_data["forks_count"].as_u64().unwrap_or(0) as u32,
            size: repo_data["size"].as_u64().unwrap_or(0) as u32,
            license: repo_data["license"]["name"].as_str().map(|s| s.to_string()),
            readme_content: None,
            files: Vec::new(),
        };

        // Fetch README content
        repo_info.readme_content = self.fetch_readme(owner, repo).await.ok();

        // Fetch repository files
        repo_info.files = self.fetch_repository_files(owner, repo, &GitHubLearningConfig::default()).await?;

        Ok(repo_info)
    }

    /// Fetch README content
    async fn fetch_readme(&self, owner: &str, repo: &str) -> Result<String> {
        let url = format!("{}/repos/{}/{}/readme", self.base_url, owner, repo);
        
        let mut request = self.client.get(&url);
        if let Some(token) = &self.token {
            request = request.header("Authorization", format!("Bearer {}", token));
        }
        request = request.header("User-Agent", "Brain-AI/1.0");

        let response = request.send().await
            .map_err(|e| BrainError::NetworkError(format!("Failed to fetch README: {}", e)))?;

        if !response.status().is_success() {
            return Err(BrainError::NetworkError("README not found".to_string()));
        }

        let readme_data: serde_json::Value = response.json().await
            .map_err(|e| BrainError::ParseError(format!("Failed to parse README data: {}", e)))?;

        let content = readme_data["content"].as_str()
            .ok_or_else(|| BrainError::ParseError("No content in README".to_string()))?;

        // Decode base64 content
        use base64::{Engine as _, engine::general_purpose};
        let decoded = general_purpose::STANDARD.decode(content.replace('\n', ""))
            .map_err(|e| BrainError::ParseError(format!("Failed to decode README: {}", e)))?;

        String::from_utf8(decoded)
            .map_err(|e| BrainError::ParseError(format!("Invalid UTF-8 in README: {}", e)))
    }

    /// Fetch repository files
    async fn fetch_repository_files(&self, owner: &str, repo: &str, config: &GitHubLearningConfig) -> Result<Vec<RepositoryFile>> {
        let mut files = Vec::new();
        let mut directories_to_process = vec![String::new()]; // Start with root directory
        let mut depth = 0;

        while let Some(current_path) = directories_to_process.pop() {
            if depth > 3 || files.len() >= config.max_files {
                break;
            }

            let url = if current_path.is_empty() {
                format!("{}/repos/{}/{}/contents", self.base_url, owner, repo)
            } else {
                format!("{}/repos/{}/{}/contents/{}", self.base_url, owner, repo, current_path)
            };

            let mut request = self.client.get(&url);
            if let Some(token) = &self.token {
                request = request.header("Authorization", format!("Bearer {}", token));
            }
            request = request.header("User-Agent", "Brain-AI/1.0");

            let response = match request.send().await {
                Ok(resp) => resp,
                Err(e) => {
                    // Skip directories we can't access
                    eprintln!("Failed to fetch directory {}: {}", current_path, e);
                    continue;
                }
            };

            if !response.status().is_success() {
                continue; // Skip directories we can't access
            }

            let contents: serde_json::Value = match response.json().await {
                Ok(contents) => contents,
                Err(e) => {
                    eprintln!("Failed to parse directory contents for {}: {}", current_path, e);
                    continue;
                }
            };

            if let Some(items) = contents.as_array() {
                for item in items {
                    if files.len() >= config.max_files {
                        break;
                    }

                    let item_type = item["type"].as_str().unwrap_or("");
                    let item_path = item["path"].as_str().unwrap_or("");

                    if item_type == "file" {
                        if let Some(file) = self.process_file_item(item, config).await {
                            files.push(file);
                        }
                    } else if item_type == "dir" && depth < 2 {
                        // Add subdirectory to process later
                        directories_to_process.push(item_path.to_string());
                    }
                }
            }

            depth += 1;
        }

        // Limit the number of files
        files.truncate(config.max_files);
        
        Ok(files)
    }

    /// Process a single file item
    async fn process_file_item(&self, item: &serde_json::Value, config: &GitHubLearningConfig) -> Option<RepositoryFile> {
        let path = item["path"].as_str()?;
        let name = item["name"].as_str()?;
        let size = item["size"].as_u64().unwrap_or(0) as usize;
        let download_url = item["download_url"].as_str()?;

        // Skip files that are too large
        if size > config.max_file_size {
            return None;
        }

        // Determine file type
        let file_type = Self::determine_file_type(path, name);

        // Check if we should include this file type
        match file_type {
            FileType::Documentation if !config.include_docs => return None,
            FileType::Code if !config.include_code => return None,
            FileType::Configuration if !config.include_config => return None,
            _ => {}
        }

        // Fetch file content
        let content = self.fetch_file_content(download_url).await.ok()?;

        Some(RepositoryFile {
            path: path.to_string(),
            name: name.to_string(),
            content,
            file_type,
            size,
            language: Self::detect_language(path, name),
        })
    }

    /// Fetch content of a single file
    async fn fetch_file_content(&self, download_url: &str) -> Result<String> {
        let response = self.client.get(download_url)
            .header("User-Agent", "Brain-AI/1.0")
            .send()
            .await
            .map_err(|e| BrainError::NetworkError(format!("Failed to fetch file content: {}", e)))?;

        if !response.status().is_success() {
            return Err(BrainError::NetworkError("Failed to download file".to_string()));
        }

        let bytes = response.bytes().await
            .map_err(|e| BrainError::NetworkError(format!("Failed to read file bytes: {}", e)))?;

        // Try to decode as UTF-8, skip binary files
        String::from_utf8(bytes.to_vec())
            .map_err(|_| BrainError::ParseError("File contains non-UTF-8 content".to_string()))
    }

    /// Determine file type based on path and name
    fn determine_file_type(path: &str, name: &str) -> FileType {
        let lower_name = name.to_lowercase();
        let lower_path = path.to_lowercase();

        // Documentation files
        if lower_name.starts_with("readme") || 
           lower_name.ends_with(".md") || 
           lower_name.ends_with(".rst") ||
           lower_name.ends_with(".txt") ||
           lower_path.contains("/docs/") ||
           lower_path.contains("/doc/") {
            return FileType::Documentation;
        }

        // Configuration files
        if lower_name.ends_with(".json") ||
           lower_name.ends_with(".yaml") ||
           lower_name.ends_with(".yml") ||
           lower_name.ends_with(".toml") ||
           lower_name.ends_with(".ini") ||
           lower_name.ends_with(".cfg") ||
           lower_name == "dockerfile" ||
           lower_name == "makefile" ||
           lower_name.starts_with(".") {
            return FileType::Configuration;
        }

        // Code files
        if lower_name.ends_with(".rs") ||
           lower_name.ends_with(".py") ||
           lower_name.ends_with(".js") ||
           lower_name.ends_with(".ts") ||
           lower_name.ends_with(".java") ||
           lower_name.ends_with(".cpp") ||
           lower_name.ends_with(".c") ||
           lower_name.ends_with(".h") ||
           lower_name.ends_with(".go") ||
           lower_name.ends_with(".rb") ||
           lower_name.ends_with(".php") ||
           lower_name.ends_with(".cs") ||
           lower_name.ends_with(".swift") ||
           lower_name.ends_with(".kt") {
            return FileType::Code;
        }

        // Data files
        if lower_name.ends_with(".xml") ||
           lower_name.ends_with(".csv") ||
           lower_name.ends_with(".sql") {
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
        else if lower_name.ends_with(".rb") { Some("Ruby".to_string()) }
        else if lower_name.ends_with(".php") { Some("PHP".to_string()) }
        else if lower_name.ends_with(".cs") { Some("C#".to_string()) }
        else if lower_name.ends_with(".swift") { Some("Swift".to_string()) }
        else if lower_name.ends_with(".kt") { Some("Kotlin".to_string()) }
        else if lower_name.ends_with(".md") { Some("Markdown".to_string()) }
        else if lower_name.ends_with(".json") { Some("JSON".to_string()) }
        else if lower_name.ends_with(".yaml") || lower_name.ends_with(".yml") { Some("YAML".to_string()) }
        else { None }
    }
}

/// GitHub learning engine that processes repositories for Brain AI
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
    pub async fn learn_from_repository(
        &self,
        memory_system: &mut MemorySystem,
        github_url: &str,
    ) -> Result<GitHubLearningResult> {
        let start_time = std::time::Instant::now();

        // Parse the GitHub URL
        let (owner, repo) = GitHubClient::parse_github_url(github_url)?;

        // Fetch repository information
        let repo_info = self.client.fetch_repository_info(&owner, &repo).await?;

        // Process the repository information
        let mut total_content_size = 0;
        let mut memory_entries_created = 0;
        let mut concepts_discovered = 0;

        // Learn from repository metadata
        let repo_summary = self.create_repository_summary(&repo_info);
        memory_system.learn(repo_summary, Priority::High)?;
        memory_entries_created += 1;

        // Learn from README if available
        if let Some(readme_content) = &repo_info.readme_content {
            let readme_summary = format!(
                "README for {}: {}",
                repo_info.full_name,
                self.extract_key_points(readme_content)
            );
            memory_system.learn(readme_summary, Priority::High)?;
            memory_entries_created += 1;
            total_content_size += readme_content.len();
        }

        // Process files
        for file in &repo_info.files {
            let priority = self.config.priority_by_type.get(&file.file_type)
                .cloned()
                .unwrap_or(Priority::Low);

            // Create learning content for the file
            let file_content = self.create_file_learning_content(&repo_info, file);
            memory_system.learn(file_content, priority)?;
            memory_entries_created += 1;
            total_content_size += file.content.len();

            // Extract concepts from code files
            if matches!(file.file_type, FileType::Code) {
                concepts_discovered += self.extract_code_concepts(&file.content);
            }
        }

        let learning_time_ms = start_time.elapsed().as_millis() as u64;

        // Generate insights and summary
        let key_insights = self.generate_key_insights(&repo_info);
        let summary = format!(
            "Learned from GitHub repository {}: {} files processed, {} concepts discovered. {}",
            repo_info.full_name,
            repo_info.files.len(),
            concepts_discovered,
            repo_info.description.as_deref().unwrap_or("No description available.")
        );

        Ok(GitHubLearningResult {
            repository: repo_info.full_name,
            files_processed: repo_info.files.len(),
            total_content_size,
            learning_time_ms,
            concepts_discovered,
            memory_entries_created,
            summary,
            key_insights,
        })
    }

    /// Create a summary of the repository
    fn create_repository_summary(&self, repo_info: &RepositoryInfo) -> String {
        format!(
            "GitHub Repository: {} - {} (Language: {}, Stars: {}, Forks: {}). Description: {}. Topics: {}. License: {}.",
            repo_info.full_name,
            repo_info.name,
            repo_info.language.as_deref().unwrap_or("Unknown"),
            repo_info.stars,
            repo_info.forks,
            repo_info.description.as_deref().unwrap_or("No description"),
            repo_info.topics.join(", "),
            repo_info.license.as_deref().unwrap_or("No license specified")
        )
    }

    /// Create learning content for a file
    fn create_file_learning_content(&self, repo_info: &RepositoryInfo, file: &RepositoryFile) -> String {
        let content_summary = if file.content.len() > 500 {
            self.extract_key_points(&file.content)
        } else {
            file.content.clone()
        };

        format!(
            "File {} in repository {}: {} ({:?}, {} bytes). Language: {}. Content: {}",
            file.path,
            repo_info.full_name,
            file.name,
            file.file_type,
            file.size,
            file.language.as_deref().unwrap_or("Unknown"),
            content_summary
        )
    }

    /// Extract key points from long text content
    fn extract_key_points(&self, content: &str) -> String {
        let lines: Vec<&str> = content.lines().collect();
        
        // Extract first few lines and any lines that look important
        let mut key_points = Vec::new();
        
        // Add first few lines
        for line in lines.iter().take(3) {
            if !line.trim().is_empty() {
                key_points.push(line.trim());
            }
        }

        // Look for lines that contain important keywords
        for line in &lines {
            let lower_line = line.to_lowercase();
            if (lower_line.contains("description") || 
                lower_line.contains("overview") ||
                lower_line.contains("purpose") ||
                lower_line.contains("features") ||
                lower_line.contains("usage") ||
                lower_line.contains("install") ||
                lower_line.contains("getting started")) && 
               !line.trim().is_empty() &&
               key_points.len() < 10 {
                key_points.push(line.trim());
            }
        }

        key_points.join(" ")
    }

    /// Extract concepts from code content
    fn extract_code_concepts(&self, content: &str) -> usize {
        let mut concepts = 0;
        let lines: Vec<&str> = content.lines().collect();

        for line in lines {
            let lower_line = line.to_lowercase();
            
            // Count function definitions
            if lower_line.contains("fn ") || 
               lower_line.contains("function ") ||
               lower_line.contains("def ") ||
               lower_line.contains("class ") ||
               lower_line.contains("struct ") ||
               lower_line.contains("enum ") ||
               lower_line.contains("interface ") {
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
        if repo_info.stars > 100 {
            insights.push(format!("Popular repository with {} stars", repo_info.stars));
        }

        // Topic insights
        if !repo_info.topics.is_empty() {
            insights.push(format!("Topics: {}", repo_info.topics.join(", ")));
        }

        // File type distribution
        let mut file_type_counts = HashMap::new();
        for file in &repo_info.files {
            *file_type_counts.entry(&file.file_type).or_insert(0) += 1;
        }

        for (file_type, count) in file_type_counts {
            insights.push(format!("{:?} files: {}", file_type, count));
        }

        insights
    }
}

// Add reqwest and base64 dependencies to Cargo.toml 