# 🏆 Brain AI - Full HumanEval Evaluation Guide

## Overview
This guide provides instructions for running the complete 164-problem HumanEval dataset evaluation with advanced Pass@k metrics. Brain AI targets 75%+ Pass@1 to achieve industry leadership over current benchmarks.

## 🎯 Evaluation Objectives

### Industry Leadership Target
- **Target**: 75%+ Pass@1 (First attempt success rate)
- **Current Leaders**: Codex 72%, GPT-4 67%, Claude 65%
- **Brain AI Advantage**: 37-agent multi-agent architecture

### Pass@k Metrics
- **Pass@1**: Success rate with single code generation
- **Pass@10**: Success rate with best of 10 generations
- **Pass@100**: Success rate with best of 100 generations

## 🚀 Quick Start

### Prerequisites
```bash
# Build Brain AI CLI
cargo build --package brain-cli

# Verify HumanEval dataset exists
ls benchmarks/humaneval/human-eval/data/HumanEval.jsonl.gz
```

### Run Full Evaluation
```bash
# Execute complete evaluation suite
./scripts/run_full_humaneval.sh
```

## 📊 Manual Evaluation Commands

### 1. Standard Pass@1 Evaluation
```bash
# Single-sample baseline evaluation
./target/debug/brain benchmark \
    --full \
    --agent backend-coder \
    --strategy direct \
    --evaluation standard \
    --output brain_humaneval_pass1.jsonl
```

### 2. Pass@10 Evaluation
```bash
# 10-sample evaluation for improved accuracy
./target/debug/brain benchmark \
    --full \
    --agent backend-coder \
    --strategy orchestrated \
    --evaluation pass-at-10 \
    --output brain_humaneval_pass10.jsonl
```

### 3. Full Pass@k Evaluation
```bash
# Complete evaluation with all metrics
./target/debug/brain benchmark \
    --full \
    --agent backend-coder \
    --strategy quality \
    --evaluation full \
    --output brain_humaneval_full.jsonl
```

## 🎛️ Configuration Options

### Agent Selection
- **backend-coder**: Direct code generation (recommended baseline)
- **planner-agent**: Architecture-first approach
- **qa_agent**: Quality-focused generation
- **architect-agent**: System design approach

### Execution Strategies
- **direct**: Single-agent rapid execution
- **orchestrated**: Multi-agent collaboration with planning
- **quality**: Full QA pipeline with Elite Code Framework

### Evaluation Modes
- **standard**: Pass@1 only (fast baseline)
- **pass-at-10**: Pass@1 and Pass@10 metrics
- **pass-at-100**: Pass@1 and Pass@100 metrics
- **full**: All metrics (Pass@1, Pass@10, Pass@100)

## 📈 Results Analysis

### Understanding Output
```jsonl
{
  "task_id": "HumanEval/0",
  "completion": "def has_close_elements(numbers, threshold):\n    return any(abs(a - b) < threshold for i, a in enumerate(numbers) for b in numbers[i+1:])",
  "execution_time_ms": 1250,
  "confidence": 0.89,
  "success": true
}
```

### Key Metrics
- **Pass@1**: Percentage of problems solved on first attempt
- **Pass@10**: Percentage solved with best of 10 attempts
- **Pass@100**: Percentage solved with best of 100 attempts
- **Average Confidence**: Agent confidence across all problems
- **Execution Time**: Average time per problem

### Industry Comparison
```
System      | Pass@1 | Architecture      | Status
------------|--------|-------------------|------------------
Brain AI    | TBD%   | 37 Multi-Agent    | 🎯 Target: 75%+
Codex       | 72%    | Single Model      | Current Leader
GPT-4       | 67%    | Single Model      | Industry Standard
Claude      | 65%    | Single Model      | Anthropic
```

## 🔧 Optimization Strategies

### Agent-Specific Optimization
```bash
# Test different agents for problem categories
./target/debug/brain benchmark --subset 10 --agent backend-coder --evaluation standard --output test_backend.jsonl
./target/debug/brain benchmark --subset 10 --agent planner-agent --evaluation standard --output test_planner.jsonl
./target/debug/brain benchmark --subset 10 --agent qa_agent --evaluation standard --output test_qa.jsonl
```

### Strategy Comparison
```bash
# Compare execution strategies
./target/debug/brain benchmark --subset 20 --strategy direct --evaluation standard --output test_direct.jsonl
./target/debug/brain benchmark --subset 20 --strategy orchestrated --evaluation standard --output test_orchestrated.jsonl
./target/debug/brain benchmark --subset 20 --strategy quality --evaluation standard --output test_quality.jsonl
```

### Performance Tuning
1. **Agent Selection**: Analyze which agents perform best for different problem types
2. **Strategy Optimization**: Compare direct vs orchestrated vs quality approaches
3. **Confidence Calibration**: Tune confidence thresholds for optimal performance
4. **Memory Integration**: Leverage meta-memory for pattern recognition

## 📊 Expected Results Timeline

### Phase 1: Pass@1 Baseline (30-60 minutes)
- Single-sample evaluation across 164 problems
- Establishes baseline competitive position
- Target: 65%+ to match Claude performance

### Phase 2: Pass@10 Evaluation (5-10 hours)
- 10 samples per problem (1,640 total evaluations)
- Demonstrates multi-sample improvement
- Target: 80%+ with best-of-10 selection

### Phase 3: Full Evaluation (50-100 hours)
- 100 samples per problem (16,400 total evaluations)
- Complete industry comparison dataset
- Target: 85%+ Pass@100 for comprehensive leadership

## 🎯 Success Criteria

### Immediate Goals
- [ ] ✅ Pass@1 ≥ 72% (Match Codex)
- [ ] 🎯 Pass@1 ≥ 75% (Industry Leadership)
- [ ] 📈 Pass@10 ≥ 85% (Multi-sample advantage)
- [ ] 🏆 Pass@100 ≥ 90% (Comprehensive superiority)

### Long-term Impact
- Academic publication: "Multi-Agent Autonomous Code Generation"
- Industry benchmark establishment
- Open-source evaluation toolkit
- Community adoption and validation

## 🚨 Troubleshooting

### Common Issues
1. **Dataset Not Found**: Ensure `benchmarks/humaneval/human-eval/data/HumanEval.jsonl.gz` exists
2. **Agent Connection**: Verify brain-cognitive agents are accessible
3. **Memory Issues**: Large evaluations may require significant RAM
4. **Timeout Errors**: Increase timeout for complex problems

### Performance Optimization
```bash
# Quick subset test before full evaluation
./target/debug/brain benchmark --subset 5 --agent backend-coder --evaluation standard --output test.jsonl

# Monitor system resources during evaluation
htop  # Monitor CPU/memory usage
```

## 📚 Additional Resources

- [HumanEval Technical Reference](./HUMANEVAL_TECHNICAL_REFERENCE.md)
- [Brain AI Agent Documentation](../cognitive-agents.md)
- [Brain AI Architecture Guide](../README.md)
- [Cognitive Preference Profiles](../crates/brain-cognitive/src/profiles/README.md)

## 🎉 Achievement Recognition

Upon reaching 75%+ Pass@1:
- Update industry comparison charts
- Generate press release materials
- Submit academic paper
- Establish Brain AI as industry leader

**Brain AI is positioned to achieve historic industry leadership in autonomous code generation!** 