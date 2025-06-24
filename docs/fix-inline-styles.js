#!/usr/bin/env node

/**
 * Post-build script to remove inline styles from mdBook generated HTML
 * This script processes the generated HTML files to replace inline styles with CSS classes
 * Resolves linting warnings about inline styles while preserving functionality
 */

const fs = require('fs');
const path = require('path');

const BOOK_DIR = path.join(__dirname, 'book');
const FILES_TO_PROCESS = ['print.html']; // Can add more files if needed

/**
 * Remove inline styles and replace with CSS classes
 * @param {string} content - HTML content
 * @returns {string} - Processed HTML content
 */
function removeInlineStyles(content) {
    let processedContent = content;
    let changesCount = 0;

    // Pattern 1: Exact match for the most common page break pattern
    const exactPageBreakPattern = /<div\s+style="break-before:\s*page;\s*page-break-before:\s*always;"\s*><\/div>/gi;
    processedContent = processedContent.replace(exactPageBreakPattern, '<div class="page-break-before"></div>');
    changesCount += (content.match(exactPageBreakPattern) || []).length;

    // Pattern 2: Variations with different spacing
    const spacingVariations = [
        /<div\s+style="break-before:\s*page;\s*page-break-before:\s*always"\s*><\/div>/gi,
        /<div\s+style="break-before:page;page-break-before:always;"\s*><\/div>/gi,
        /<div\s+style="break-before:page;page-break-before:always"\s*><\/div>/gi,
        /<div\s+style="break-before:\s*page\s*;\s*page-break-before:\s*always\s*;"\s*><\/div>/gi
    ];

    spacingVariations.forEach(pattern => {
        const matches = content.match(pattern) || [];
        processedContent = processedContent.replace(pattern, '<div class="page-break-before"></div>');
        changesCount += matches.length;
    });

    // Pattern 3: More flexible pattern for any div with page break styles
    const flexibleBreakPattern = /<div\s+style="[^"]*(?:break-before:\s*page|page-break-before:\s*always)[^"]*"\s*><\/div>/gi;
    processedContent = processedContent.replace(flexibleBreakPattern, (match) => {
        // Only count if not already replaced
        if (!match.includes('class="page-break-before"')) {
            changesCount++;
            return '<div class="page-break-before"></div>';
        }
        return match;
    });

    // Pattern 4: Handle any remaining inline styles with page breaks (not just divs)
    const anyElementPattern = /<(\w+)([^>]*)\s+style="([^"]*(?:break-before:\s*page|page-break-before:\s*always)[^"]*)"([^>]*)>/gi;
    processedContent = processedContent.replace(anyElementPattern, (match, tagName, beforeStyle, styleContent, afterStyle) => {
        changesCount++;
        
        // Remove page break styles from the style attribute
        let cleanStyle = styleContent
            .replace(/break-before:\s*page\s*;?\s*/gi, '')
            .replace(/page-break-before:\s*always\s*;?\s*/gi, '')
            .replace(/^\s*;\s*/, '') // Remove leading semicolon
            .replace(/\s*;\s*$/, '') // Remove trailing semicolon
            .replace(/;\s*;+/g, ';') // Remove double semicolons
            .trim();

        // Build the replacement tag
        let result = `<${tagName}${beforeStyle}`;
        
        // Add CSS class
        if (beforeStyle.includes('class=')) {
            result = result.replace(/class="([^"]*)"/, 'class="$1 page-break-before"');
        } else {
            result += ' class="page-break-before"';
        }

        // Add cleaned style attribute only if there are remaining styles
        if (cleanStyle) {
            result += ` style="${cleanStyle}"`;
        }

        result += afterStyle + '>';
        return result;
    });

    // Pattern 5: Clean up any remaining style attributes that only contain whitespace or empty values
    const emptyStylePattern = /\s+style="[\s;]*"/gi;
    processedContent = processedContent.replace(emptyStylePattern, '');

    return { content: processedContent, changes: changesCount };
}

/**
 * Process a single HTML file
 * @param {string} filePath - Path to the HTML file
 */
function processFile(filePath) {
    try {
        const content = fs.readFileSync(filePath, 'utf8');
        const { content: processedContent, changes } = removeInlineStyles(content);
        
        if (changes > 0) {
            fs.writeFileSync(filePath, processedContent, 'utf8');
            console.log(`✅ Processed ${path.basename(filePath)}: Removed ${changes} inline styles`);
        } else {
            console.log(`ℹ️  ${path.basename(filePath)}: No inline styles found`);
        }
    } catch (error) {
        console.error(`❌ Error processing ${filePath}:`, error.message);
    }
}

/**
 * Main execution function
 */
function main() {
    console.log('🔧 Starting inline styles removal...');
    
    if (!fs.existsSync(BOOK_DIR)) {
        console.error(`❌ Book directory not found: ${BOOK_DIR}`);
        console.log('💡 Run "mdbook build" first to generate the book');
        process.exit(1);
    }

    let totalProcessed = 0;
    
    FILES_TO_PROCESS.forEach(fileName => {
        const filePath = path.join(BOOK_DIR, fileName);
        
        if (fs.existsSync(filePath)) {
            processFile(filePath);
            totalProcessed++;
        } else {
            console.log(`⚠️  File not found: ${fileName}`);
        }
    });

    console.log(`\n🎉 Complete! Processed ${totalProcessed} file(s)`);
    console.log('📋 Inline styles have been replaced with CSS classes');
    console.log('✨ Your documentation should now pass inline style linting checks');
}

// Run the script
main(); 