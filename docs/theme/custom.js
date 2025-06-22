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