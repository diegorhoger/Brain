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

    // Pattern 1: Page break before
    const pageBreakBeforePattern = /<div\s+style="break-before:\s*page;\s*page-break-before:\s*always;"\s*><\/div>/gi;
    processedContent = processedContent.replace(pageBreakBeforePattern, '<div class="page-break-before"></div>');
    changesCount += (content.match(pageBreakBeforePattern) || []).length;

    // Pattern 2: More flexible page break patterns
    const flexibleBreakPattern = /<div\s+style="[^"]*break-before:\s*page[^"]*"[^>]*><\/div>/gi;
    processedContent = processedContent.replace(flexibleBreakPattern, (match) => {
        changesCount++;
        // Extract any other attributes
        const attributeMatch = match.match(/<div\s+([^>]*?)>/);
        if (attributeMatch) {
            const attributes = attributeMatch[1].replace(/style="[^"]*"/g, '').trim();
            return `<div class="page-break-before" ${attributes}></div>`;
        }
        return '<div class="page-break-before"></div>';
    });

    // Pattern 3: Any remaining inline styles with page breaks
    const anyPageBreakPattern = /<([^>]+)\s+style="([^"]*(?:break-before|page-break-before|break-after|page-break-after|break-inside|page-break-inside)[^"]*)"([^>]*)>/gi;
    processedContent = processedContent.replace(anyPageBreakPattern, (match, tag, styleContent, rest) => {
        changesCount++;
        
        // Remove page break styles from the style attribute
        let cleanStyle = styleContent
            .replace(/break-before:\s*[^;]+;?\s*/gi, '')
            .replace(/page-break-before:\s*[^;]+;?\s*/gi, '')
            .replace(/break-after:\s*[^;]+;?\s*/gi, '')
            .replace(/page-break-after:\s*[^;]+;?\s*/gi, '')
            .replace(/break-inside:\s*[^;]+;?\s*/gi, '')
            .replace(/page-break-inside:\s*[^;]+;?\s*/gi, '')
            .replace(/^\s*;\s*/, '') // Remove leading semicolon
            .replace(/\s*;\s*$/, '') // Remove trailing semicolon
            .trim();

        // Add appropriate CSS class
        let cssClass = '';
        if (styleContent.includes('break-before') || styleContent.includes('page-break-before')) {
            cssClass = 'page-break-before';
        } else if (styleContent.includes('break-after') || styleContent.includes('page-break-after')) {
            cssClass = 'page-break-after';
        } else if (styleContent.includes('break-inside') || styleContent.includes('page-break-inside')) {
            cssClass = 'page-break-avoid';
        }

        // Reconstruct the tag
        let result = `<${tag}`;
        
        if (cssClass) {
            // Check if class attribute already exists
            if (rest.includes('class=')) {
                result += rest.replace(/class="([^"]*)"/, `class="$1 ${cssClass}"`);
            } else {
                result += ` class="${cssClass}"${rest}`;
            }
        } else {
            result += rest;
        }

        // Add style attribute only if there are remaining styles
        if (cleanStyle) {
            result += ` style="${cleanStyle}"`;
        }

        result += '>';
        return result;
    });

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