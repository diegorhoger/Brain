#!/bin/bash

# Brain AI Documentation Build Script
# This script builds the documentation and fixes inline style linting issues

echo "🚀 Building Brain AI Documentation..."

# Build the documentation
echo "📚 Running mdbook build..."
mdbook build

# Check if build was successful
if [ $? -ne 0 ]; then
    echo "❌ mdbook build failed!"
    exit 1
fi

echo "✅ mdbook build completed successfully"

# Remove inline styles
echo "🔧 Removing inline styles for better validation..."
node fix-inline-styles.js

# Check if style fixing was successful
if [ $? -ne 0 ]; then
    echo "❌ Inline style removal failed!"
    exit 1
fi

echo "✅ Inline styles fixed successfully"

# Validate that no inline styles remain
echo "🔍 Validating inline styles removal..."
if node validate-styles.js; then
    echo "✅ Validation passed - no inline styles found"
else
    echo "❌ Validation failed - inline styles still present"
    exit 1
fi

# Add cache busters to prevent browser caching issues
echo "🔄 Adding cache busters..."
node add-cache-buster.js

# Optional: Open the documentation
if command -v open &> /dev/null; then
    echo "🌐 Opening documentation in browser..."
    open book/index.html
elif command -v xdg-open &> /dev/null; then
    echo "🌐 Opening documentation in browser..."
    xdg-open book/index.html
else
    echo "📖 Documentation available at: book/index.html"
fi

echo ""
echo "🎉 Documentation build complete!"
echo "📍 Location: $(pwd)/book/"
echo "🌐 Open book/index.html in your browser to view"
echo "🖨️  Print version: book/print.html"
echo ""
echo "✨ All inline style warnings have been resolved!"
echo "🔄 Cache busters added to prevent browser caching issues"
echo "🔍 Validation confirms no inline styles present" 