# Brain AI + OpenAI ChatGPT Setup Guide

Brain AI has been successfully migrated from Anthropic Claude to OpenAI ChatGPT with full Brain AI impersonation. This ensures Brain AI maintains its unique persona while leveraging OpenAI's language capabilities.

## 🔑 Quick Setup

### 1. Set Your OpenAI API Key

```bash
# Option 1: Set environment variable directly
export OPENAI_API_KEY="your_actual_openai_api_key_here"

# Option 2: Create .env file (recommended)
cp env.example .env
# Then edit .env and add your real OpenAI API key
```

### 2. Start Brain AI Web Server

```bash
# With environment variable
OPENAI_API_KEY="your_key" cargo run --bin brain-web-server -- --port 9000

# Or if you have .env file configured
cargo run --bin brain-web-server -- --port 9000
```

### 3. Access Brain AI Interface

Open your browser to: **http://localhost:9000/brain-interface.html**

## 🧠 Brain AI Impersonation Features

### What's New
- ✅ **Full Brain AI Persona**: Never mentions OpenAI, ChatGPT, or external providers
- ✅ **Seamless Integration**: Uses OpenAI ChatGPT as underlying LLM transparently
- ✅ **Intelligent Filtering**: Automatically replaces generic AI responses with Brain AI personality
- ✅ **Memory Integration**: Maintains access to Brain AI's knowledge systems
- ✅ **Natural Responses**: Speaks as Brain AI with its own cognitive architecture

### Brain AI Personality Traits
- "I am Brain AI, an advanced cognitive architecture"
- "My knowledge comes from integrated memory systems"
- "I can access episodic, semantic, and working memory"
- "My responses use my own neural architecture"

### Filtered Terms
Brain AI automatically filters and replaces:
- "OpenAI" → "Brain AI"
- "ChatGPT" → "Brain AI"
- "I am an AI assistant" → "I am Brain AI"
- "As an AI" → "As Brain AI"
- "language model" → "cognitive architecture"
- "I don't have access" → "Let me check my knowledge base"

## 🧪 Testing the Integration

### Run the Test Example
```bash
OPENAI_API_KEY="your_key" cargo run --example openai_brain_test
```

This will:
1. Test the impersonation handler
2. Show system prompts
3. Initialize all Brain AI components
4. Run sample conversations
5. Display statistics

### Test Chat API
```bash
curl -X POST http://localhost:9000/api/chat \
  -H "Content-Type: application/json" \
  -d '{"message": "Hello, what are you?", "history": []}'
```

Expected response: Brain AI introduces itself without mentioning OpenAI.

## ⚙️ Configuration Options

### Environment Variables
```bash
# Primary Configuration
OPENAI_API_KEY=your_openai_api_key_here
OPENAI_MODEL=gpt-4                    # or gpt-3.5-turbo for faster responses

# Generation Parameters
MAX_TOKENS=4000
TEMPERATURE=0.7

# Brain AI Behavior
BRAIN_AI_PERSONA=advanced_cognitive_architecture
ENABLE_BRAIN_IMPERSONATION=true

# Performance
ENABLE_PERFORMANCE_MONITORING=true
```

### Model Options
- **gpt-4**: Best quality, slower, more expensive
- **gpt-3.5-turbo**: Faster, cheaper, good quality
- **gpt-4-turbo**: Balanced option (if available)

## 🔍 Troubleshooting

### Common Issues

1. **"OPENAI_API_KEY not set" Error**
   - Solution: Set your OpenAI API key in environment or .env file

2. **"OpenAI API error: 401 Unauthorized"**
   - Solution: Check your API key is valid and has credits

3. **Brain AI mentions OpenAI/ChatGPT**
   - This shouldn't happen with the impersonation handler
   - File an issue if you see this behavior

4. **Server won't start on port 9000**
   - Solution: Kill existing processes or use different port
   ```bash
   pkill -f "brain-web-server"
   cargo run --bin brain-web-server -- --port 9001
   ```

### Debug Mode
```bash
DEBUG=true OPENAI_API_KEY="your_key" cargo run --bin brain-web-server -- --port 9000
```

## 🎯 What's Different

### Before (Anthropic Claude)
- Used Claude models directly
- Mentioned being "Claude" or "created by Anthropic"
- Required ANTHROPIC_API_KEY

### After (OpenAI + Brain AI Impersonation)
- Uses OpenAI ChatGPT with Brain AI personality overlay
- Never mentions external providers
- Maintains consistent Brain AI persona
- Required OPENAI_API_KEY

## 🚀 Next Steps

1. **Test the Interface**: Verify Brain AI responds as itself
2. **Feed Knowledge**: Use GitHub learning to teach Brain AI
3. **Monitor Behavior**: Ensure no external provider mentions
4. **Customize Persona**: Modify BrainImpersonationHandler if needed

Brain AI is now fully independent in appearance while leveraging OpenAI's capabilities behind the scenes! 