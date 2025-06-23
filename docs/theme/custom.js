/**
 * Brain AI Documentation Custom JavaScript
 * Handles browser compatibility and removes inline styles for better validation
 */

// Fix browser compatibility issues
(function() {
    'use strict';

    // Remove theme-color meta tag for browsers that don't support it
    function fixThemeColorCompatibility() {
        const themeColorMeta = document.querySelector('meta[name="theme-color"]');
        if (themeColorMeta) {
            // Check if browser supports theme-color
            const isFirefox = navigator.userAgent.toLowerCase().indexOf('firefox') > -1;
            const isOpera = navigator.userAgent.toLowerCase().indexOf('opera') > -1 || 
                           navigator.userAgent.toLowerCase().indexOf('opr') > -1;
            
            // Remove for unsupported browsers to avoid warnings
            if (isFirefox || isOpera) {
                themeColorMeta.remove();
            }
        }
    }

    // Remove inline styles and replace with CSS classes
    function removeInlineStyles() {
        // Find all elements with inline styles
        const elementsWithInlineStyles = document.querySelectorAll('[style]');
        
        elementsWithInlineStyles.forEach(element => {
            const inlineStyle = element.getAttribute('style');
            
            // Handle page break styles specifically (main source of warnings)
            if (inlineStyle && (inlineStyle.includes('break-before: page') || inlineStyle.includes('page-break-before: always'))) {
                element.classList.add('page-break');
                element.removeAttribute('style');
                return;
            }
            
            if (inlineStyle && (inlineStyle.includes('break-after: page') || inlineStyle.includes('page-break-after: always'))) {
                element.classList.add('page-break-after');
                element.removeAttribute('style');
                return;
            }
            
            if (inlineStyle && (inlineStyle.includes('break-inside: avoid') || inlineStyle.includes('page-break-inside: avoid'))) {
                element.classList.add('page-break-avoid');
                element.removeAttribute('style');
                return;
            }
            
            // Handle clear: both specifically
            if (inlineStyle && inlineStyle.includes('clear: both')) {
                element.classList.add('clear-both');
                element.removeAttribute('style');
                return;
            }
            
            // Handle other common inline styles
            if (inlineStyle) {
                // Replace common inline styles with classes
                if (inlineStyle.includes('display: none')) {
                    element.classList.add('hidden');
                    element.removeAttribute('style');
                }
                
                if (inlineStyle.includes('text-align: center')) {
                    element.classList.add('text-center');
                    element.removeAttribute('style');
                }
                
                if (inlineStyle.includes('margin: 0')) {
                    element.classList.add('no-margin');
                    element.removeAttribute('style');
                }
            }
        });
    }

    // Add utility CSS classes dynamically if they don't exist
    function addUtilityClasses() {
        const style = document.createElement('style');
        style.textContent = `
            .clear-both { clear: both !important; }
            .hidden { display: none !important; }
            .text-center { text-align: center !important; }
            .no-margin { margin: 0 !important; }
            
            /* Page break classes to replace inline styles */
            .page-break {
                break-before: page !important;
                page-break-before: always !important;
            }
            
            .page-break-after {
                break-after: page !important;
                page-break-after: always !important;
            }
            
            .page-break-avoid {
                break-inside: avoid !important;
                page-break-inside: avoid !important;
            }
            
            /* Ensure nav-wrapper has proper clearing */
            .nav-wrapper {
                clear: both !important;
            }
            
            .nav-wrapper::after {
                content: "";
                display: table;
                clear: both;
            }
        `;
        document.head.appendChild(style);
    }

    // Initialize fixes when DOM is ready
    function initializeFixes() {
        fixThemeColorCompatibility();
        addUtilityClasses();
        
        // Use a small delay to ensure all content is loaded
        setTimeout(() => {
            removeInlineStyles();
        }, 100);
        
        // Also run after any dynamic content changes
        const observer = new MutationObserver((mutations) => {
            mutations.forEach((mutation) => {
                if (mutation.type === 'childList' && mutation.addedNodes.length > 0) {
                    setTimeout(removeInlineStyles, 50);
                }
            });
        });
        
        observer.observe(document.body, {
            childList: true,
            subtree: true
        });
    }

    // Run when DOM is ready
    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', initializeFixes);
    } else {
        initializeFixes();
    }

})();

