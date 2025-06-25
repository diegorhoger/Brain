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

    /// Generate key insights about the repository with detailed analysis
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

        // Detailed data structure analysis
        let data_structures = self.extract_detailed_data_structures(repo_info);
        if !data_structures.is_empty() {
            insights.push(format!("Data structures identified: {}", data_structures.len()));
            for (i, ds) in data_structures.iter().enumerate() {
                insights.push(format!("Data Structure {}: {} - {}", i + 1, ds.name, ds.description));
            }
        }

        // Detailed API endpoint analysis  
        let api_endpoints = self.extract_detailed_api_endpoints(repo_info);
        if !api_endpoints.is_empty() {
            insights.push(format!("API endpoints discovered: {}", api_endpoints.len()));
            for (i, endpoint) in api_endpoints.iter().enumerate() {
                insights.push(format!("API Endpoint {}: {} {} - {}", i + 1, endpoint.method, endpoint.path, endpoint.description));
            }
        }

        // Detailed architectural pattern analysis
        let architectural_patterns = self.extract_detailed_architectural_patterns(repo_info);
        if !architectural_patterns.is_empty() {
            insights.push(format!("Architecture patterns: {}", architectural_patterns.len()));
            for (i, pattern) in architectural_patterns.iter().enumerate() {
                insights.push(format!("Architecture Pattern {}: {} - {} (Evidence: {})", i + 1, pattern.name, pattern.description, pattern.evidence));
            }
        }

        // Detailed dependency analysis
        let dependencies = self.extract_detailed_dependencies(repo_info);
        if !dependencies.is_empty() {
            insights.push(format!("Dependencies analyzed: {}", dependencies.len()));
            for (i, dep) in dependencies.iter().take(10).enumerate() { // Limit to first 10 for brevity
                insights.push(format!("Dependency {}: {} - {}", i + 1, dep.name, dep.purpose));
            }
        }

        // File type distribution (keep this for basic stats)
        let mut file_type_counts = HashMap::new();
        for file in &repo_info.files {
            *file_type_counts.entry(&file.file_type).or_insert(0) += 1;
        }

        for (file_type, count) in file_type_counts {
            insights.push(format!("{:?} files: {}", file_type, count));
        }

        insights
    }

    /// Extract detailed data structures from repository files
    fn extract_detailed_data_structures(&self, repo_info: &RepositoryInfo) -> Vec<DetailedDataStructure> {
        let mut data_structures = Vec::new();

        for file in &repo_info.files {
            if matches!(file.file_type, FileType::Code) {
                let file_structures = self.parse_data_structures_from_code(&file.content, &file.path);
                data_structures.extend(file_structures);
            }
        }

        data_structures
    }

    /// Extract detailed API endpoints from repository files
    fn extract_detailed_api_endpoints(&self, repo_info: &RepositoryInfo) -> Vec<DetailedAPIEndpoint> {
        let mut endpoints = Vec::new();

        for file in &repo_info.files {
            if matches!(file.file_type, FileType::Code) {
                let file_endpoints = self.parse_api_endpoints_from_code(&file.content, &file.path);
                endpoints.extend(file_endpoints);
            }
        }

        // Also check README for API documentation
        if let Some(readme) = &repo_info.readme_content {
            let readme_endpoints = self.parse_api_endpoints_from_docs(readme, "README.md");
            endpoints.extend(readme_endpoints);
        }

        endpoints
    }

    /// Extract detailed architectural patterns with evidence
    fn extract_detailed_architectural_patterns(&self, repo_info: &RepositoryInfo) -> Vec<DetailedArchitecturalPattern> {
        let mut patterns = Vec::new();

        // Analyze README for architectural information
        if let Some(readme) = &repo_info.readme_content {
            let readme_patterns = self.parse_architectural_patterns_from_docs(readme, "README.md");
            patterns.extend(readme_patterns);
        }

        // Analyze code files for architectural patterns
        for file in &repo_info.files {
            if matches!(file.file_type, FileType::Code) {
                let code_patterns = self.parse_architectural_patterns_from_code(&file.content, &file.path);
                patterns.extend(code_patterns);
            }
        }

        patterns
    }

    /// Extract detailed dependencies with purposes
    fn extract_detailed_dependencies(&self, repo_info: &RepositoryInfo) -> Vec<DetailedDependency> {
        let mut dependencies = Vec::new();

        for file in &repo_info.files {
            let file_deps = match file.name.as_str() {
                "package.json" => self.parse_npm_dependencies(&file.content, &file.path),
                "Cargo.toml" => self.parse_cargo_dependencies(&file.content, &file.path),
                "requirements.txt" => self.parse_python_dependencies(&file.content, &file.path),
                "pom.xml" => self.parse_maven_dependencies(&file.content, &file.path),
                _ => Vec::new(),
            };
            dependencies.extend(file_deps);
        }

        dependencies
    }

    /// Parse data structures from code content
    fn parse_data_structures_from_code(&self, content: &str, file_path: &str) -> Vec<DetailedDataStructure> {
        let mut structures = Vec::new();
        let lines: Vec<&str> = content.lines().collect();

        for (i, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            
            // JavaScript/TypeScript structures
            if trimmed.starts_with("interface ") || trimmed.starts_with("type ") {
                if let Some(name) = self.extract_name_after_keyword(trimmed, &["interface", "type"]) {
                    let fields = self.extract_structure_fields(&lines, i);
                    structures.push(DetailedDataStructure {
                        name,
                        description: format!("TypeScript {} defined in {}", 
                                           if trimmed.starts_with("interface") { "interface" } else { "type" }, 
                                           file_path),
                        structure_type: if trimmed.starts_with("interface") { "interface".to_string() } else { "type".to_string() },
                        fields,
                        file_location: file_path.to_string(),
                    });
                }
            }
            
            // Rust structures
            else if trimmed.starts_with("struct ") || trimmed.starts_with("enum ") {
                if let Some(name) = self.extract_name_after_keyword(trimmed, &["struct", "enum"]) {
                    let fields = self.extract_structure_fields(&lines, i);
                    structures.push(DetailedDataStructure {
                        name: name.to_string(),
                        description: format!("Rust {} defined in {}", 
                                           if trimmed.starts_with("struct") { "struct" } else { "enum" }, 
                                           file_path),
                        structure_type: if trimmed.starts_with("struct") { "struct".to_string() } else { "enum".to_string() },
                        fields,
                        file_location: file_path.to_string(),
                    });
                }
            }
            
            // Python classes
            else if trimmed.starts_with("class ") {
                if let Some(name) = self.extract_name_after_keyword(trimmed, &["class"]) {
                    let fields = self.extract_python_class_fields(&lines, i);
                    structures.push(DetailedDataStructure {
                        name: name.to_string(),
                        description: format!("Python class defined in {}", file_path),
                        structure_type: "class".to_string(),
                        fields,
                        file_location: file_path.to_string(),
                    });
                }
            }
        }

        structures
    }

    /// Parse API endpoints from code content
    fn parse_api_endpoints_from_code(&self, content: &str, file_path: &str) -> Vec<DetailedAPIEndpoint> {
        let mut endpoints = Vec::new();
        let lines: Vec<&str> = content.lines().collect();

        for line in lines {
            let trimmed = line.trim();
            
            // Express.js style routes
            if let Some(captures) = self.extract_express_route(trimmed) {
                let path = captures.1.clone();
                endpoints.push(DetailedAPIEndpoint {
                    method: captures.0,
                    path: captures.1,
                    description: format!("Express.js route in {}", file_path),
                    parameters: self.extract_route_parameters(&path),
                    response_type: None,
                    file_location: file_path.to_string(),
                });
            }
            
            // FastAPI/Flask style routes
            else if let Some(captures) = self.extract_python_route(trimmed) {
                let path = captures.1.clone();
                endpoints.push(DetailedAPIEndpoint {
                    method: captures.0,
                    path: captures.1,
                    description: format!("Python API route in {}", file_path),
                    parameters: self.extract_route_parameters(&path),
                    response_type: None,
                    file_location: file_path.to_string(),
                });
            }
            
            // Spring Boot style routes
            else if let Some(captures) = self.extract_spring_route(trimmed) {
                let path = captures.1.clone();
                endpoints.push(DetailedAPIEndpoint {
                    method: captures.0,
                    path: captures.1,
                    description: format!("Spring Boot endpoint in {}", file_path),
                    parameters: self.extract_route_parameters(&path),
                    response_type: None,
                    file_location: file_path.to_string(),
                });
            }
        }

        endpoints
    }

    /// Parse API endpoints from documentation
    fn parse_api_endpoints_from_docs(&self, content: &str, file_path: &str) -> Vec<DetailedAPIEndpoint> {
        let mut endpoints = Vec::new();
        let lines: Vec<&str> = content.lines().collect();

        for line in lines {
            // Look for common API documentation patterns
            if line.contains("GET ") || line.contains("POST ") || line.contains("PUT ") || line.contains("DELETE ") {
                if let Some(captures) = self.extract_documented_endpoint(line) {
                    let path = captures.1.clone();
                    endpoints.push(DetailedAPIEndpoint {
                        method: captures.0,
                        path: captures.1,
                        description: format!("API endpoint documented in {}", file_path),
                        parameters: self.extract_route_parameters(&path),
                        response_type: None,
                        file_location: file_path.to_string(),
                    });
                }
            }
        }

        endpoints
    }

    /// Parse architectural patterns from documentation
    fn parse_architectural_patterns_from_docs(&self, content: &str, file_path: &str) -> Vec<DetailedArchitecturalPattern> {
        let mut patterns = Vec::new();
        let content_lower = content.to_lowercase();

        let pattern_definitions = vec![
            ("Microservices Architecture", "microservice", "Service-oriented architecture with independently deployable services"),
            ("REST API Architecture", "rest", "Representational State Transfer architectural style for web services"),
            ("MVC Pattern", "mvc", "Model-View-Controller architectural pattern"),
            ("Event-Driven Architecture", "event-driven", "Architecture based on event production and consumption"),
            ("Plugin Architecture", "plugin", "Extensible architecture supporting third-party extensions"),
            ("Layered Architecture", "layered", "Architecture organized into horizontal layers"),
            ("Client-Server Architecture", "client-server", "Distributed architecture with client and server components"),
            ("Serverless Architecture", "serverless", "Cloud computing execution model with stateless compute containers"),
        ];

        for (name, keyword, description) in pattern_definitions {
            if content_lower.contains(keyword) {
                patterns.push(DetailedArchitecturalPattern {
                    name: name.to_string(),
                    description: description.to_string(),
                    pattern_type: "architectural".to_string(),
                    evidence: format!("Mentioned in {} documentation", file_path),
                    implementation_details: self.extract_pattern_details(&content, keyword),
                    files_involved: vec![file_path.to_string()],
                });
            }
        }

        patterns
    }

    /// Parse architectural patterns from code
    fn parse_architectural_patterns_from_code(&self, content: &str, file_path: &str) -> Vec<DetailedArchitecturalPattern> {
        let mut patterns = Vec::new();

        // Detect common code patterns
        if content.contains("@RestController") || content.contains("@Controller") {
            patterns.push(DetailedArchitecturalPattern {
                name: "MVC Controller Pattern".to_string(),
                description: "Spring Boot MVC controller implementation".to_string(),
                pattern_type: "framework".to_string(),
                evidence: format!("Spring annotations found in {}", file_path),
                implementation_details: vec!["Uses Spring Boot annotations for REST endpoints".to_string()],
                files_involved: vec![file_path.to_string()],
            });
        }

        if content.contains("express()") || content.contains("app.get") || content.contains("app.post") {
            patterns.push(DetailedArchitecturalPattern {
                name: "Express.js Web Framework Pattern".to_string(),
                description: "Node.js web application framework implementation".to_string(),
                pattern_type: "framework".to_string(),
                evidence: format!("Express.js patterns found in {}", file_path),
                implementation_details: vec!["Uses Express.js for HTTP server and routing".to_string()],
                files_involved: vec![file_path.to_string()],
            });
        }

        patterns
    }

    /// Parse NPM dependencies
    fn parse_npm_dependencies(&self, content: &str, file_path: &str) -> Vec<DetailedDependency> {
        let mut dependencies = Vec::new();
        
        if let Ok(package_json) = serde_json::from_str::<serde_json::Value>(content) {
            if let Some(deps) = package_json.get("dependencies").and_then(|d| d.as_object()) {
                for (name, version) in deps {
                    dependencies.push(DetailedDependency {
                        name: name.clone(),
                        version: version.as_str().map(|s| s.to_string()),
                        purpose: self.infer_dependency_purpose(name),
                        dependency_type: "runtime".to_string(),
                        source_file: file_path.to_string(),
                    });
                }
            }
            
            if let Some(dev_deps) = package_json.get("devDependencies").and_then(|d| d.as_object()) {
                for (name, version) in dev_deps {
                    dependencies.push(DetailedDependency {
                        name: name.clone(),
                        version: version.as_str().map(|s| s.to_string()),
                        purpose: self.infer_dependency_purpose(name),
                        dependency_type: "development".to_string(),
                        source_file: file_path.to_string(),
                    });
                }
            }
        }

        dependencies
    }

    /// Parse Cargo dependencies (simplified)
    fn parse_cargo_dependencies(&self, content: &str, file_path: &str) -> Vec<DetailedDependency> {
        let mut dependencies = Vec::new();
        let lines: Vec<&str> = content.lines().collect();
        let mut in_dependencies = false;

        for line in lines {
            let trimmed = line.trim();
            
            if trimmed == "[dependencies]" {
                in_dependencies = true;
                continue;
            }
            
            if trimmed.starts_with('[') && trimmed != "[dependencies]" {
                in_dependencies = false;
                continue;
            }
            
            if in_dependencies && trimmed.contains('=') {
                let parts: Vec<&str> = trimmed.split('=').collect();
                if parts.len() >= 2 {
                    let name = parts[0].trim().trim_matches('"');
                    let version = parts[1].trim().trim_matches('"');
                    
                    dependencies.push(DetailedDependency {
                        name: name.to_string(),
                        version: Some(version.to_string()),
                        purpose: self.infer_dependency_purpose(name),
                        dependency_type: "runtime".to_string(),
                        source_file: file_path.to_string(),
                    });
                }
            }
        }

        dependencies
    }

    /// Parse Python dependencies (simplified)
    fn parse_python_dependencies(&self, content: &str, file_path: &str) -> Vec<DetailedDependency> {
        let mut dependencies = Vec::new();
        let lines: Vec<&str> = content.lines().collect();

        for line in lines {
            let trimmed = line.trim();
            if !trimmed.is_empty() && !trimmed.starts_with('#') {
                let name = if trimmed.contains("==") {
                    trimmed.split("==").next().unwrap_or(trimmed)
                } else if trimmed.contains(">=") {
                    trimmed.split(">=").next().unwrap_or(trimmed)
                } else {
                    trimmed
                };

                dependencies.push(DetailedDependency {
                    name: name.to_string(),
                    version: None,
                    purpose: self.infer_dependency_purpose(name),
                    dependency_type: "runtime".to_string(),
                    source_file: file_path.to_string(),
                });
            }
        }

        dependencies
    }

    /// Parse Maven dependencies (simplified)
    fn parse_maven_dependencies(&self, _content: &str, _file_path: &str) -> Vec<DetailedDependency> {
        // Simplified implementation - would need proper XML parsing for full support
        Vec::new()
    }

    // Helper methods for parsing
    fn extract_name_after_keyword(&self, line: &str, keywords: &[&str]) -> Option<String> {
        for keyword in keywords {
            if line.starts_with(keyword) {
                let after_keyword = &line[keyword.len()..].trim();
                if let Some(space_pos) = after_keyword.find(' ') {
                    return Some(after_keyword[..space_pos].to_string());
                } else if let Some(brace_pos) = after_keyword.find('{') {
                    return Some(after_keyword[..brace_pos].trim().to_string());
                } else {
                    return Some(after_keyword.to_string());
                }
            }
        }
        None
    }

    fn extract_structure_fields(&self, lines: &[&str], start_index: usize) -> Vec<String> {
        let mut fields = Vec::new();
        let mut brace_count = 0;
        let mut found_opening_brace = false;

        for line in lines.iter().skip(start_index) {
            for ch in line.chars() {
                match ch {
                    '{' => {
                        brace_count += 1;
                        found_opening_brace = true;
                    }
                    '}' => {
                        brace_count -= 1;
                        if brace_count == 0 && found_opening_brace {
                            return fields;
                        }
                    }
                    _ => {}
                }
            }

            if found_opening_brace && brace_count > 0 {
                let trimmed = line.trim();
                if !trimmed.is_empty() && !trimmed.starts_with('{') && !trimmed.starts_with('}') {
                    fields.push(trimmed.to_string());
                }
            }
        }

        fields
    }

    fn extract_python_class_fields(&self, lines: &[&str], start_index: usize) -> Vec<String> {
        let mut fields = Vec::new();
        let mut _in_class = false;

        for line in lines.iter().skip(start_index + 1) {
            let trimmed = line.trim();
            
            if trimmed.is_empty() {
                continue;
            }
            
            // If we hit another class or function at the same level, we're done
            if !line.starts_with(' ') && !line.starts_with('\t') {
                break;
            }
            
            _in_class = true;
            
            // Look for field definitions (self.field = ...)
            if trimmed.starts_with("self.") && trimmed.contains('=') {
                let field_name = trimmed.split('=').next().unwrap_or(trimmed).trim();
                fields.push(field_name.to_string());
            }
            // Look for method definitions
            else if trimmed.starts_with("def ") {
                let method_name = trimmed.split('(').next().unwrap_or(trimmed);
                fields.push(method_name.to_string());
            }
        }

        fields
    }

    fn extract_express_route(&self, line: &str) -> Option<(String, String)> {
        let methods = ["get", "post", "put", "delete", "patch"];
        
        for method in &methods {
            let pattern = format!("app.{}(", method);
            if line.contains(&pattern) {
                if let Some(start) = line.find('\'') {
                    if let Some(end) = line[start + 1..].find('\'') {
                        let path = &line[start + 1..start + 1 + end];
                        return Some((method.to_uppercase(), path.to_string()));
                    }
                }
                if let Some(start) = line.find('"') {
                    if let Some(end) = line[start + 1..].find('"') {
                        let path = &line[start + 1..start + 1 + end];
                        return Some((method.to_uppercase(), path.to_string()));
                    }
                }
            }
        }
        
        None
    }

    fn extract_python_route(&self, line: &str) -> Option<(String, String)> {
        // FastAPI style: @app.get("/path")
        if line.starts_with("@app.") {
            let methods = ["get", "post", "put", "delete", "patch"];
            for method in &methods {
                if line.contains(&format!("@app.{}(", method)) {
                    if let Some(start) = line.find('"') {
                        if let Some(end) = line[start + 1..].find('"') {
                            let path = &line[start + 1..start + 1 + end];
                            return Some((method.to_uppercase(), path.to_string()));
                        }
                    }
                }
            }
        }
        
        None
    }

    fn extract_spring_route(&self, line: &str) -> Option<(String, String)> {
        // Spring Boot style: @GetMapping("/path")
        let mappings = [
            ("@GetMapping", "GET"),
            ("@PostMapping", "POST"),
            ("@PutMapping", "PUT"),
            ("@DeleteMapping", "DELETE"),
            ("@PatchMapping", "PATCH"),
        ];
        
        for (annotation, method) in &mappings {
            if line.contains(annotation) {
                if let Some(start) = line.find('"') {
                    if let Some(end) = line[start + 1..].find('"') {
                        let path = &line[start + 1..start + 1 + end];
                        return Some((method.to_string(), path.to_string()));
                    }
                }
            }
        }
        
        None
    }

    fn extract_documented_endpoint(&self, line: &str) -> Option<(String, String)> {
        let methods = ["GET", "POST", "PUT", "DELETE", "PATCH"];
        
        for method in &methods {
            if line.contains(method) {
                // Look for path after the method
                let parts: Vec<&str> = line.split_whitespace().collect();
                for (i, part) in parts.iter().enumerate() {
                    if part == method && i + 1 < parts.len() {
                        let path = parts[i + 1].trim_matches('`').trim_matches('*');
                        return Some((method.to_string(), path.to_string()));
                    }
                }
            }
        }
        
        None
    }

    fn extract_route_parameters(&self, path: &str) -> Vec<String> {
        let mut params = Vec::new();
        
        // Extract path parameters like :id, {id}, <id>
        let patterns = [(':', ' '), ('{', '}'), ('<', '>')];
        
        for (start_char, end_char) in &patterns {
            let mut chars = path.chars().peekable();
            while let Some(ch) = chars.next() {
                if ch == *start_char {
                    let mut param = String::new();
                    while let Some(&next_ch) = chars.peek() {
                        if next_ch == *end_char || next_ch == '/' {
                            break;
                        }
                        param.push(chars.next().unwrap());
                    }
                    if !param.is_empty() {
                        params.push(param);
                    }
                }
            }
        }
        
        params
    }

    fn extract_pattern_details(&self, content: &str, keyword: &str) -> Vec<String> {
        let mut details = Vec::new();
        let lines: Vec<&str> = content.lines().collect();
        
        for line in lines {
            if line.to_lowercase().contains(keyword) {
                details.push(line.trim().to_string());
            }
        }
        
        details
    }

    fn infer_dependency_purpose(&self, name: &str) -> String {
        match name {
            // Web frameworks
            "express" => "Web application framework for Node.js".to_string(),
            "fastapi" => "Modern web framework for building APIs with Python".to_string(),
            "flask" => "Lightweight web application framework for Python".to_string(),
            "spring-boot-starter-web" => "Spring Boot web starter for building web applications".to_string(),
            
            // Databases
            "mongoose" => "MongoDB object modeling for Node.js".to_string(),
            "sequelize" => "Promise-based Node.js ORM for SQL databases".to_string(),
            "sqlalchemy" => "Python SQL toolkit and Object Relational Mapping".to_string(),
            
            // HTTP clients
            "axios" => "Promise-based HTTP client for JavaScript".to_string(),
            "requests" => "HTTP library for Python".to_string(),
            "reqwest" => "HTTP client for Rust".to_string(),
            
            // Testing
            "jest" => "JavaScript testing framework".to_string(),
            "pytest" => "Testing framework for Python".to_string(),
            "junit" => "Unit testing framework for Java".to_string(),
            
            // Build tools
            "webpack" => "Module bundler for JavaScript applications".to_string(),
            "babel" => "JavaScript compiler for modern syntax".to_string(),
            
            // Default
            _ => format!("Dependency for {}", name),
        }
    }
}

// Add reqwest and base64 dependencies to Cargo.toml 