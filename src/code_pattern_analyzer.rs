use regex::Regex;
use std::collections::HashMap;
use crate::web_server::{CodePattern, CodePatternType, PatternAnalysisDepth};

/// Analysis result containing patterns and metadata
#[derive(Debug)]
pub struct PatternAnalysisResult {
    pub patterns: Vec<CodePattern>,
    pub overall_confidence: f64,
    pub architectural_insights: Vec<String>,
}

/// Configuration for pattern analysis
#[derive(Debug, Clone)]
pub struct PatternAnalysisConfig {
    pub min_confidence: f64,
    pub include_snippets: bool,
    pub max_snippet_length: usize,
}

impl Default for PatternAnalysisConfig {
    fn default() -> Self {
        Self {
            min_confidence: 0.3,
            include_snippets: true,
            max_snippet_length: 200,
        }
    }
}

/// Code pattern analyzer with multi-language support
pub struct CodePatternAnalyzer {
    config: PatternAnalysisConfig,
    language_patterns: HashMap<String, LanguagePatterns>,
}

/// Language-specific regex patterns for code analysis
#[derive(Debug, Clone)]
struct LanguagePatterns {
    functions: Vec<Regex>,
    classes: Vec<Regex>,
    imports: Vec<Regex>,
    api_endpoints: Vec<Regex>,
    error_handling: Vec<Regex>,
    test_patterns: Vec<Regex>,
}

impl CodePatternAnalyzer {
    /// Create a new code pattern analyzer
    pub fn new() -> Self {
        let mut analyzer = Self {
            config: PatternAnalysisConfig::default(),
            language_patterns: HashMap::new(),
        };
        analyzer.initialize_language_patterns();
        analyzer
    }

    /// Create analyzer with custom configuration
    pub fn with_config(config: PatternAnalysisConfig) -> Self {
        let mut analyzer = Self {
            config,
            language_patterns: HashMap::new(),
        };
        analyzer.initialize_language_patterns();
        analyzer
    }

