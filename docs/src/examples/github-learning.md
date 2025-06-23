# GitHub Repository Learning

Brain AI can learn from GitHub repositories by analyzing their code, documentation, and structure. This feature allows you to quickly understand and query information about any public repository.

## Quick Start

```python
import brain

# Initialize Brain AI
brain_engine = brain.BrainEngine()

# Learn from a repository
result = brain_engine.learn_from_github("microsoft/vscode")

print(f"Learned from {result.repository}")
print(f"Files processed: {result.files_processed}")
print(f"Summary: {result.summary}")

# Query what was learned
memories = brain_engine.query_memory("vscode", limit=5)
for memory in memories:
    print(f"- {memory.content}")
```

## Detailed Usage

### Basic Learning

```python
import brain

brain_engine = brain.BrainEngine()

# Simple repository learning
result = brain_engine.learn_from_github(
    github_url="rust-lang/mdbook",
    max_files=50,
    include_code=True,
    include_docs=True
)

print(f"Repository: {result.repository}")
print(f"Files processed: {result.files_processed}")
print(f"Concepts discovered: {result.concepts_discovered}")
print(f"Learning time: {result.learning_time_ms}ms")
```

### With GitHub Token (Recommended)

For better rate limits and access to private repositories:

```python
import os
import brain

# Set your GitHub token as an environment variable
# export GITHUB_TOKEN="your_personal_access_token"

brain_engine = brain.BrainEngine()

result = brain_engine.learn_from_github(
    github_url="https://github.com/owner/private-repo",
    github_token=os.getenv('GITHUB_TOKEN'),
    max_files=100,
    include_code=True,
    include_docs=True
)
```

### Advanced Configuration

```python
import brain

brain_engine = brain.BrainEngine()

# Customize learning behavior
result = brain_engine.learn_from_github(
    github_url="facebook/react",
    github_token=None,  # Use public API
    max_files=75,       # Limit number of files
    include_code=True,  # Include source code
    include_docs=True   # Include documentation
)

# Display key insights
print("Key Insights:")
for insight in result.key_insights:
    print(f"  • {insight}")
```

## Querying Learned Knowledge

After learning from repositories, you can query the knowledge in multiple ways:

### Memory Queries

```python
# Query specific topics
memories = brain_engine.query_memory("React components", limit=10)
for memory in memories:
    print(f"[{memory.memory_type}] {memory.content}")
    print(f"Relevance: {memory.relevance:.3f}")
```

### Advanced Queries

```python
# More sophisticated queries
results = brain_engine.advanced_query("JavaScript framework patterns")
for result in results:
    print(f"[{result.result_type}] {result.content}")
    print(f"Score: {result.score:.3f}")
    if result.related_items:
        print(f"Related: {', '.join(result.related_items[:3])}")
```

### Related Concepts

```python
# Find related concepts
related = brain_engine.find_related_concepts(
    concept_name="React",
    max_depth=2,
    limit=5
)

for concept in related:
    print(f"• {concept.content}")
    print(f"  Score: {concept.score:.3f}")
```

## Supported Repository Formats

Brain AI accepts various GitHub URL formats:

```python
# All of these work:
brain_engine.learn_from_github("microsoft/vscode")
brain_engine.learn_from_github("https://github.com/microsoft/vscode")
brain_engine.learn_from_github("github.com/microsoft/vscode")
```

## File Types Processed

Brain AI automatically categorizes and processes different file types:

- **Documentation**: README files, .md, .rst, .txt, docs/ directories
- **Source Code**: .py, .js, .rs, .java, .cpp, .go, etc.
- **Configuration**: .json, .yaml, .toml, Dockerfile, Makefile
- **Data**: .xml, .csv, .sql files

## Best Practices

### 1. Use GitHub Tokens

```bash
# Set up a GitHub personal access token
export GITHUB_TOKEN="ghp_your_token_here"
```

Benefits:
- Higher rate limits (5000 vs 60 requests/hour)
- Access to private repositories
- Better reliability

### 2. Start Small

