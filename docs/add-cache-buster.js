#!/usr/bin/env node

/**
 * Add cache-busting parameters to CSS and JS files in HTML
 * This helps ensure browsers load the latest versions
 */

const fs = require('fs');
const path = require('path');
const crypto = require('crypto');

function addCacheBuster(filePath) {
    console.log(`🔄 Adding cache buster to: ${filePath}`);
    
    if (!fs.existsSync(filePath)) {
        console.log(`❌ File not found: ${filePath}`);
        return false;
    }
    
    let content = fs.readFileSync(filePath, 'utf8');
    const timestamp = Date.now();
    const hash = crypto.createHash('md5').update(content).digest('hex').substring(0, 8);
    const cacheBuster = `v=${timestamp}-${hash}`;
    
    // Add cache buster to CSS files
    content = content.replace(
        /(<link[^>]*href=["'])([^"']*\.css)(["'][^>]*>)/gi,
        `$1$2?${cacheBuster}$3`
    );
    
    // Add cache buster to JS files
    content = content.replace(
        /(<script[^>]*src=["'])([^"']*\.js)(["'][^>]*>)/gi,
        `$1$2?${cacheBuster}$3`
    );
    
    // Add meta tag to prevent caching of the HTML itself
    const metaTag = `    <meta http-equiv="Cache-Control" content="no-cache, no-store, must-revalidate">
    <meta http-equiv="Pragma" content="no-cache">
    <meta http-equiv="Expires" content="0">
    <meta name="cache-buster" content="${cacheBuster}">`;
    
    content = content.replace(
        /(<head[^>]*>)/i,
        `$1\n${metaTag}`
    );
    
    fs.writeFileSync(filePath, content, 'utf8');
    console.log(`✅ Cache buster added: ${cacheBuster}`);
    return true;
}

function main() {
    console.log('🚀 Adding cache busters to documentation...\n');
    
    const filesToProcess = [
        'book/index.html',
        'book/print.html'
    ];
    
    let processedCount = 0;
    
    filesToProcess.forEach(file => {
        if (addCacheBuster(file)) {
            processedCount++;
        }
        console.log('');
    });
    
    console.log(`🎉 Cache busters added to ${processedCount} file(s)!`);
    console.log('🔄 Browsers should now load the latest versions');
}

if (require.main === module) {
    main();
}

module.exports = { addCacheBuster }; 