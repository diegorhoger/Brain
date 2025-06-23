// Populate the sidebar
//
// This is a script, and not included directly in the page, to control the total size of the book.
// The TOC contains an entry for each page, so if each page includes a copy of the TOC,
// the total size of the page becomes O(n**2).
class MDBookSidebarScrollbox extends HTMLElement {
    constructor() {
        super();
    }
    connectedCallback() {
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded affix "><a href="introduction.html">Introduction</a></li><li class="chapter-item expanded affix "><li class="part-title">Getting Started</li><li class="chapter-item expanded "><a href="getting-started/quick-start.html">Quick Start</a></li><li class="chapter-item expanded "><a href="getting-started/installation.html">Installation</a></li><li class="chapter-item expanded "><a href="getting-started/configuration.html">Configuration</a></li><li class="chapter-item expanded "><a href="getting-started/first-steps.html">First Steps</a></li><li class="chapter-item expanded affix "><li class="part-title">Architecture Overview</li><li class="chapter-item expanded "><a href="architecture/system-architecture.html">System Architecture</a></li><li class="chapter-item expanded "><a href="architecture/cognitive-pipeline.html">Cognitive Pipeline</a></li><li class="chapter-item expanded "><a href="architecture/data-flow.html">Data Flow</a></li><li class="chapter-item expanded "><a href="architecture/component-interactions.html">Component Interactions</a></li><li class="chapter-item expanded affix "><li class="part-title">Core Components</li><li class="chapter-item expanded "><a href="components/character-ingestion.html">Character Ingestion Engine</a></li><li class="chapter-item expanded "><a href="components/segment-discovery.html">Segment Discovery Module</a></li><li class="chapter-item expanded "><a href="components/memory-system.html">Memory System</a></li><li class="chapter-item expanded "><a href="components/concept-graph.html">Concept Graph Engine</a></li><li class="chapter-item expanded "><a href="components/insight-extraction.html">Insight Extraction Engine</a></li><li class="chapter-item expanded "><a href="components/simulation-engine.html">Simulation Engine</a></li><li class="chapter-item expanded "><a href="components/neural-architecture.html">Neural Architecture</a></li><li class="chapter-item expanded affix "><li class="part-title">Advanced Features</li><li class="chapter-item expanded "><a href="advanced/meta-memory.html">Meta-Memory System</a></li><li class="chapter-item expanded "><a href="advanced/novelty-detection.html">Novelty Detection</a></li><li class="chapter-item expanded "><a href="advanced/curiosity-learning.html">Curiosity Learning</a></li><li class="chapter-item expanded "><a href="advanced/performance-monitoring.html">Performance Monitoring</a></li><li class="chapter-item expanded "><a href="advanced/system-integration.html">System Integration</a></li><li class="chapter-item expanded affix "><li class="part-title">API Reference</li><li class="chapter-item expanded "><a href="api/overview.html">REST API Overview</a></li><li class="chapter-item expanded "><a href="api/authentication.html">Authentication</a></li><li class="chapter-item expanded "><a href="api/core-endpoints.html">Core Endpoints</a></li><li class="chapter-item expanded "><a href="api/query-system.html">Query System</a></li><li class="chapter-item expanded "><a href="api/visualization.html">Visualization API</a></li><li class="chapter-item expanded "><a href="api/error-handling.html">Error Handling</a></li><li class="chapter-item expanded affix "><li class="part-title">Python Bindings</li><li class="chapter-item expanded "><a href="python/overview.html">Python API Overview</a></li><li class="chapter-item expanded "><a href="python/installation.html">Installation &amp; Setup</a></li><li class="chapter-item expanded "><a href="python/basic-usage.html">Basic Usage</a></li><li class="chapter-item expanded "><a href="python/advanced-examples.html">Advanced Examples</a></li><li class="chapter-item expanded "><a href="python/type-definitions.html">Type Definitions</a></li><li class="chapter-item expanded affix "><li class="part-title">Deployment &amp; Operations</li><li class="chapter-item expanded "><a href="deployment/docker.html">Docker Deployment</a></li><li class="chapter-item expanded "><a href="deployment/configuration.html">Configuration Management</a></li><li class="chapter-item expanded "><a href="deployment/monitoring.html">Monitoring &amp; Logging</a></li><li class="chapter-item expanded "><a href="deployment/backup-recovery.html">Backup &amp; Recovery</a></li><li class="chapter-item expanded "><a href="deployment/scaling.html">Scaling &amp; Performance</a></li><li class="chapter-item expanded "><a href="deployment/troubleshooting.html">Troubleshooting</a></li><li class="chapter-item expanded affix "><li class="part-title">Development</li><li class="chapter-item expanded "><a href="development/setup.html">Development Setup</a></li><li class="chapter-item expanded "><a href="development/code-organization.html">Code Organization</a></li><li class="chapter-item expanded "><a href="development/testing.html">Testing Strategy</a></li><li class="chapter-item expanded "><a href="development/contributing.html">Contributing Guidelines</a></li><li class="chapter-item expanded "><a href="development/release-process.html">Release Process</a></li><li class="chapter-item expanded affix "><li class="part-title">Examples &amp; Tutorials</li><li class="chapter-item expanded "><a href="examples/basic-examples.html">Basic Examples</a></li><li class="chapter-item expanded "><a href="examples/advanced-use-cases.html">Advanced Use Cases</a></li><li class="chapter-item expanded "><a href="examples/integration-examples.html">Integration Examples</a></li><li class="chapter-item expanded "><a href="examples/performance-optimization.html">Performance Optimization</a></li><li class="chapter-item expanded affix "><li class="part-title">Reference</li><li class="chapter-item expanded "><a href="reference/configuration.html">Configuration Reference</a></li><li class="chapter-item expanded "><a href="reference/error-codes.html">Error Codes</a></li><li class="chapter-item expanded "><a href="reference/performance-metrics.html">Performance Metrics</a></li><li class="chapter-item expanded "><a href="reference/glossary.html">Glossary</a></li><li class="chapter-item expanded "><a href="reference/faq.html">FAQ</a></li><li class="chapter-item expanded affix "><li class="part-title">Appendices</li><li class="chapter-item expanded "><a href="appendices/changelog.html">Changelog</a></li><li class="chapter-item expanded "><a href="appendices/migration-guide.html">Migration Guide</a></li><li class="chapter-item expanded "><a href="appendices/research-background.html">Research Background</a></li><li class="chapter-item expanded "><a href="appendices/license.html">License</a></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString().split("#")[0].split("?")[0];
        if (current_page.endsWith("/")) {
            current_page += "index.html";
        }
        var links = Array.prototype.slice.call(this.querySelectorAll("a"));
        var l = links.length;
        for (var i = 0; i < l; ++i) {
            var link = links[i];
            var href = link.getAttribute("href");
            if (href && !href.startsWith("#") && !/^(?:[a-z+]+:)?\/\//.test(href)) {
                link.href = path_to_root + href;
            }
            // The "index" page is supposed to alias the first chapter in the book.
            if (link.href === current_page || (i === 0 && path_to_root === "" && current_page.endsWith("/index.html"))) {
                link.classList.add("active");
                var parent = link.parentElement;
                if (parent && parent.classList.contains("chapter-item")) {
                    parent.classList.add("expanded");
                }
                while (parent) {
                    if (parent.tagName === "LI" && parent.previousElementSibling) {
                        if (parent.previousElementSibling.classList.contains("chapter-item")) {
                            parent.previousElementSibling.classList.add("expanded");
                        }
                    }
                    parent = parent.parentElement;
                }
            }
        }
        // Track and set sidebar scroll position
        this.addEventListener('click', function(e) {
            if (e.target.tagName === 'A') {
                sessionStorage.setItem('sidebar-scroll', this.scrollTop);
            }
        }, { passive: true });
        var sidebarScrollTop = sessionStorage.getItem('sidebar-scroll');
        sessionStorage.removeItem('sidebar-scroll');
        if (sidebarScrollTop) {
            // preserve sidebar scroll position when navigating via links within sidebar
            this.scrollTop = sidebarScrollTop;
        } else {
            // scroll sidebar to current active section when navigating via "next/previous chapter" buttons
            var activeSection = document.querySelector('#sidebar .active');
            if (activeSection) {
                activeSection.scrollIntoView({ block: 'center' });
            }
        }
        // Toggle buttons
        var sidebarAnchorToggles = document.querySelectorAll('#sidebar a.toggle');
        function toggleSection(ev) {
            ev.currentTarget.parentElement.classList.toggle('expanded');
        }
        Array.from(sidebarAnchorToggles).forEach(function (el) {
            el.addEventListener('click', toggleSection);
        });
    }
}
window.customElements.define("mdbook-sidebar-scrollbox", MDBookSidebarScrollbox);
