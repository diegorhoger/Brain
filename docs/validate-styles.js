#!/usr/bin/env node

/**
 * Validate HTML files for inline styles
 * This script checks for any remaining inline style attributes
 */

const fs = require('fs');
const path = require('path');

function validateFile(filePath) {
    console.log(`🔍 Validating: ${filePath}`);
    
    if (!fs.existsSync(filePath)) {
        console.log(`❌ File not found: ${filePath}`);
        return false;
    }
    
    const content = fs.readFileSync(filePath, 'utf8');
    const lines = content.split('\n');
    
    let hasInlineStyles = false;
    let styleCount = 0;
    
    lines.forEach((line, index) => {
        const lineNumber = index + 1;
        
        // Look for style attributes
        const styleMatches = line.match(/\sstyle\s*=\s*["'][^"']*["']/gi);
        
        if (styleMatches) {
            hasInlineStyles = true;
            styleCount += styleMatches.length;
            
            console.log(`❌ Line ${lineNumber}: Found ${styleMatches.length} inline style(s)`);
            styleMatches.forEach(match => {
                console.log(`   → ${match.trim()}`);
            });
        }
    });
    
    if (!hasInlineStyles) {
        console.log(`✅ No inline styles found`);
        return true;
    } else {
        console.log(`❌ Found ${styleCount} total inline styles`);
        return false;
    }
}

function main() {
    console.log('🚀 Starting inline style validation...\n');
    
    const filesToCheck = [
        'book/index.html',
        'book/print.html'
    ];
    
    let allValid = true;
    
    filesToCheck.forEach(file => {
        const isValid = validateFile(file);
        allValid = allValid && isValid;
        console.log('');
    });
    
    if (allValid) {
        console.log('🎉 All files passed validation!');
        console.log('✨ No inline styles detected');
        process.exit(0);
    } else {
        console.log('❌ Validation failed!');
        console.log('🔧 Some files contain inline styles');
        process.exit(1);
    }
}

if (require.main === module) {
    main();
}

module.exports = { validateFile }; 