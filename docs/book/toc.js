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
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded affix "><a href="introduction.html">Introduction</a></li><li class="chapter-item expanded affix "><li class="part-title">Getting Started</li><li class="chapter-item expanded "><a href="getting-started/quick-start.html"><strong aria-hidden="true">1.</strong> Quick Start</a></li><li class="chapter-item expanded "><a href="getting-started/installation.html"><strong aria-hidden="true">2.</strong> Installation</a></li><li class="chapter-item expanded "><a href="getting-started/configuration.html"><strong aria-hidden="true">3.</strong> Configuration</a></li><li class="chapter-item expanded "><a href="getting-started/first-steps.html"><strong aria-hidden="true">4.</strong> First Steps</a></li><li class="chapter-item expanded affix "><li class="part-title">Architecture Overview</li><li class="chapter-item expanded "><a href="architecture/system-architecture.html"><strong aria-hidden="true">5.</strong> System Architecture</a></li><li class="chapter-item expanded "><a href="architecture/cognitive-pipeline.html"><strong aria-hidden="true">6.</strong> Cognitive Pipeline</a></li><li class="chapter-item expanded "><a href="architecture/data-flow.html"><strong aria-hidden="true">7.</strong> Data Flow</a></li><li class="chapter-item expanded "><a href="architecture/component-interactions.html"><strong aria-hidden="true">8.</strong> Component Interactions</a></li><li class="chapter-item expanded affix "><li class="part-title">Core Components</li><li class="chapter-item expanded "><a href="components/character-ingestion.html"><strong aria-hidden="true">9.</strong> Character Ingestion Engine</a></li><li class="chapter-item expanded "><a href="components/segment-discovery.html"><strong aria-hidden="true">10.</strong> Segment Discovery Module</a></li><li class="chapter-item expanded "><a href="components/memory-system.html"><strong aria-hidden="true">11.</strong> Memory System</a></li><li class="chapter-item expanded "><a href="components/concept-graph.html"><strong aria-hidden="true">12.</strong> Concept Graph Engine</a></li><li class="chapter-item expanded "><a href="components/insight-extraction.html"><strong aria-hidden="true">13.</strong> Insight Extraction Engine</a></li><li class="chapter-item expanded "><a href="components/simulation-engine.html"><strong aria-hidden="true">14.</strong> Simulation Engine</a></li><li class="chapter-item expanded "><a href="components/neural-architecture.html"><strong aria-hidden="true">15.</strong> Neural Architecture</a></li><li class="chapter-item expanded affix "><li class="part-title">Advanced Features</li><li class="chapter-item expanded "><a href="advanced/meta-memory.html"><strong aria-hidden="true">16.</strong> Meta-Memory System</a></li><li class="chapter-item expanded "><a href="advanced/novelty-detection.html"><strong aria-hidden="true">17.</strong> Novelty Detection</a></li><li class="chapter-item expanded "><a href="advanced/curiosity-learning.html"><strong aria-hidden="true">18.</strong> Curiosity Learning</a></li><li class="chapter-item expanded "><a href="advanced/performance-monitoring.html"><strong aria-hidden="true">19.</strong> Performance Monitoring</a></li><li class="chapter-item expanded "><a href="advanced/system-integration.html"><strong aria-hidden="true">20.</strong> System Integration</a></li><li class="chapter-item expanded affix "><li class="part-title">API Reference</li><li class="chapter-item expanded "><a href="api/overview.html"><strong aria-hidden="true">21.</strong> REST API Overview</a></li><li class="chapter-item expanded "><a href="api/authentication.html"><strong aria-hidden="true">22.</strong> Authentication</a></li><li class="chapter-item expanded "><a href="api/core-endpoints.html"><strong aria-hidden="true">23.</strong> Core Endpoints</a></li><li class="chapter-item expanded "><a href="api/query-system.html"><strong aria-hidden="true">24.</strong> Query System</a></li><li class="chapter-item expanded "><a href="api/visualization.html"><strong aria-hidden="true">25.</strong> Visualization API</a></li><li class="chapter-item expanded "><a href="api/error-handling.html"><strong aria-hidden="true">26.</strong> Error Handling</a></li><li class="chapter-item expanded affix "><li class="part-title">Python Bindings</li><li class="chapter-item expanded "><a href="python/overview.html"><strong aria-hidden="true">27.</strong> Python API Overview</a></li><li class="chapter-item expanded "><a href="python/installation.html"><strong aria-hidden="true">28.</strong> Installation &amp; Setup</a></li><li class="chapter-item expanded "><a href="python/basic-usage.html"><strong aria-hidden="true">29.</strong> Basic Usage</a></li><li class="chapter-item expanded "><a href="python/advanced-examples.html"><strong aria-hidden="true">30.</strong> Advanced Examples</a></li><li class="chapter-item expanded "><a href="python/type-definitions.html"><strong aria-hidden="true">31.</strong> Type Definitions</a></li><li class="chapter-item expanded affix "><li class="part-title">Deployment &amp; Operations</li><li class="chapter-item expanded "><a href="deployment/docker.html"><strong aria-hidden="true">32.</strong> Docker Deployment</a></li><li class="chapter-item expanded "><a href="deployment/configuration.html"><strong aria-hidden="true">33.</strong> Configuration Management</a></li><li class="chapter-item expanded "><a href="deployment/monitoring.html"><strong aria-hidden="true">34.</strong> Monitoring &amp; Logging</a></li><li class="chapter-item expanded "><a href="deployment/backup-recovery.html"><strong aria-hidden="true">35.</strong> Backup &amp; Recovery</a></li><li class="chapter-item expanded "><a href="deployment/scaling.html"><strong aria-hidden="true">36.</strong> Scaling &amp; Performance</a></li><li class="chapter-item expanded "><a href="deployment/troubleshooting.html"><strong aria-hidden="true">37.</strong> Troubleshooting</a></li><li class="chapter-item expanded affix "><li class="part-title">Development</li><li class="chapter-item expanded "><a href="development/setup.html"><strong aria-hidden="true">38.</strong> Development Setup</a></li><li class="chapter-item expanded "><a href="development/code-organization.html"><strong aria-hidden="true">39.</strong> Code Organization</a></li><li class="chapter-item expanded "><a href="development/testing.html"><strong aria-hidden="true">40.</strong> Testing Strategy</a></li><li class="chapter-item expanded "><a href="development/contributing.html"><strong aria-hidden="true">41.</strong> Contributing Guidelines</a></li><li class="chapter-item expanded "><a href="development/release-process.html"><strong aria-hidden="true">42.</strong> Release Process</a></li><li class="chapter-item expanded affix "><li class="part-title">Examples &amp; Tutorials</li><li class="chapter-item expanded "><a href="examples/basic-examples.html"><strong aria-hidden="true">43.</strong> Basic Examples</a></li><li class="chapter-item expanded "><a href="examples/advanced-use-cases.html"><strong aria-hidden="true">44.</strong> Advanced Use Cases</a></li><li class="chapter-item expanded "><a href="examples/integration-examples.html"><strong aria-hidden="true">45.</strong> Integration Examples</a></li><li class="chapter-item expanded "><a href="examples/performance-optimization.html"><strong aria-hidden="true">46.</strong> Performance Optimization</a></li><li class="chapter-item expanded affix "><li class="part-title">Reference</li><li class="chapter-item expanded "><a href="reference/configuration.html"><strong aria-hidden="true">47.</strong> Configuration Reference</a></li><li class="chapter-item expanded "><a href="reference/error-codes.html"><strong aria-hidden="true">48.</strong> Error Codes</a></li><li class="chapter-item expanded "><a href="reference/performance-metrics.html"><strong aria-hidden="true">49.</strong> Performance Metrics</a></li><li class="chapter-item expanded "><a href="reference/glossary.html"><strong aria-hidden="true">50.</strong> Glossary</a></li><li class="chapter-item expanded "><a href="reference/faq.html"><strong aria-hidden="true">51.</strong> FAQ</a></li><li class="chapter-item expanded affix "><li class="part-title">Appendices</li><li class="chapter-item expanded "><a href="appendices/changelog.html"><strong aria-hidden="true">52.</strong> Changelog</a></li><li class="chapter-item expanded "><a href="appendices/migration-guide.html"><strong aria-hidden="true">53.</strong> Migration Guide</a></li><li class="chapter-item expanded "><a href="appendices/research-background.html"><strong aria-hidden="true">54.</strong> Research Background</a></li><li class="chapter-item expanded "><a href="appendices/license.html"><strong aria-hidden="true">55.</strong> License</a></li></ol>';
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