// Enhanced search functionality
(function() {
    'use strict';

    // Improve search results display
    function enhanceSearchResults() {
        const searchResults = document.getElementById('searchresults');
        if (searchResults) {
            searchResults.addEventListener('DOMNodeInserted', function() {
                const results = searchResults.querySelectorAll('li');
                results.forEach(result => {
                    result.style.borderRadius = '4px';
                    result.style.marginBottom = '8px';
                    result.style.padding = '8px';
                    result.style.border = '1px solid #e5e7eb';
                });
            });
        }
    }

    // Add keyboard navigation improvements
    function enhanceKeyboardNavigation() {
        document.addEventListener('keydown', function(e) {
            // Improve accessibility
            if (e.key === 'Escape') {
                const searchWrapper = document.getElementById('search-wrapper');
                if (searchWrapper && !searchWrapper.classList.contains('hidden')) {
                    document.getElementById('search-toggle').click();
                }
            }
        });
    }

    // Improve accessibility for form elements
    function enhanceAccessibility() {
        // Add proper label for sidebar toggle checkbox
        const sidebarToggle = document.getElementById('sidebar-toggle-anchor');
        if (sidebarToggle && !sidebarToggle.getAttribute('aria-label')) {
            sidebarToggle.setAttribute('aria-label', 'Toggle sidebar navigation');
            sidebarToggle.setAttribute('title', 'Toggle sidebar navigation');
        }

        // Ensure search input has proper labeling
        const searchInput = document.getElementById('searchbar');
        if (searchInput) {
            if (!searchInput.getAttribute('aria-label')) {
                searchInput.setAttribute('aria-label', 'Search documentation');
            }
            if (!searchInput.getAttribute('title')) {
                searchInput.setAttribute('title', 'Search the Brain AI documentation');
            }
        }

        // Add role and aria-label to sidebar resize handle
        const resizeHandle = document.getElementById('sidebar-resize-handle');
        if (resizeHandle) {
            resizeHandle.setAttribute('role', 'separator');
            resizeHandle.setAttribute('aria-label', 'Resize sidebar');
            resizeHandle.setAttribute('aria-orientation', 'vertical');
            resizeHandle.setAttribute('tabindex', '0');
        }

        // Improve focus management for sidebar links
        const sidebarLinks = document.querySelectorAll('#sidebar a');
        sidebarLinks.forEach(link => {
            if (!link.getAttribute('aria-describedby')) {
                const text = link.textContent.trim();
                if (text) {
                    link.setAttribute('aria-describedby', `Link to ${text} section`);
                }
            }
        });
    }

    // Initialize enhancements
    document.addEventListener('DOMContentLoaded', function() {
        enhanceSearchResults();
        enhanceKeyboardNavigation();
        enhanceAccessibility();
    });

})();

// Code block enhancements
(function() {
    'use strict';

    // Add copy buttons to code blocks
    function addCopyButtons() {
        const codeBlocks = document.querySelectorAll('pre code');
        
        codeBlocks.forEach(codeBlock => {
            const pre = codeBlock.parentElement;
            
            // Skip if copy button already exists
            if (pre.querySelector('.copy-button')) {
                return;
            }
            
            const copyButton = document.createElement('button');
            copyButton.className = 'copy-button';
            copyButton.textContent = 'Copy';
            copyButton.setAttribute('aria-label', 'Copy code to clipboard');
            
            copyButton.addEventListener('click', async () => {
                try {
                    await navigator.clipboard.writeText(codeBlock.textContent);
                    copyButton.textContent = 'Copied!';
                    setTimeout(() => {
                        copyButton.textContent = 'Copy';
                    }, 2000);
                } catch (err) {
                    console.error('Failed to copy code:', err);
                    copyButton.textContent = 'Failed';
                    setTimeout(() => {
                        copyButton.textContent = 'Copy';
                    }, 2000);
                }
            });
            
            pre.style.position = 'relative';
            copyButton.style.position = 'absolute';
            copyButton.style.top = '8px';
            copyButton.style.right = '8px';
            
            pre.appendChild(copyButton);
        });
    }

    // Initialize code enhancements
    document.addEventListener('DOMContentLoaded', function() {
        addCopyButtons();
        
        // Re-add copy buttons when new content is loaded
        const observer = new MutationObserver(() => {
            addCopyButtons();
        });
        
        observer.observe(document.body, {
            childList: true,
            subtree: true
        });
    });

})();

// Theme enhancements
(function() {
    'use strict';

    // Smooth theme transitions
    function enableSmoothThemeTransitions() {
        const style = document.createElement('style');
        style.textContent = `
            * {
                transition: background-color 0.3s ease, color 0.3s ease, border-color 0.3s ease;
            }
            
            .chapter-title {
                transition: all 0.3s ease;
            }
            
            .sidebar {
                transition: all 0.3s ease;
            }
        `;
        document.head.appendChild(style);
    }

    // Initialize theme enhancements
    document.addEventListener('DOMContentLoaded', function() {
        enableSmoothThemeTransitions();
    });

})();

