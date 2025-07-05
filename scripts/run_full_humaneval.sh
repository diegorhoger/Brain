#!/bin/bash

# 🏆 Brain AI - Full HumanEval Evaluation Script
# =============================================
# Runs complete 164-problem HumanEval dataset evaluation with Pass@k metrics
# Targets 75%+ Pass@1 to achieve industry leadership

set -e  # Exit on any error

echo "🏆 BRAIN AI - FULL HUMANEVAL EVALUATION"
echo "======================================"
echo ""

# Configuration
TIMESTAMP=$(date +"%Y%m%d_%H%M%S")
RESULTS_DIR="./benchmark_results_${TIMESTAMP}"
BASE_OUTPUT="brain_humaneval_full_${TIMESTAMP}"

echo "📊 Evaluation Configuration:"
echo "   • Dataset: Full 164-problem HumanEval"
echo "   • Target: 75%+ Pass@1 for industry leadership"
echo "   • Metrics: Pass@1, Pass@10, Pass@100"
echo "   • Results: ${RESULTS_DIR}/"
echo ""

# Create results directory
mkdir -p "${RESULTS_DIR}"

echo "🎯 PHASE 1: Standard Pass@1 Evaluation"
echo "====================================="
echo "Running single-sample evaluation for baseline..."
echo ""

# Phase 1: Standard evaluation (Pass@1)
./target/debug/brain benchmark \
    --full \
    --agent backend-coder \
    --strategy direct \
    --evaluation standard \
    --output "${RESULTS_DIR}/${BASE_OUTPUT}_pass1.jsonl"

echo ""
echo "✅ Phase 1 complete! Results saved to: ${RESULTS_DIR}/${BASE_OUTPUT}_pass1.jsonl"
echo ""

echo "🎯 PHASE 2: Pass@10 Evaluation"
echo "============================="
echo "Running 10-sample evaluation for improved accuracy..."
echo ""

# Phase 2: Pass@10 evaluation
./target/debug/brain benchmark \
    --full \
    --agent backend-coder \
    --strategy direct \
    --evaluation pass-at-10 \
    --output "${RESULTS_DIR}/${BASE_OUTPUT}_pass10.jsonl"

echo ""
echo "✅ Phase 2 complete! Results saved to: ${RESULTS_DIR}/${BASE_OUTPUT}_pass10.jsonl"
echo ""

echo "🎯 PHASE 3: Full Evaluation (Pass@1, Pass@10, Pass@100)"
echo "===================================================="
echo "Running comprehensive 100-sample evaluation for industry comparison..."
echo ""

# Phase 3: Full evaluation (all metrics)
./target/debug/brain benchmark \
    --full \
    --agent backend-coder \
    --strategy orchestrated \
    --evaluation full \
    --output "${RESULTS_DIR}/${BASE_OUTPUT}_full.jsonl"

echo ""
echo "✅ Phase 3 complete! Results saved to: ${RESULTS_DIR}/${BASE_OUTPUT}_full.jsonl"
echo ""

echo "🏆 EVALUATION COMPLETE!"
echo "======================="
echo ""
echo "📊 Results Summary:"
echo "   • Pass@1 baseline: ${RESULTS_DIR}/${BASE_OUTPUT}_pass1.jsonl"
echo "   • Pass@10 improved: ${RESULTS_DIR}/${BASE_OUTPUT}_pass10.jsonl"
echo "   • Full metrics:     ${RESULTS_DIR}/${BASE_OUTPUT}_full.jsonl"
echo ""
echo "🎯 Industry Comparison Targets:"
echo "   • Brain AI Target: 75%+ Pass@1 (Industry Leadership)"
echo "   • Current Leaders: Codex 72%, GPT-4 67%, Claude 65%"
echo ""
echo "📈 Next Steps:"
echo "   1. Analyze results for performance optimization"
echo "   2. Compare against industry benchmarks"
echo "   3. Document competitive positioning"
echo "   4. Publish industry-leading results"
echo ""
echo "🎉 Brain AI is ready to demonstrate industry leadership!" 