    /// Initialize regex patterns for different programming languages
    fn initialize_language_patterns(&mut self) {
        // Rust patterns
        let rust_patterns = LanguagePatterns {
            functions: vec![
                Regex::new(r"(?m)^(?:pub\s+)?(?:async\s+)?fn\s+(\w+)").unwrap(),
                Regex::new(r"(?m)^(?:pub\s+)?(?:unsafe\s+)?fn\s+(\w+)").unwrap(),
            ],
            classes: vec![
                Regex::new(r"(?m)^(?:pub\s+)?struct\s+(\w+)").unwrap(),
                Regex::new(r"(?m)^(?:pub\s+)?enum\s+(\w+)").unwrap(),
                Regex::new(r"(?m)^(?:pub\s+)?trait\s+(\w+)").unwrap(),
            ],
            imports: vec![
                Regex::new(r"use\s+([\w:]+)").unwrap(),
                Regex::new(r"extern\s+crate\s+(\w+)").unwrap(),
            ],
            api_endpoints: vec![
                Regex::new(r#"\.route\(\s*"([^"]+)""#).unwrap(),
                Regex::new(r#"path\(\s*"([^"]+)""#).unwrap(),
            ],
            error_handling: vec![
                Regex::new(r"Result<([^,>]+),\s*([^>]+)>").unwrap(),
                Regex::new(r"Option<([^>]+)>").unwrap(),
                Regex::new(r"\.map_err\(").unwrap(),
            ],
            test_patterns: vec![
                Regex::new(r"#\[test\]").unwrap(),
                Regex::new(r"#\[cfg\(test\)\]").unwrap(),
                Regex::new(r"assert_eq!").unwrap(),
            ],
        };
        self.language_patterns.insert("rust".to_string(), rust_patterns);

        // JavaScript/TypeScript patterns
        let js_patterns = LanguagePatterns {
            functions: vec![
                Regex::new(r"(?m)^(?:export\s+)?(?:async\s+)?function\s+(\w+)").unwrap(),
                Regex::new(r"(?m)(\w+)\s*[:=]\s*(?:async\s+)?\([^)]*\)\s*=>").unwrap(),
                Regex::new(r"(?m)(\w+)\s*\([^)]*\)\s*\{").unwrap(),
            ],
            classes: vec![
                Regex::new(r"(?m)^(?:export\s+)?class\s+(\w+)").unwrap(),
                Regex::new(r"(?m)^(?:export\s+)?interface\s+(\w+)").unwrap(),
                Regex::new(r"(?m)^(?:export\s+)?type\s+(\w+)").unwrap(),
            ],
            imports: vec![
                Regex::new(r#"import\s+.*\s+from\s+['"]([^'"]+)['"]"#).unwrap(),
                Regex::new(r#"require\(['"]([^'"]+)['"]\)"#).unwrap(),
            ],
            api_endpoints: vec![
                Regex::new(r#"app\.(?:get|post|put|delete|patch)\(\s*['"]([^'"]+)['"]"#).unwrap(),
                Regex::new(r#"router\.(?:get|post|put|delete|patch)\(\s*['"]([^'"]+)['"]"#).unwrap(),
            ],
            error_handling: vec![
                Regex::new(r"try\s*\{").unwrap(),
                Regex::new(r"catch\s*\([^)]*\)").unwrap(),
                Regex::new(r"\.catch\(").unwrap(),
            ],
            test_patterns: vec![
                Regex::new(r#"(?:describe|it|test)\(\s*['"]([^'"]+)['"]"#).unwrap(),
                Regex::new(r"expect\(").unwrap(),
                Regex::new(r"assert\.").unwrap(),
            ],
        };
        self.language_patterns.insert("javascript".to_string(), js_patterns.clone());
        self.language_patterns.insert("typescript".to_string(), js_patterns);

        // Python patterns
        let python_patterns = LanguagePatterns {
            functions: vec![
                Regex::new(r"(?m)^(?:async\s+)?def\s+(\w+)").unwrap(),
                Regex::new(r"(?m)^(\w+)\s*=\s*lambda").unwrap(),
            ],
            classes: vec![
                Regex::new(r"(?m)^class\s+(\w+)").unwrap(),
                Regex::new(r"(?m)^@dataclass").unwrap(),
            ],
            imports: vec![
                Regex::new(r"^import\s+([\w.]+)").unwrap(),
                Regex::new(r"^from\s+([\w.]+)\s+import").unwrap(),
            ],
            api_endpoints: vec![
                Regex::new(r#"@app\.route\(\s*['"]([^'"]+)['"]"#).unwrap(),
                Regex::new(r#"@[\w.]*\.(?:get|post|put|delete|patch)\(\s*['"]([^'"]+)['"]"#).unwrap(),
            ],
            error_handling: vec![
                Regex::new(r"try:").unwrap(),
                Regex::new(r"except\s+(\w+)").unwrap(),
                Regex::new(r"raise\s+(\w+)").unwrap(),
            ],
            test_patterns: vec![
                Regex::new(r"def\s+test_(\w+)").unwrap(),
                Regex::new(r"assert\s+").unwrap(),
                Regex::new(r"@pytest\.").unwrap(),
            ],
        };
        self.language_patterns.insert("python".to_string(), python_patterns);

        // Java patterns
        let java_patterns = LanguagePatterns {
            functions: vec![
                Regex::new(r"(?m)^\s*(?:public|private|protected)?\s*(?:static\s+)?(?:final\s+)?[\w<>\[\]]+\s+(\w+)\s*\(").unwrap(),
            ],
            classes: vec![
                Regex::new(r"(?m)^(?:public\s+)?(?:abstract\s+)?class\s+(\w+)").unwrap(),
                Regex::new(r"(?m)^(?:public\s+)?interface\s+(\w+)").unwrap(),
                Regex::new(r"(?m)^(?:public\s+)?enum\s+(\w+)").unwrap(),
            ],
            imports: vec![
                Regex::new(r"import\s+([\w.]+)").unwrap(),
                Regex::new(r"package\s+([\w.]+)").unwrap(),
            ],
            api_endpoints: vec![
                Regex::new(r#"@(?:Get|Post|Put|Delete|Patch)Mapping\(\s*['"]([^'"]+)['"]"#).unwrap(),
                Regex::new(r#"@RequestMapping\(\s*value\s*=\s*['"]([^'"]+)['"]"#).unwrap(),
            ],
            error_handling: vec![
                Regex::new(r"try\s*\{").unwrap(),
                Regex::new(r"catch\s*\([^)]*\)").unwrap(),
                Regex::new(r"throws\s+(\w+)").unwrap(),
            ],
            test_patterns: vec![
                Regex::new(r"@Test").unwrap(),
                Regex::new(r"@BeforeEach").unwrap(),
                Regex::new(r"assertEquals\(").unwrap(),
            ],
        };
        self.language_patterns.insert("java".to_string(), java_patterns);
    }

    /// Detect programming language from code content and file path
    pub fn detect_language(&self, code: &str, file_path: Option<&str>) -> Option<String> {
        // First try to detect from file extension
        if let Some(path) = file_path {
            if let Some(extension) = path.split('.').last() {
                let lang = match extension.to_lowercase().as_str() {
                    "rs" => "rust",
                    "js" => "javascript",
                    "ts" => "typescript",
                    "py" => "python",
                    "java" => "java",
                    "kt" => "kotlin",
                    "go" => "go",
                    "cpp" | "cc" | "cxx" => "cpp",
                    "c" => "c",
                    "cs" => "csharp",
                    "rb" => "ruby",
                    "php" => "php",
                    _ => return None,
                };
                return Some(lang.to_string());
            }
        }

        // Fallback to content-based detection
        if code.contains("fn ") && code.contains("impl ") {
            Some("rust".to_string())
        } else if code.contains("function ") || code.contains("const ") && code.contains("=>") {
            Some("javascript".to_string())
        } else if code.contains("def ") && code.contains("import ") {
            Some("python".to_string())
        } else if code.contains("public class ") || code.contains("import java.") {
            Some("java".to_string())
        } else {
            None
        }
    }

    /// Analyze code patterns with specified depth
    pub fn analyze_patterns(
        &self,
        code: &str,
        file_path: Option<&str>,
        language: Option<&str>,
        depth: PatternAnalysisDepth,
    ) -> PatternAnalysisResult {
        let detected_language = language
            .map(|l| l.to_string())
            .or_else(|| self.detect_language(code, file_path))
            .unwrap_or_else(|| "unknown".to_string());

        let mut patterns = Vec::new();
        let mut insights = Vec::new();

        // Get language-specific patterns
        if let Some(lang_patterns) = self.language_patterns.get(&detected_language) {
            // Extract functions
            patterns.extend(self.extract_functions(code, file_path, lang_patterns, &depth));
            
            // Extract classes/structs
            patterns.extend(self.extract_classes(code, file_path, lang_patterns, &depth));
            
            // Extract imports
            patterns.extend(self.extract_imports(code, file_path, lang_patterns, &depth));
            
            // Extract API endpoints
            patterns.extend(self.extract_api_endpoints(code, file_path, lang_patterns, &depth));
            
            // Extract error handling patterns
            patterns.extend(self.extract_error_handling(code, file_path, lang_patterns, &depth));
            
            // Extract test patterns
            patterns.extend(self.extract_test_patterns(code, file_path, lang_patterns, &depth));

            // Generate architectural insights for detailed/deep analysis
            if matches!(depth, PatternAnalysisDepth::Detailed | PatternAnalysisDepth::Deep) {
                insights.extend(self.generate_architectural_insights(&patterns, &detected_language));
            }
        }

        // Filter patterns by confidence threshold
        patterns.retain(|p| p.confidence >= self.config.min_confidence);

        // Calculate overall confidence
        let overall_confidence = if patterns.is_empty() {
            0.0
        } else {
            patterns.iter().map(|p| p.confidence).sum::<f64>() / patterns.len() as f64
        };

        PatternAnalysisResult {
            patterns,
            overall_confidence,
            architectural_insights: insights,
        }
    }

    /// Extract function patterns from code
    fn extract_functions(
        &self,
        code: &str,
        file_path: Option<&str>,
        lang_patterns: &LanguagePatterns,
        depth: &PatternAnalysisDepth,
    ) -> Vec<CodePattern> {
        let mut patterns = Vec::new();

        for regex in &lang_patterns.functions {
            for cap in regex.captures_iter(code) {
                if let Some(name) = cap.get(1) {
                    let function_name = name.as_str().to_string();
                    let snippet = if self.config.include_snippets && !matches!(depth, PatternAnalysisDepth::Basic) {
                        self.extract_code_snippet(code, name.start(), self.config.max_snippet_length)
                    } else {
                        None
                    };

                    patterns.push(CodePattern {
                        pattern_type: CodePatternType::Function,
                        name: function_name.clone(),
                        description: format!("Function: {}", function_name),
                        code_snippet: snippet,
                        file_location: file_path.map(|p| p.to_string()),
                        confidence: 0.9,
                        related_patterns: Vec::new(),
                        concept_id: None,
                    });
                }
            }
        }

        patterns
    }

    /// Extract class/struct patterns from code
    fn extract_classes(
        &self,
        code: &str,
        file_path: Option<&str>,
        lang_patterns: &LanguagePatterns,
        depth: &PatternAnalysisDepth,
    ) -> Vec<CodePattern> {
        let mut patterns = Vec::new();

        for regex in &lang_patterns.classes {
            for cap in regex.captures_iter(code) {
                if let Some(name) = cap.get(1) {
                    let class_name = name.as_str().to_string();
                    let snippet = if self.config.include_snippets && !matches!(depth, PatternAnalysisDepth::Basic) {
                        self.extract_code_snippet(code, name.start(), self.config.max_snippet_length)
                    } else {
                        None
                    };

                    patterns.push(CodePattern {
                        pattern_type: CodePatternType::DataStructure,
                        name: class_name.clone(),
                        description: format!("Data structure: {}", class_name),
                        code_snippet: snippet,
                        file_location: file_path.map(|p| p.to_string()),
                        confidence: 0.9,
                        related_patterns: Vec::new(),
                        concept_id: None,
                    });
                }
            }
        }

        patterns
    }

    /// Extract import patterns from code
    fn extract_imports(
        &self,
        code: &str,
        file_path: Option<&str>,
        lang_patterns: &LanguagePatterns,
        _depth: &PatternAnalysisDepth,
    ) -> Vec<CodePattern> {
        let mut patterns = Vec::new();

        for regex in &lang_patterns.imports {
            for cap in regex.captures_iter(code) {
                if let Some(import) = cap.get(1) {
                    let import_path = import.as_str().to_string();
                    
                    patterns.push(CodePattern {
                        pattern_type: CodePatternType::ImportPattern,
                        name: import_path.clone(),
                        description: format!("Import: {}", import_path),
                        code_snippet: Some(cap.get(0).unwrap().as_str().to_string()),
                        file_location: file_path.map(|p| p.to_string()),
                        confidence: 0.8,
                        related_patterns: Vec::new(),
                        concept_id: None,
                    });
                }
            }
        }

        patterns
    }

    /// Extract API endpoint patterns from code
    fn extract_api_endpoints(
        &self,
        code: &str,
        file_path: Option<&str>,
        lang_patterns: &LanguagePatterns,
        depth: &PatternAnalysisDepth,
    ) -> Vec<CodePattern> {
        let mut patterns = Vec::new();

        for regex in &lang_patterns.api_endpoints {
            for cap in regex.captures_iter(code) {
                if let Some(endpoint) = cap.get(1) {
                    let endpoint_path = endpoint.as_str().to_string();
                    let snippet = if self.config.include_snippets && !matches!(depth, PatternAnalysisDepth::Basic) {
                        self.extract_code_snippet(code, endpoint.start(), self.config.max_snippet_length)
                    } else {
                        None
                    };

                    patterns.push(CodePattern {
                        pattern_type: CodePatternType::APIEndpoint,
                        name: endpoint_path.clone(),
                        description: format!("API endpoint: {}", endpoint_path),
                        code_snippet: snippet,
                        file_location: file_path.map(|p| p.to_string()),
                        confidence: 0.95,
                        related_patterns: Vec::new(),
                        concept_id: None,
                    });
                }
            }
        }

        patterns
    }

    /// Extract error handling patterns from code
    fn extract_error_handling(
        &self,
        code: &str,
        file_path: Option<&str>,
        lang_patterns: &LanguagePatterns,
        depth: &PatternAnalysisDepth,
    ) -> Vec<CodePattern> {
        let mut patterns = Vec::new();

        for regex in &lang_patterns.error_handling {
            for cap in regex.captures_iter(code) {
                let match_text = cap.get(0).unwrap().as_str();
                let snippet = if self.config.include_snippets && !matches!(depth, PatternAnalysisDepth::Basic) {
                    self.extract_code_snippet(code, cap.get(0).unwrap().start(), self.config.max_snippet_length)
                } else {
                    None
                };

                patterns.push(CodePattern {
                    pattern_type: CodePatternType::ErrorHandling,
                    name: format!("Error handling: {}", match_text),
                    description: "Error handling pattern".to_string(),
                    code_snippet: snippet,
                    file_location: file_path.map(|p| p.to_string()),
                    confidence: 0.7,
                    related_patterns: Vec::new(),
                    concept_id: None,
                });
            }
        }

        patterns
    }

    /// Extract test patterns from code
    fn extract_test_patterns(
        &self,
        code: &str,
        file_path: Option<&str>,
        lang_patterns: &LanguagePatterns,
        depth: &PatternAnalysisDepth,
    ) -> Vec<CodePattern> {
        let mut patterns = Vec::new();

        for regex in &lang_patterns.test_patterns {
            for cap in regex.captures_iter(code) {
                let match_text = cap.get(0).unwrap().as_str();
                let snippet = if self.config.include_snippets && !matches!(depth, PatternAnalysisDepth::Basic) {
                    self.extract_code_snippet(code, cap.get(0).unwrap().start(), self.config.max_snippet_length)
                } else {
                    None
                };

                patterns.push(CodePattern {
                    pattern_type: CodePatternType::TestPattern,
                    name: format!("Test: {}", match_text),
                    description: "Test pattern".to_string(),
                    code_snippet: snippet,
                    file_location: file_path.map(|p| p.to_string()),
                    confidence: 0.8,
                    related_patterns: Vec::new(),
                    concept_id: None,
                });
            }
        }

        patterns
    }

    /// Extract a code snippet around a specific position
    fn extract_code_snippet(&self, code: &str, position: usize, max_length: usize) -> Option<String> {
        let lines: Vec<&str> = code.lines().collect();
        let mut char_count = 0;
        let mut start_line = 0;

        // Find the line containing the position
        for (i, line) in lines.iter().enumerate() {
            if char_count + line.len() >= position {
                start_line = i;
                break;
            }
            char_count += line.len() + 1; // +1 for newline
        }

        // Extract a few lines around the match
        let end_line = std::cmp::min(start_line + 3, lines.len());
        let snippet = lines[start_line..end_line].join("\n");

        if snippet.len() <= max_length {
            Some(snippet)
        } else {
            Some(format!("{}...", &snippet[..max_length]))
        }
    }

    /// Generate architectural insights based on detected patterns
    fn generate_architectural_insights(&self, patterns: &[CodePattern], language: &str) -> Vec<String> {
        let mut insights = Vec::new();

        // Count pattern types
        let mut function_count = 0;
        let mut class_count = 0;
        let mut api_count = 0;
        let mut test_count = 0;
        let mut error_handling_count = 0;

        for pattern in patterns {
            match pattern.pattern_type {
                CodePatternType::Function => function_count += 1,
                CodePatternType::DataStructure => class_count += 1,
                CodePatternType::APIEndpoint => api_count += 1,
                CodePatternType::TestPattern => test_count += 1,
                CodePatternType::ErrorHandling => error_handling_count += 1,
                _ => {}
            }
        }

        // Generate insights about code structure
        if class_count > 0 {
            insights.push(format!("Detected {} data structures/classes - structured programming approach", class_count));
        }

        // Generate insights based on patterns
        if api_count > 0 {
            insights.push(format!("Detected {} API endpoints - appears to be a web service/API", api_count));
        }

        if test_count > 0 {
            let test_ratio = test_count as f64 / function_count.max(1) as f64;
            if test_ratio > 0.5 {
                insights.push("High test coverage detected - good testing practices".to_string());
            } else if test_ratio > 0.2 {
                insights.push("Moderate test coverage detected".to_string());
            } else {
                insights.push("Low test coverage detected - consider adding more tests".to_string());
            }
        }

        if error_handling_count > 0 {
            let error_ratio = error_handling_count as f64 / function_count.max(1) as f64;
            if error_ratio > 0.3 {
                insights.push("Good error handling practices detected".to_string());
            } else {
                insights.push("Limited error handling detected - consider improving error handling".to_string());
            }
        }

        match language {
            "rust" => {
                insights.push("Rust codebase - memory safety and performance focused".to_string());
                if patterns.iter().any(|p| p.name.contains("async")) {
                    insights.push("Async programming patterns detected".to_string());
                }
            }
            "javascript" | "typescript" => {
                insights.push("JavaScript/TypeScript codebase - web development focused".to_string());
                if patterns.iter().any(|p| p.name.contains("Promise") || p.name.contains("async")) {
                    insights.push("Asynchronous JavaScript patterns detected".to_string());
                }
            }
            "python" => {
                insights.push("Python codebase - rapid development and versatility focused".to_string());
            }
            "java" => {
                insights.push("Java codebase - enterprise and object-oriented development".to_string());
            }
            _ => {}
        }

        insights
    }
}

impl Default for CodePatternAnalyzer {
    fn default() -> Self {
        Self::new()
    }
} 