// Brain AI Documentation Custom JavaScript

(function() {
    'use strict';

    // Initialize when DOM is ready
    document.addEventListener('DOMContentLoaded', function() {
        initializeCustomFeatures();
    });

    function initializeCustomFeatures() {
        addCopyButtonsToCodeBlocks();
        addAPIMethodStyling();
        addStatusBadges();
        addCalloutBoxes();
        initializeSearchEnhancements();
        addProgressIndicators();
    }

    // Add copy buttons to code blocks
    function addCopyButtonsToCodeBlocks() {
        const codeBlocks = document.querySelectorAll('pre code');
        
        codeBlocks.forEach(function(block) {
            const pre = block.parentNode;
            const button = document.createElement('button');
            button.className = 'copy-button';
            button.textContent = 'Copy';
            button.onclick = function() {
                copyToClipboard(block.textContent);
                button.textContent = 'Copied!';
                setTimeout(function() {
                    button.textContent = 'Copy';
                }, 2000);
            };
            
            pre.style.position = 'relative';
            button.style.position = 'absolute';
            button.style.top = '8px';
            button.style.right = '8px';
            
            pre.appendChild(button);
        });
    }

    // Style API method indicators
    function addAPIMethodStyling() {
        const content = document.querySelector('.content');
        if (!content) return;

        // Find API endpoints and style them
        const apiRegex = /(GET|POST|PUT|DELETE|PATCH)\s+(\/[^\s]+)/g;
        
        function walkTextNodes(node) {
            if (node.nodeType === 3) { // Text node
                const text = node.textContent;
                const matches = text.match(apiRegex);
                
                if (matches) {
                    const parent = node.parentNode;
                    const wrapper = document.createElement('div');
                    wrapper.className = 'api-endpoint';
                    
                    const newHTML = text.replace(apiRegex, function(match, method, path) {
                        return `<span class="api-method ${method.toLowerCase()}">${method}</span><span class="api-path">${path}</span>`;
                    });
                    
                    wrapper.innerHTML = newHTML;
                    parent.replaceChild(wrapper, node);
                }
            } else {
                for (let i = 0; i < node.childNodes.length; i++) {
                    walkTextNodes(node.childNodes[i]);
                }
            }
        }
        
        // Only process code blocks for API endpoints
        const codeBlocks = content.querySelectorAll('code');
        codeBlocks.forEach(walkTextNodes);
    }

    // Add status badges
    function addStatusBadges() {
        const statusRegex = /\b(complete|completed|done|in-progress|progress|pending|todo)\b/gi;
        const textNodes = getTextNodes(document.querySelector('.content'));
        
        textNodes.forEach(function(node) {
            const text = node.textContent;
            if (statusRegex.test(text)) {
                const parent = node.parentNode;
                const newHTML = text.replace(statusRegex, function(match) {
                    const status = match.toLowerCase();
                    let className = 'status-pending';
                    
                    if (status === 'complete' || status === 'completed' || status === 'done') {
                        className = 'status-complete';
                    } else if (status === 'in-progress' || status === 'progress') {
                        className = 'status-progress';
                    }
                    
                    return `<span class="status-badge ${className}">${match}</span>`;
                });
                
                const wrapper = document.createElement('span');
                wrapper.innerHTML = newHTML;
                parent.replaceChild(wrapper, node);
            }
        });
    }

    // Add callout boxes for special content
    function addCalloutBoxes() {
        const content = document.querySelector('.content');
        if (!content) return;

        // Look for specific patterns and convert to callouts
        const paragraphs = content.querySelectorAll('p');
        
        paragraphs.forEach(function(p) {
            const text = p.textContent.trim();
            
            if (text.startsWith('🎉') || text.startsWith('✅')) {
                p.className = 'callout success';
            } else if (text.startsWith('⚠️') || text.startsWith('Warning:')) {
                p.className = 'callout warning';
            } else if (text.startsWith('ℹ️') || text.startsWith('Note:')) {
                p.className = 'callout info';
            } else if (text.startsWith('❌') || text.startsWith('Error:')) {
                p.className = 'callout error';
            }
        });
    }

    // Enhance search functionality
    function initializeSearchEnhancements() {
        const searchInput = document.querySelector('#searchbar');
        if (!searchInput) return;

        // Add search suggestions
        searchInput.addEventListener('input', function() {
            const query = this.value.toLowerCase();
            if (query.length < 2) return;

            // Simple search suggestions based on common terms
            const suggestions = [
                'character ingestion', 'memory system', 'concept graph',
                'simulation engine', 'api reference', 'installation',
                'configuration', 'deployment', 'python bindings'
            ];

            const matches = suggestions.filter(s => s.includes(query));
            // Implementation would depend on mdBook's search system
        });
    }

    // Add progress indicators for documentation sections
    function addProgressIndicators() {
        const chapters = document.querySelectorAll('.chapter-item');
        
        chapters.forEach(function(chapter) {
            const link = chapter.querySelector('a');
            if (!link) return;

            // Add completion indicators based on content
            const href = link.getAttribute('href');
            if (href && href.includes('getting-started')) {
                addProgressBadge(link, 'Essential');
            } else if (href && href.includes('api')) {
                addProgressBadge(link, 'Reference');
            } else if (href && href.includes('examples')) {
                addProgressBadge(link, 'Tutorial');
            }
        });
    }

    function addProgressBadge(element, type) {
        const badge = document.createElement('span');
        badge.className = `progress-badge ${type.toLowerCase()}`;
        badge.textContent = type;
        badge.style.cssText = `
            font-size: 10px;
            padding: 2px 6px;
            border-radius: 8px;
            margin-left: 8px;
            background: var(--brain-accent);
            color: white;
        `;
        element.appendChild(badge);
    }

    // Utility functions
    function copyToClipboard(text) {
        if (navigator.clipboard) {
            navigator.clipboard.writeText(text);
        } else {
            // Fallback for older browsers
            const textArea = document.createElement('textarea');
            textArea.value = text;
            document.body.appendChild(textArea);
            textArea.select();
            document.execCommand('copy');
            document.body.removeChild(textArea);
        }
    }

    function getTextNodes(element) {
        const textNodes = [];
        
        function walk(node) {
            if (node.nodeType === 3) {
                textNodes.push(node);
            } else {
                for (let i = 0; i < node.childNodes.length; i++) {
                    walk(node.childNodes[i]);
                }
            }
        }
        
        if (element) walk(element);
        return textNodes;
    }

    // Add smooth scrolling for anchor links
    document.addEventListener('click', function(e) {
        if (e.target.tagName === 'A' && e.target.getAttribute('href').startsWith('#')) {
            e.preventDefault();
            const target = document.querySelector(e.target.getAttribute('href'));
            if (target) {
                target.scrollIntoView({ behavior: 'smooth' });
            }
        }
    });

    // Add keyboard shortcuts
    document.addEventListener('keydown', function(e) {
        // Ctrl/Cmd + K to focus search
        if ((e.ctrlKey || e.metaKey) && e.key === 'k') {
            e.preventDefault();
            const searchInput = document.querySelector('#searchbar');
            if (searchInput) {
                searchInput.focus();
                searchInput.select();
            }
        }
    });

    // Initialize mermaid diagrams if present
    if (typeof mermaid !== 'undefined') {
        mermaid.initialize({
            theme: 'default',
            themeVariables: {
                primaryColor: '#2563eb',
                primaryTextColor: '#1f2937',
                primaryBorderColor: '#3b82f6',
                lineColor: '#6b7280',
                secondaryColor: '#f3f4f6',
                tertiaryColor: '#ffffff'
            }
        });
    }

    // Add table of contents for long pages
    function addTableOfContents() {
        const content = document.querySelector('.content');
        if (!content) return;

        const headings = content.querySelectorAll('h2, h3, h4');
        if (headings.length < 3) return; // Only add TOC if there are enough headings

        const toc = document.createElement('div');
        toc.className = 'table-of-contents';
        toc.innerHTML = '<h3>Table of Contents</h3><ul></ul>';
        
        const list = toc.querySelector('ul');
        
        headings.forEach(function(heading, index) {
            const id = heading.id || `heading-${index}`;
            heading.id = id;
            
            const li = document.createElement('li');
            const a = document.createElement('a');
            a.href = `#${id}`;
            a.textContent = heading.textContent;
            a.className = `toc-${heading.tagName.toLowerCase()}`;
            
            li.appendChild(a);
            list.appendChild(li);
        });

        // Insert TOC after the first paragraph
        const firstP = content.querySelector('p');
        if (firstP) {
            firstP.parentNode.insertBefore(toc, firstP.nextSibling);
        }
    }

    // Initialize TOC for appropriate pages
    if (document.querySelector('.content h2')) {
        addTableOfContents();
    }

})(); 