```python
# For large repositories, start with fewer files
result = brain_engine.learn_from_github(
    "large-org/huge-repo",
    max_files=30  # Start small, increase as needed
)
```

### 3. Focus Learning

```python
# Focus on specific content types
result = brain_engine.learn_from_github(
    "documentation-heavy/repo",
    include_code=False,  # Skip code files
    include_docs=True    # Focus on documentation
)
```

## Error Handling

```python
import brain

brain_engine = brain.BrainEngine()

try:
    result = brain_engine.learn_from_github("owner/repo")
    print(f"Success: {result.summary}")
    
except Exception as e:
    if "rate limit" in str(e).lower():
        print("Rate limited - consider using a GitHub token")
    elif "not found" in str(e).lower():
        print("Repository not found or private")
    else:
        print(f"Learning failed: {e}")
```

## Export Learned Data

After learning, you can export the knowledge:

```python
# Export as JSON graph
export_result = brain_engine.export_data("json_graph")
with open("learned_graph.json", "w") as f:
    f.write(export_result.data)

# Export concepts as CSV
csv_result = brain_engine.export_data("csv_concepts")
with open("concepts.csv", "w") as f:
    f.write(csv_result.data)
```

## Examples

### Learning from Documentation Tools

```python
# Learn about documentation generation
result = brain_engine.learn_from_github("rust-lang/mdbook")

# Query documentation-related knowledge
docs_info = brain_engine.query_memory("documentation generation")
for info in docs_info:
    print(f"• {info.content}")
```

### Learning from Web Frameworks

```python
# Learn about React
result = brain_engine.learn_from_github("facebook/react")

# Find component patterns
patterns = brain_engine.advanced_query("component patterns")
for pattern in patterns[:5]:
    print(f"Pattern: {pattern.content}")
```

### Learning from System Tools

```python
# Learn about command-line tools
result = brain_engine.learn_from_github("BurntSushi/ripgrep")

# Query about performance optimization
perf_info = brain_engine.query_memory("performance optimization")
for info in perf_info:
    print(f"Optimization: {info.content}")
```

## Limitations

- **Rate Limits**: GitHub API has rate limits (60/hour without token, 5000/hour with token)
- **File Size**: Large files (>100KB) are skipped to avoid memory issues
- **Binary Files**: Only text files are processed
- **Repository Size**: Very large repositories may take time to process

## Tips for Better Results

1. **Use descriptive queries**: Instead of "code", try "React component patterns"
2. **Combine learning**: Learn from multiple related repositories
3. **Export data**: Save learned knowledge for later analysis
4. **Iterate**: Start with broad queries, then get more specific

## Troubleshooting

### Common Issues

**"Repository not found"**
- Check the repository URL/name
- Ensure the repository is public (or use a token for private repos)

**"Rate limit exceeded"**
- Set up a GitHub personal access token
- Wait for the rate limit to reset (1 hour)

**"No results found"**
- Try broader search terms
- Learn from more repositories on the topic
- Check if the repository actually contains relevant content

### Getting Help

If you encounter issues:

1. Check your internet connection
2. Verify the repository exists and is accessible
3. Try with a smaller `max_files` value
4. Use a GitHub token for better reliability

## API Reference

### `learn_from_github(github_url, github_token=None, max_files=100, include_code=True, include_docs=True)`

**Parameters:**
- `github_url` (str): Repository URL or "owner/repo" format
- `github_token` (str, optional): GitHub personal access token
- `max_files` (int, optional): Maximum files to process (default: 100)
- `include_code` (bool, optional): Include source code files (default: True)
- `include_docs` (bool, optional): Include documentation files (default: True)

**Returns:** `PyGitHubLearningResult` with:
- `repository`: Repository name
- `files_processed`: Number of files processed
- `total_content_size`: Total content size in bytes
- `learning_time_ms`: Processing time in milliseconds
- `concepts_discovered`: Number of concepts found
- `memory_entries_created`: Number of memory entries created
- `summary`: Learning summary
- `key_insights`: List of key insights discovered 