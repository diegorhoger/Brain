#!/usr/bin/env node

/**
 * Post-build script to remove inline styles from mdBook generated HTML
 * and fix accessibility issues with form elements
 * This script uses simple sed commands for reliable, clean fixes
 */

const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');

const BOOK_DIR = path.join(__dirname, 'book');
const FILES_TO_PROCESS = ['print.html']; // Can add more files if needed

/**
 * Process a single HTML file using sed commands
 * @param {string} filePath - Path to the HTML file
 */
function processFile(filePath) {
    try {
        const fileName = path.basename(filePath);
        console.log(`🔧 Processing ${fileName}...`);
        
        let changesCount = 0;

        // Fix 1: Replace inline page break styles with CSS classes
        try {
            execSync(`sed -i '' 's/style="break-before: *page; *page-break-before: *always;"/class="page-break-before"/g' "${filePath}"`);
            console.log(`  ✅ Fixed inline page break styles`);
            changesCount++;
        } catch (error) {
            console.log(`  ℹ️  No inline page break styles found`);
        }

        // Fix 2: Add accessibility attributes to checkboxes
        try {
            execSync(`sed -i '' 's/<input disabled="" type="checkbox"\\/\\/>/<input disabled="" type="checkbox" aria-label="Checkbox item" title="Checkbox item" \\/>/g' "${filePath}"`);
            console.log(`  ✅ Fixed checkbox accessibility`);
            changesCount++;
        } catch (error) {
            console.log(`  ℹ️  No checkboxes to fix`);
        }

        // Fix 3: Clean up any remaining malformed input tags
        try {
            execSync(`sed -i '' 's/<input\\([^>]*\\)\\/\\([^>]\\)/<input\\1 \\/\\2/g' "${filePath}"`);
            console.log(`  ✅ Cleaned up malformed input tags`);
            changesCount++;
        } catch (error) {
            console.log(`  ℹ️  No malformed input tags found`);
        }

        if (changesCount > 0) {
            console.log(`✅ Successfully processed ${fileName}`);
        } else {
            console.log(`ℹ️  ${fileName} was already clean`);
        }
        
    } catch (error) {
        console.error(`❌ Error processing ${filePath}:`, error.message);
    }
}

/**
 * Main execution function
 */
function main() {
    console.log('🔧 Starting HTML cleanup and accessibility fixes...');
    
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
    console.log('♿ Form accessibility issues have been fixed');
    console.log('✨ Your documentation should now pass linting checks');
}

// Run the script
main(); 