//! Frontend Coder Agent - Frontend Implementation and Code Generation
//! 
//! The FrontendCoder transforms UI/UX designs and API specifications into comprehensive
//! frontend implementation code, supporting multiple frameworks, component architectures,
//! and modern development patterns optimized for performance and maintainability.

use std::collections::HashMap;
use async_trait::async_trait;
use serde_json::{json, Value};

use crate::agents::traits::{
    BrainAgent, AgentMetadata, AgentInput, AgentOutput, CognitivePreferences,
    CognitiveContext, VerbosityLevel, ExecutionMetadata, ExecutionStatus,
    BrainResult
};
use crate::agents::standards::{EliteCodeGenerator, EliteCodeValidator};

/// Specialized agent for frontend implementation and code generation
#[derive(Clone)]
pub struct FrontendCoder {
    metadata: AgentMetadata,
    preferences: CognitivePreferences,
    #[allow(dead_code)]
    elite_generator: EliteCodeGenerator,
    #[allow(dead_code)]
    elite_validator: EliteCodeValidator,
}

impl FrontendCoder {
    /// Create a new FrontendCoder instance
    pub fn new() -> Self {
        let metadata = AgentMetadata {
            name: "Frontend Implementation Specialist".to_string(),
            id: "frontend-coder".to_string(),
            description: "Frontend development agent specializing in React, Vue.js, responsive design, and modern frontend technologies.".to_string(),
            version: "1.0.0".to_string(),
            persona: "Transforms UI/UX designs and API specifications into production-ready frontend code".to_string(),
            capabilities: vec![
                "component_generation".to_string(),
                "react_development".to_string(),
                "vue_development".to_string(),
                "angular_development".to_string(),
                "state_management".to_string(),
                "api_integration".to_string(),
                "responsive_design".to_string(),
                "accessibility_implementation".to_string(),
                "performance_optimization".to_string(),
                "testing_implementation".to_string(),
            ],
            dependencies: vec!["designer-agent".to_string(), "api-agent".to_string()],
            supported_input_types: vec![
                "ui_design_specifications".to_string(),
                "api_specifications".to_string(),
                "component_requirements".to_string(),
                "user_interactions".to_string(),
                "responsive_requirements".to_string(),
                "accessibility_requirements".to_string(),
            ],
            supported_output_types: vec![
                "frontend_codebase".to_string(),
                "component_library".to_string(),
                "api_integration_layer".to_string(),
                "routing_configuration".to_string(),
                "state_management_setup".to_string(),
                "testing_suite".to_string(),
            ],
            tags: vec!["frontend".to_string(), "development".to_string(), "code-generation".to_string()],
            base_confidence: 0.85,
        };

        let preferences = CognitivePreferences {
            verbosity: VerbosityLevel::Standard,
            risk_tolerance: 0.3, // Conservative for production code
            collaboration_preference: 0.8, // High collaboration
            learning_enabled: true,
            adaptation_rate: 0.5,
            creativity_level: 0.85, // High creativity for frontend solutions
            detail_level: 0.75, // Good detail level for implementation
            collaboration_style: "iterative".to_string(), // Iterative collaboration for frontend development
        };

        // Load Elite Code Framework from code.json or use defaults
        let framework = super::super::standards::framework::load_framework()
            .unwrap_or_else(|_| super::super::standards::framework::default_framework());
        
        let elite_generator = EliteCodeGenerator::new();
        let elite_validator = EliteCodeValidator::new(framework);

        Self { metadata, preferences, elite_generator, elite_validator }
    }

    /// Generate comprehensive frontend codebase from designs and API specs
    async fn generate_frontend_codebase(&self, ui_design: &Value, api_specs: &Value, _context: &CognitiveContext) -> BrainResult<Value> {
        let mut codebase = HashMap::new();
        
        // Extract framework preference from design requirements
        let framework = self.determine_frontend_framework(ui_design);
        
        // Generate component architecture
        let components = self.generate_component_library(ui_design, &framework);
        let routing = self.generate_routing_configuration(ui_design, &framework);
        let state_management = self.generate_state_management(ui_design, api_specs, &framework);
        let api_integration = self.generate_api_integration_layer(api_specs, &framework);
        let styling = self.generate_styling_system(ui_design, &framework);
        let accessibility = self.generate_accessibility_features(ui_design, &framework);
        
        codebase.insert("framework", json!(framework));
        codebase.insert("components", components);
        codebase.insert("routing", routing);
        codebase.insert("state_management", state_management);
        codebase.insert("api_integration", api_integration);
        codebase.insert("styling_system", styling);
        codebase.insert("accessibility_features", accessibility);
        codebase.insert("project_structure", self.generate_project_structure(&framework));
        codebase.insert("build_configuration", self.generate_build_configuration(&framework));
        codebase.insert("package_dependencies", self.generate_package_dependencies(&framework));
        
        Ok(json!(codebase))
    }

    /// Determine optimal frontend framework based on requirements
    fn determine_frontend_framework(&self, ui_design: &Value) -> String {
        // Analyze design complexity and requirements
        let complexity_score = self.analyze_ui_complexity(ui_design);
        let team_preference = ui_design.get("framework_preference")
            .and_then(|f| f.as_str())
            .unwrap_or("");
        
        match team_preference {
            "react" => "React".to_string(),
            "vue" => "Vue 3".to_string(),
            "angular" => "Angular".to_string(),
            _ => {
                // Auto-select based on complexity
                if complexity_score > 0.8 {
                    "React".to_string() // High complexity - React for flexibility
                } else if complexity_score > 0.5 {
                    "Vue 3".to_string() // Medium complexity - Vue for balance
                } else {
                    "React".to_string() // Default to React for ecosystem
                }
            }
        }
    }

    /// Analyze UI complexity to inform framework choice
    fn analyze_ui_complexity(&self, _ui_design: &Value) -> f64 {
        // Simplified complexity analysis
        // In a real implementation, this would analyze:
        // - Number of unique components
        // - Interaction complexity
        // - State management requirements
        // - Real-time features
        // - Performance requirements
        
        0.7 // Default medium complexity
    }

    /// Generate comprehensive component library
    fn generate_component_library(&self, ui_design: &Value, framework: &str) -> Value {
        let mut components = HashMap::new();
        
        // Extract components from design specifications
        let empty_components = json!({});
        let ui_components = ui_design.get("components").unwrap_or(&empty_components);
        
        // Generate base component structure
        components.insert("layout", self.generate_layout_components(ui_components, framework));
        components.insert("navigation", self.generate_navigation_components(ui_components, framework));
        components.insert("forms", self.generate_form_components(ui_components, framework));
        components.insert("data_display", self.generate_data_display_components(ui_components, framework));
        components.insert("interactive", self.generate_interactive_components(ui_components, framework));
        components.insert("utility", self.generate_utility_components(framework));
        
        json!(components)
    }

    /// Generate layout components (Header, Footer, Sidebar, etc.)
    fn generate_layout_components(&self, _ui_components: &Value, framework: &str) -> Value {
        match framework {
            "React" => json!({
                "Header": {
                    "file": "src/components/layout/Header.tsx",
                    "code": "import React from 'react';\nimport { Link } from 'react-router-dom';\nimport { useAuth } from '../hooks/useAuth';\n\ninterface HeaderProps {\n  title?: string;\n  showAuth?: boolean;\n}\n\nexport const Header: React.FC<HeaderProps> = ({ title = 'Brain AI', showAuth = true }) => {\n  const { user, logout } = useAuth();\n\n  return (\n    <header className=\"bg-white shadow-sm border-b\">\n      <div className=\"max-w-7xl mx-auto px-4 sm:px-6 lg:px-8\">\n        <div className=\"flex justify-between items-center h-16\">\n          <div className=\"flex items-center\">\n            <Link to=\"/\" className=\"text-xl font-bold text-gray-900\">\n              {title}\n            </Link>\n          </div>\n          {showAuth && (\n            <div className=\"flex items-center space-x-4\">\n              {user ? (\n                <>\n                  <span className=\"text-gray-700\">Welcome, {user.name}</span>\n                  <button\
                    onClick={logout}\
                    className=\"text-gray-500 hover:text-gray-700\"\
                  >\
                    Logout\
                  </button>\
                </>\
              ) : (\
                <Link\
                  to=\"/login\"\
                  className=\"text-blue-600 hover:text-blue-800\"\
                >\
                  Login\
                </Link>\
              )}\
            </div>\
          )}\
        </div>\
      </div>\
    </header>\n  );\n};"
                },
                "Layout": {
                    "file": "src/components/layout/Layout.tsx",
                    "code": "import React from 'react';\nimport { Header } from './Header';\nimport { Footer } from './Footer';\nimport { Sidebar } from './Sidebar';\n\ninterface LayoutProps {\n  children: React.ReactNode;\n  showSidebar?: boolean;\n  title?: string;\n}\n\nexport const Layout: React.FC<LayoutProps> = ({ \n  children, \n  showSidebar = false, \n  title \n}) => {\n  return (\n    <div className=\"min-h-screen bg-gray-50 flex flex-col\">\n      <Header title={title} />\n      <div className=\"flex-1 flex\">\n        {showSidebar && <Sidebar />}\n        <main className=\"flex-1 p-6\">\n          {children}\n        </main>\n      </div>\n      <Footer />\n    </div>\n  );\n};"
                }
            }),
            "Vue 3" => json!({
                "Header": {
                    "file": "src/components/layout/Header.vue",
                    "code": "<template>\n  <header class=\"bg-white shadow-sm border-b\">\n    <div class=\"max-w-7xl mx-auto px-4 sm:px-6 lg:px-8\">\n      <div class=\"flex justify-between items-center h-16\">\n        <div class=\"flex items-center\">\n          <router-link to=\"/\" class=\"text-xl font-bold text-gray-900\">\n            {{ title }}\n          </router-link>\n        </div>\n        <div v-if=\"showAuth\" class=\"flex items-center space-x-4\">\n          <template v-if=\"user\">\n            <span class=\"text-gray-700\">Welcome, {{ user.name }}</span>\n            <button\
                      @click=\"logout\"\
                      class=\"text-gray-500 hover:text-gray-700\"\
                    >\
                      Logout\
                    </button>\
                  </template>\
                  <router-link\
                    v-else\
                    to=\"/login\"\
                    class=\"text-blue-600 hover:text-blue-800\"\
                  >\
                    Login\
                  </router-link>\
                </div>\
              </div>\
            </div>\
          </header>\
        </template>\
        \
        <script setup lang=\"ts\">\
        import { computed } from 'vue';\
        import { useAuthStore } from '@/stores/auth';\
        \
        interface Props {\
          title?: string;\
          showAuth?: boolean;\
        }\
        \
        withDefaults(defineProps<Props>(), {\
          title: 'Brain AI',\
          showAuth: true,\
        });\
        \
        const authStore = useAuthStore();\
        const user = computed(() => authStore.user);\
        const logout = () => authStore.logout();\
        </script>"
                }
            }),
            _ => json!({"message": "Framework not supported for component generation"})
        }
    }

    /// Generate navigation components
    fn generate_navigation_components(&self, _ui_components: &Value, framework: &str) -> Value {
        match framework {
            "React" => json!({
                "Sidebar": {
                    "file": "src/components/navigation/Sidebar.tsx",
                    "code": "import React from 'react';\nimport { Link, useLocation } from 'react-router-dom';\nimport { clsx } from 'clsx';\n\ninterface NavItem {\n  name: string;\n  path: string;\n  icon?: string;\n}\n\nconst navItems: NavItem[] = [\n  { name: 'Dashboard', path: '/', icon: '🏠' },\n  { name: 'Projects', path: '/projects', icon: '📁' },\n  { name: 'Settings', path: '/settings', icon: '⚙️' },\n];\n\nexport const Sidebar: React.FC = () => {\n  const location = useLocation();\n\n  return (\n    <nav className=\"w-64 bg-white shadow-sm border-r\">\n      <div className=\"p-4\">\n        <h2 className=\"text-lg font-semibold text-gray-900 mb-4\">Navigation</h2>\n        <ul className=\"space-y-2\">\n          {navItems.map((item) => (\n            <li key={item.path}>\n              <Link\
                      to={item.path}\
                      className={clsx(\
                        'flex items-center px-3 py-2 rounded-md text-sm font-medium transition-colors',\
                        location.pathname === item.path\
                          ? 'bg-blue-100 text-blue-700'\
                          : 'text-gray-600 hover:bg-gray-100 hover:text-gray-900'\
                      )}\
                    >\
                      {item.icon && <span className=\"mr-3\">{item.icon}</span>}\
                      {item.name}\
                    </Link>\
                  </li>\
                ))}\
              </ul>\
            </div>\
          </nav>\
        );\
      };"
                }
            }),
            "Vue 3" => json!({
                "Sidebar": {
                    "file": "src/components/navigation/Sidebar.vue", 
                    "code": "<template>\n  <nav class=\"w-64 bg-white shadow-sm border-r\">\n    <div class=\"p-4\">\n      <h2 class=\"text-lg font-semibold text-gray-900 mb-4\">Navigation</h2>\n      <ul class=\"space-y-2\">\n        <li v-for=\"item in navItems\" :key=\"item.path\">\n          <router-link\
                      :to=\"item.path\"\
                      class=\"flex items-center px-3 py-2 rounded-md text-sm font-medium transition-colors\"\
                      :class=\"{\
                        'bg-blue-100 text-blue-700': $route.path === item.path,\
                        'text-gray-600 hover:bg-gray-100 hover:text-gray-900': $route.path !== item.path\
                      }\"\
                    >\
                      <span v-if=\"item.icon\" class=\"mr-3\">{{ item.icon }}</span>\
                      {{ item.name }}\
                    </router-link>\
                  </li>\
                </ul>\
              </div>\
            </nav>\
          </template>\
          \
          <script setup lang=\"ts\">\
          interface NavItem {\
            name: string;\
            path: string;\
            icon?: string;\
          }\
          \
          const navItems: NavItem[] = [\
            { name: 'Dashboard', path: '/', icon: '🏠' },\
            { name: 'Projects', path: '/projects', icon: '📁' },\
            { name: 'Settings', path: '/settings', icon: '⚙️' },\
          ];\
          </script>"
                }
            }),
            _ => json!({"message": "Framework not supported"})
        }
    }

    /// Generate form components
    fn generate_form_components(&self, _ui_components: &Value, framework: &str) -> Value {
        match framework {
            "React" => json!({
                "LoginForm": {
                    "file": "src/components/forms/LoginForm.tsx",
                    "code": "import React, { useState } from 'react';\nimport { useAuth } from '@/hooks/useAuth';\nimport { Button } from '@/components/ui/Button';\nimport { Input } from '@/components/ui/Input';\n\nexport const LoginForm: React.FC = () => {\n  const [email, setEmail] = useState('');\n  const [password, setPassword] = useState('');\n  const [loading, setLoading] = useState(false);\n  const { login } = useAuth();\n\n  const handleSubmit = async (e: React.FormEvent) => {\n    e.preventDefault();\n    setLoading(true);\n    try {\n      await login(email, password);\n    } catch (error) {\n      console.error('Login failed:', error);\n    } finally {\n      setLoading(false);\n    }\n  };\n\n  return (\n    <form onSubmit={handleSubmit} className=\"space-y-4\">\n      <div>\n        <label htmlFor=\"email\" className=\"block text-sm font-medium text-gray-700\">\n          Email\n        </label>\n        <Input\
                      id=\"email\"\
                      type=\"email\"\
                      value={email}\
                      onChange={(e) => setEmail(e.target.value)}\
                      required\
                      className=\"mt-1\"\
                    />\
                  </div>\
                  <div>\
                    <label htmlFor=\"password\" className=\"block text-sm font-medium text-gray-700\">\
                      Password\
                    </label>\
                    <Input\
                      id=\"password\"\
                      type=\"password\"\
                      value={password}\
                      onChange={(e) => setPassword(e.target.value)}\
                      required\
                      className=\"mt-1\"\
                    />\
                  </div>\
                  <Button\
                    type=\"submit\"\
                    disabled={loading}\
                    className=\"w-full\"\
                  >\
                    {loading ? 'Signing in...' : 'Sign in'}\
                  </Button>\
                </form>\
              );\
            };"
                }
            }),
            _ => json!({"forms": "Generated for specified framework"})
        }
    }

    /// Generate data display components  
    fn generate_data_display_components(&self, _ui_components: &Value, framework: &str) -> Value {
        match framework {
            "React" => json!({
                "DataTable": {
                    "file": "src/components/data/DataTable.tsx",
                    "code": "import React from 'react';\n\ninterface Column<T> {\n  key: keyof T;\n  title: string;\n  render?: (value: any, record: T) => React.ReactNode;\n}\n\ninterface DataTableProps<T> {\n  data: T[];\n  columns: Column<T>[];\n  loading?: boolean;\n  onRowClick?: (record: T) => void;\n}\n\nexport function DataTable<T extends Record<string, any>>({\n  data,\n  columns,\n  loading = false,\n  onRowClick,\n}: DataTableProps<T>) {\n  if (loading) {\n    return (\n      <div className=\"flex justify-center items-center h-32\">\n        <div className=\"animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600\"></div>\n      </div>\n    );\n  }\n\n  return (\n    <div className=\"overflow-x-auto\">\n      <table className=\"min-w-full divide-y divide-gray-200\">\n        <thead className=\"bg-gray-50\">\n          <tr>\n            {columns.map((column) => (\n              <th\
                    key={String(column.key)}\
                    className=\"px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider\"\
                  >\
                    {column.title}\
                  </th>\
                ))}\
              </tr>\n        </thead>\n        <tbody className=\"bg-white divide-y divide-gray-200\">\n          {data.map((record, index) => (\n            <tr\
                    key={index}\
                    onClick={() => onRowClick?.(record)}\
                    className={onRowClick ? 'cursor-pointer hover:bg-gray-50' : ''}\
                  >\
                    {columns.map((column) => (\n                      <td key={String(column.key)} className=\"px-6 py-4 whitespace-nowrap text-sm text-gray-900\">\
                        {column.render\
                          ? column.render(record[column.key], record)\
                          : record[column.key]\
                        }\
                      </td>\
                    ))}\
                  </tr>\
                ))}\
              </tbody>\
            </table>\
          </div>\
        );\
      }"
                }
            }),
            _ => json!({"data_display": "Generated for specified framework"})
        }
    }

    /// Generate interactive components
    fn generate_interactive_components(&self, _ui_components: &Value, framework: &str) -> Value {
        match framework {
            "React" => json!({
                "Button": {
                    "file": "src/components/ui/Button.tsx",
                    "code": "import React from 'react';\nimport { clsx } from 'clsx';\n\ninterface ButtonProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {\n  variant?: 'primary' | 'secondary' | 'danger';\n  size?: 'sm' | 'md' | 'lg';\n  loading?: boolean;\n  children: React.ReactNode;\n}\n\nexport const Button: React.FC<ButtonProps> = ({\n  variant = 'primary',\n  size = 'md',\n  loading = false,\n  className,\n  children,\n  disabled,\n  ...props\n}) => {\n  return (\n    <button\
                      className={clsx(\
                        'inline-flex items-center justify-center font-medium rounded-md transition-colors',\
                        'focus:outline-none focus:ring-2 focus:ring-offset-2',\
                        {\
                          'px-3 py-1.5 text-sm': size === 'sm',\
                          'px-4 py-2 text-sm': size === 'md',\
                          'px-6 py-3 text-base': size === 'lg',\
                        },\
                        {\
                          'bg-blue-600 text-white hover:bg-blue-700 focus:ring-blue-500': variant === 'primary',\
                          'bg-gray-200 text-gray-900 hover:bg-gray-300 focus:ring-gray-500': variant === 'secondary',\
                          'bg-red-600 text-white hover:bg-red-700 focus:ring-red-500': variant === 'danger',\
                        },\
                        {\
                          'opacity-50 cursor-not-allowed': disabled || loading,\
                        },\
                        className\
                      )}\
                      disabled={disabled || loading}\
                      {...props}\
                    >\
                      {loading && (\n                        <svg className=\"animate-spin -ml-1 mr-2 h-4 w-4\" fill=\"none\" viewBox=\"0 0 24 24\">\n                          <circle className=\"opacity-25\" cx=\"12\" cy=\"12\" r=\"10\" stroke=\"currentColor\" strokeWidth=\"4\"></circle>\n                          <path className=\"opacity-75\" fill=\"currentColor\" d=\"M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z\"></path>\n                        </svg>\n                      )}\
                      {children}\
                    </button>\
                  );\
                };"
                },
                "Input": {
                    "file": "src/components/ui/Input.tsx", 
                    "code": "import React from 'react';\nimport { clsx } from 'clsx';\n\ninterface InputProps extends React.InputHTMLAttributes<HTMLInputElement> {\n  error?: string;\n  label?: string;\n}\n\nexport const Input: React.FC<InputProps> = ({\n  className,\n  error,\n  label,\n  id,\n  ...props\n}) => {\n  return (\n    <div className=\"w-full\">\n      {label && (\n        <label htmlFor={id} className=\"block text-sm font-medium text-gray-700 mb-1\">\n          {label}\n        </label>\n      )}\n      <input\
                      id={id}\
                      className={clsx(\
                        'block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm',\
                        'placeholder-gray-400 focus:outline-none focus:ring-blue-500 focus:border-blue-500',\
                        'sm:text-sm',\
                        {\
                          'border-red-300 focus:ring-red-500 focus:border-red-500': error,\
                        },\
                        className\
                      )}\
                      {...props}\
                    />\
                    {error && (\n                      <p className=\"mt-1 text-sm text-red-600\">{error}</p>\n                    )}\
                  </div>\
                );\
              };"
                }
            }),
            _ => json!({"interactive": "Generated for specified framework"})
        }
    }

    /// Generate utility components
    fn generate_utility_components(&self, framework: &str) -> Value {
        match framework {
            "React" => json!({
                "LoadingSpinner": {
                    "file": "src/components/ui/LoadingSpinner.tsx",
                    "code": "import React from 'react';\nimport { clsx } from 'clsx';\n\ninterface LoadingSpinnerProps {\n  size?: 'sm' | 'md' | 'lg';\n  className?: string;\n}\n\nexport const LoadingSpinner: React.FC<LoadingSpinnerProps> = ({\n  size = 'md',\n  className,\n}) => {\n  return (\n    <div\
                      className={clsx(\
                        'animate-spin rounded-full border-b-2 border-blue-600',\
                        {\
                          'h-4 w-4': size === 'sm',\
                          'h-8 w-8': size === 'md',\
                          'h-12 w-12': size === 'lg',\
                        },\
                        className\
                      )}\
                    ></div>\
                  );\
                };"
                },
                "ErrorBoundary": {
                    "file": "src/components/ui/ErrorBoundary.tsx",
                    "code": "import React, { Component, ReactNode } from 'react';\n\ninterface Props {\n  children: ReactNode;\n  fallback?: ReactNode;\n}\n\ninterface State {\n  hasError: boolean;\n  error?: Error;\n}\n\nexport class ErrorBoundary extends Component<Props, State> {\n  constructor(props: Props) {\n    super(props);\n    this.state = { hasError: false };\n  }\n\n  static getDerivedStateFromError(error: Error): State {\n    return { hasError: true, error };\n  }\n\n  componentDidCatch(error: Error, errorInfo: React.ErrorInfo) {\n    console.error('ErrorBoundary caught an error:', error, errorInfo);\n  }\n\n  render() {\n    if (this.state.hasError) {\n      return (\n        this.props.fallback || (\n          <div className=\"min-h-screen flex items-center justify-center bg-gray-50\">\n            <div className=\"max-w-md w-full bg-white shadow-lg rounded-lg p-6\">\n              <div className=\"flex\">\n                <div className=\"flex-shrink-0\">\n                  <svg className=\"h-6 w-6 text-red-400\" fill=\"none\" viewBox=\"0 0 24 24\" stroke=\"currentColor\">\n                    <path strokeLinecap=\"round\" strokeLinejoin=\"round\" strokeWidth=\"2\" d=\"M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.732 16.5c-.77.833.192 2.5 1.732 2.5z\" />\n                  </svg>\n                </div>\n                <div className=\"ml-3\">\n                  <h3 className=\"text-sm font-medium text-gray-800\">\n                    Something went wrong\n                  </h3>\n                  <div className=\"mt-2 text-sm text-gray-500\">\n                    <p>We're sorry, but something unexpected happened. Please try refreshing the page.</p>\n                  </div>\n                  <div className=\"mt-4\">\n                    <button\
                      onClick={() => window.location.reload()}\
                      className=\"bg-red-600 text-white px-4 py-2 rounded-md text-sm font-medium hover:bg-red-700\"\
                    >\
                      Refresh Page\
                    </button>\
                  </div>\n                </div>\n              </div>\n            </div>\n          </div>\n        )\n      );\n    }\n\n    return this.props.children;\n  }\n}"
                }
            }),
            _ => json!({"utility": "Generated for specified framework"})
        }
    }

    /// Generate routing configuration
    fn generate_routing_configuration(&self, ui_design: &Value, framework: &str) -> Value {
        let empty_pages = json!([]);
        let _pages = ui_design.get("pages").unwrap_or(&empty_pages);
        
        match framework {
            "React" => json!({
                "router_setup": {
                    "file": "src/App.tsx",
                    "code": "import React from 'react';\nimport { BrowserRouter as Router, Routes, Route } from 'react-router-dom';\nimport { Layout } from './components/layout/Layout';\nimport { HomePage } from './pages/HomePage';\nimport { LoginPage } from './pages/LoginPage';\nimport { ProjectsPage } from './pages/ProjectsPage';\nimport { SettingsPage } from './pages/SettingsPage';\nimport { NotFoundPage } from './pages/NotFoundPage';\nimport { ErrorBoundary } from './components/ui/ErrorBoundary';\n\nexport const App: React.FC = () => {\n  return (\n    <ErrorBoundary>\n      <Router>\n        <Layout>\n          <Routes>\n            <Route path=\"/\" element={<HomePage />} />\n            <Route path=\"/login\" element={<LoginPage />} />\n            <Route path=\"/projects\" element={<ProjectsPage />} />\n            <Route path=\"/settings\" element={<SettingsPage />} />\n            <Route path=\"*\" element={<NotFoundPage />} />\n          </Routes>\n        </Layout>\n      </Router>\n    </ErrorBoundary>\n  );\n};"
                },
                "protected_routes": {
                    "file": "src/components/routing/ProtectedRoute.tsx",
                    "code": "import React from 'react';\nimport { Navigate } from 'react-router-dom';\nimport { useAuth } from '@/hooks/useAuth';\n\ninterface ProtectedRouteProps {\n  children: React.ReactNode;\n  redirectTo?: string;\n}\n\nexport const ProtectedRoute: React.FC<ProtectedRouteProps> = ({\n  children,\n  redirectTo = '/login',\n}) => {\n  const { isAuthenticated } = useAuth();\n\n  if (!isAuthenticated) {\n    return <Navigate to={redirectTo} replace />;\n  }\n\n  return <>{children}</>;\n};"
                }
            }),
            "Vue 3" => json!({
                "router_setup": {
                    "file": "src/router/index.ts",
                    "code": "import { createRouter, createWebHistory } from 'vue-router';\nimport { useAuthStore } from '@/stores/auth';\nimport HomePage from '@/views/HomePage.vue';\nimport LoginPage from '@/views/LoginPage.vue';\nimport ProjectsPage from '@/views/ProjectsPage.vue';\nimport SettingsPage from '@/views/SettingsPage.vue';\nimport NotFoundPage from '@/views/NotFoundPage.vue';\n\nconst router = createRouter({\n  history: createWebHistory(),\n  routes: [\n    {\n      path: '/',\n      name: 'Home',\n      component: HomePage,\n    },\n    {\n      path: '/login',\n      name: 'Login',\n      component: LoginPage,\n    },\n    {\n      path: '/projects',\n      name: 'Projects',\n      component: ProjectsPage,\n      meta: { requiresAuth: true },\n    },\n    {\n      path: '/settings',\n      name: 'Settings',\n      component: SettingsPage,\n      meta: { requiresAuth: true },\n    },\n    {\n      path: '/:pathMatch(.*)*',\n      name: 'NotFound',\n      component: NotFoundPage,\n    },\n  ],\n});\n\nrouter.beforeEach((to, from, next) => {\n  const authStore = useAuthStore();\n  if (to.meta.requiresAuth && !authStore.isAuthenticated) {\n    next('/login');\n  } else {\n    next();\n  }\n});\n\nexport default router;"
                }
            }),
            _ => json!({"routing": "Generated for specified framework"})
        }
    }

    /// Generate state management setup
    fn generate_state_management(&self, _ui_design: &Value, _api_specs: &Value, framework: &str) -> Value {
        match framework {
            "React" => json!({
                "auth_store": {
                    "file": "src/hooks/useAuth.ts",
                    "code": "import { useState, useEffect, useContext, createContext, ReactNode } from 'react';\nimport { api } from '@/lib/api';\n\ninterface User {\n  id: string;\n  email: string;\n  name: string;\n}\n\ninterface AuthContextType {\n  user: User | null;\n  login: (email: string, password: string) => Promise<void>;\n  logout: () => void;\n  isAuthenticated: boolean;\n  loading: boolean;\n}\n\nconst AuthContext = createContext<AuthContextType | undefined>(undefined);\n\nexport const AuthProvider: React.FC<{ children: ReactNode }> = ({ children }) => {\n  const [user, setUser] = useState<User | null>(null);\n  const [loading, setLoading] = useState(true);\n\n  useEffect(() => {\n    const token = localStorage.getItem('auth_token');\n    if (token) {\n      // Verify token and fetch user\n      api.get('/auth/me')\n        .then(response => setUser(response.data))\n        .catch(() => localStorage.removeItem('auth_token'))\n        .finally(() => setLoading(false));\n    } else {\n      setLoading(false);\n    }\n  }, []);\n\n  const login = async (email: string, password: string) => {\n    const response = await api.post('/auth/login', { email, password });\n    const { token, user: userData } = response.data;\n    localStorage.setItem('auth_token', token);\n    setUser(userData);\n  };\n\n  const logout = () => {\n    localStorage.removeItem('auth_token');\n    setUser(null);\n  };\n\n  const value = {\n    user,\n    login,\n    logout,\n    isAuthenticated: !!user,\n    loading,\n  };\n\n  return <AuthContext.Provider value={value}>{children}</AuthContext.Provider>;\n};\n\nexport const useAuth = () => {\n  const context = useContext(AuthContext);\n  if (context === undefined) {\n    throw new Error('useAuth must be used within an AuthProvider');\n  }\n  return context;\n};"
                }
            }),
            "Vue 3" => json!({
                "auth_store": {
                    "file": "src/stores/auth.ts",
                    "code": "import { defineStore } from 'pinia';\nimport { ref, computed } from 'vue';\nimport { api } from '@/lib/api';\n\ninterface User {\n  id: string;\n  email: string;\n  name: string;\n}\n\nexport const useAuthStore = defineStore('auth', () => {\n  const user = ref<User | null>(null);\n  const loading = ref(false);\n\n  const isAuthenticated = computed(() => !!user.value);\n\n  const login = async (email: string, password: string) => {\n    loading.value = true;\n    try {\n      const response = await api.post('/auth/login', { email, password });\n      const { token, user: userData } = response.data;\n      localStorage.setItem('auth_token', token);\n      user.value = userData;\n    } finally {\n      loading.value = false;\n    }\n  };\n\n  const logout = () => {\n    localStorage.removeItem('auth_token');\n    user.value = null;\n  };\n\n  const fetchUser = async () => {\n    const token = localStorage.getItem('auth_token');\n    if (token) {\n      try {\n        const response = await api.get('/auth/me');\n        user.value = response.data;\n      } catch (error) {\n        localStorage.removeItem('auth_token');\n      }\n    }\n  };\n\n  return {\n    user,\n    loading,\n    isAuthenticated,\n    login,\n    logout,\n    fetchUser,\n  };\n});"
                }
            }),
            _ => json!({"state_management": "Generated for specified framework"})
        }
    }

    /// Generate API integration layer
    fn generate_api_integration_layer(&self, _api_specs: &Value, framework: &str) -> Value {
        match framework {
            "React" | "Vue 3" => json!({
                "api_client": {
                    "file": "src/lib/api.ts",
                    "code": "import axios, { AxiosInstance, AxiosRequestConfig, AxiosResponse } from 'axios';\n\nclass ApiClient {\n  private client: AxiosInstance;\n\n  constructor(baseURL: string = process.env.VITE_API_URL || '/api') {\n    this.client = axios.create({\n      baseURL,\n      timeout: 10000,\n      headers: {\n        'Content-Type': 'application/json',\n      },\n    });\n\n    this.setupInterceptors();\n  }\n\n  private setupInterceptors() {\n    // Request interceptor to add auth token\n    this.client.interceptors.request.use(\n      (config) => {\n        const token = localStorage.getItem('auth_token');\n        if (token) {\n          config.headers.Authorization = `Bearer ${token}`;\n        }\n        return config;\n      },\n      (error) => Promise.reject(error)\n    );\n\n    // Response interceptor for error handling\n    this.client.interceptors.response.use(\n      (response) => response,\n      (error) => {\n        if (error.response?.status === 401) {\n          localStorage.removeItem('auth_token');\n          window.location.href = '/login';\n        }\n        return Promise.reject(error);\n      }\n    );\n  }\n\n  async get<T = any>(url: string, config?: AxiosRequestConfig): Promise<AxiosResponse<T>> {\n    return this.client.get(url, config);\n  }\n\n  async post<T = any>(url: string, data?: any, config?: AxiosRequestConfig): Promise<AxiosResponse<T>> {\n    return this.client.post(url, data, config);\n  }\n\n  async put<T = any>(url: string, data?: any, config?: AxiosRequestConfig): Promise<AxiosResponse<T>> {\n    return this.client.put(url, data, config);\n  }\n\n  async delete<T = any>(url: string, config?: AxiosRequestConfig): Promise<AxiosResponse<T>> {\n    return this.client.delete(url, config);\n  }\n}\n\nexport const api = new ApiClient();"
                },
                "api_hooks": {
                    "file": "src/hooks/useApi.ts",
                    "code": "import { useState, useEffect } from 'react';\nimport { api } from '@/lib/api';\n\ninterface UseApiOptions {\n  immediate?: boolean;\n}\n\ninterface UseApiReturn<T> {\n  data: T | null;\n  loading: boolean;\n  error: string | null;\n  execute: (...args: any[]) => Promise<void>;\n}\n\nexport function useApi<T = any>(\n  apiCall: (...args: any[]) => Promise<{ data: T }>,\n  options: UseApiOptions = {}\n): UseApiReturn<T> {\n  const [data, setData] = useState<T | null>(null);\n  const [loading, setLoading] = useState(false);\n  const [error, setError] = useState<string | null>(null);\n\n  const execute = async (...args: any[]) => {\n    setLoading(true);\n    setError(null);\n    try {\n      const response = await apiCall(...args);\n      setData(response.data);\n    } catch (err: any) {\n      setError(err.response?.data?.message || err.message || 'An error occurred');\n    } finally {\n      setLoading(false);\n    }\n  };\n\n  useEffect(() => {\n    if (options.immediate) {\n      execute();\n    }\n  }, []);\n\n  return { data, loading, error, execute };\n}"
                }
            }),
            _ => json!({"api_integration": "Generated for specified framework"})
        }
    }

    /// Generate styling system
    fn generate_styling_system(&self, _ui_design: &Value, _framework: &str) -> Value {
        json!({
            "tailwind_config": {
                "file": "tailwind.config.js",
                "code": "/** @type {import('tailwindcss').Config} */\nmodule.exports = {\n  content: [\n    './index.html',\n    './src/**/*.{js,ts,jsx,tsx,vue}',\n  ],\n  theme: {\n    extend: {\n      colors: {\n        primary: {\n          50: '#eff6ff',\n          100: '#dbeafe',\n          500: '#3b82f6',\n          600: '#2563eb',\n          700: '#1d4ed8',\n        },\n        gray: {\n          50: '#f9fafb',\n          100: '#f3f4f6',\n          500: '#6b7280',\n          700: '#374151',\n          900: '#111827',\n        },\n      },\n      fontFamily: {\n        sans: ['Inter', 'system-ui', 'sans-serif'],\n      },\n    },\n  },\n  plugins: [\n    require('@tailwindcss/forms'),\n    require('@tailwindcss/typography'),\n  ],\n};"
            },
            "global_styles": {
                "file": "src/styles/globals.css",
                "code": "@tailwind base;\n@tailwind components;\n@tailwind utilities;\n\n@layer base {\n  html {\n    font-family: Inter, system-ui, sans-serif;\n  }\n  \n  body {\n    @apply bg-gray-50 text-gray-900;\n  }\n}\n\n@layer components {\n  .btn {\n    @apply inline-flex items-center justify-center px-4 py-2 text-sm font-medium rounded-md transition-colors;\n    @apply focus:outline-none focus:ring-2 focus:ring-offset-2;\n  }\n  \n  .btn-primary {\n    @apply btn bg-primary-600 text-white hover:bg-primary-700 focus:ring-primary-500;\n  }\n  \n  .btn-secondary {\n    @apply btn bg-gray-200 text-gray-900 hover:bg-gray-300 focus:ring-gray-500;\n  }\n}\n\n@layer utilities {\n  .text-balance {\n    text-wrap: balance;\n  }\n}"
            }
        })
    }

    /// Generate accessibility features
    fn generate_accessibility_features(&self, _ui_design: &Value, _framework: &str) -> Value {
        json!({
            "aria_helpers": {
                "file": "src/utils/accessibility.ts",
                "code": "// ARIA utility functions for enhanced accessibility\n\nexport const generateId = (prefix: string = 'element'): string => {\n  return `${prefix}-${Math.random().toString(36).substr(2, 9)}`;\n};\n\nexport const announceToScreenReader = (message: string): void => {\n  const announcement = document.createElement('div');\n  announcement.setAttribute('aria-live', 'polite');\n  announcement.setAttribute('aria-atomic', 'true');\n  announcement.setAttribute('class', 'sr-only');\n  announcement.textContent = message;\n  \n  document.body.appendChild(announcement);\n  \n  setTimeout(() => {\n    document.body.removeChild(announcement);\n  }, 1000);\n};\n\nexport const trapFocus = (element: HTMLElement): (() => void) => {\n  const focusableElements = element.querySelectorAll(\n    'button, [href], input, select, textarea, [tabindex]:not([tabindex=\"-1\"])'\n  );\n  \n  const firstElement = focusableElements[0] as HTMLElement;\n  const lastElement = focusableElements[focusableElements.length - 1] as HTMLElement;\n  \n  const handleTabKey = (e: KeyboardEvent) => {\n    if (e.key === 'Tab') {\n      if (e.shiftKey) {\n        if (document.activeElement === firstElement) {\n          lastElement.focus();\n          e.preventDefault();\n        }\n      } else {\n        if (document.activeElement === lastElement) {\n          firstElement.focus();\n          e.preventDefault();\n        }\n      }\n    }\n  };\n  \n  element.addEventListener('keydown', handleTabKey);\n  \n  return () => {\n    element.removeEventListener('keydown', handleTabKey);\n  };\n};"
            },
            "skip_navigation": {
                "file": "src/components/accessibility/SkipNavigation.tsx",
                "code": "import React from 'react';\n\nexport const SkipNavigation: React.FC = () => {\n  return (\n    <a\n      href=\"#main-content\"\
      className=\"sr-only focus:not-sr-only focus:absolute focus:top-4 focus:left-4 focus:z-50 focus:px-4 focus:py-2 focus:bg-blue-600 focus:text-white focus:rounded-md\"\
    >\
      Skip to main content\
    </a>\
  );\
};"
            }
        })
    }

    /// Generate project structure
    fn generate_project_structure(&self, framework: &str) -> Value {
        match framework {
            "React" => json!({
                "structure": {
                    "src/": {
                        "components/": {
                            "layout/": ["Header.tsx", "Footer.tsx", "Layout.tsx", "Sidebar.tsx"],
                            "navigation/": ["Sidebar.tsx", "Breadcrumbs.tsx"],
                            "forms/": ["LoginForm.tsx", "ContactForm.tsx"],
                            "ui/": ["Button.tsx", "Input.tsx", "Modal.tsx", "LoadingSpinner.tsx"],
                            "accessibility/": ["SkipNavigation.tsx"]
                        },
                        "pages/": ["HomePage.tsx", "LoginPage.tsx", "ProjectsPage.tsx", "SettingsPage.tsx"],
                        "hooks/": ["useAuth.ts", "useApi.ts"],
                        "lib/": ["api.ts", "utils.ts"],
                        "styles/": ["globals.css"],
                        "utils/": ["accessibility.ts"],
                        "types/": ["index.ts"]
                    },
                    "public/": ["index.html", "favicon.ico"],
                    "config": ["vite.config.ts", "tailwind.config.js", "tsconfig.json"]
                }
            }),
            "Vue 3" => json!({
                "structure": {
                    "src/": {
                        "components/": {
                            "layout/": ["Header.vue", "Footer.vue", "Layout.vue"],
                            "navigation/": ["Sidebar.vue", "Breadcrumbs.vue"],
                            "forms/": ["LoginForm.vue", "ContactForm.vue"],
                            "ui/": ["Button.vue", "Input.vue", "Modal.vue"]
                        },
                        "views/": ["HomePage.vue", "LoginPage.vue", "ProjectsPage.vue"],
                        "stores/": ["auth.ts", "projects.ts"],
                        "router/": ["index.ts"],
                        "lib/": ["api.ts"],
                        "styles/": ["globals.css"],
                        "utils/": ["accessibility.ts"],
                        "types/": ["index.ts"]
                    },
                    "public/": ["index.html", "favicon.ico"],
                    "config": ["vite.config.ts", "tailwind.config.js", "tsconfig.json"]
                }
            }),
            _ => json!({"structure": "Generated for specified framework"})
        }
    }

    /// Generate build configuration
    fn generate_build_configuration(&self, framework: &str) -> Value {
        match framework {
            "React" => json!({
                "vite_config": {
                    "file": "vite.config.ts",
                    "code": "import { defineConfig } from 'vite';\nimport react from '@vitejs/plugin-react';\nimport path from 'path';\n\nexport default defineConfig({\n  plugins: [react()],\n  resolve: {\n    alias: {\n      '@': path.resolve(__dirname, './src'),\n    },\n  },\n  server: {\n    port: 3000,\n    proxy: {\n      '/api': {\n        target: 'http://localhost:8000',\n        changeOrigin: true,\n      },\n    },\n  },\n  build: {\n    outDir: 'dist',\n    sourcemap: true,\n    rollupOptions: {\n      output: {\n        manualChunks: {\n          vendor: ['react', 'react-dom'],\n          router: ['react-router-dom'],\n        },\n      },\n    },\n  },\n});"
                }
            }),
            "Vue 3" => json!({
                "vite_config": {
                    "file": "vite.config.ts",
                    "code": "import { defineConfig } from 'vite';\nimport vue from '@vitejs/plugin-vue';\nimport path from 'path';\n\nexport default defineConfig({\n  plugins: [vue()],\n  resolve: {\n    alias: {\n      '@': path.resolve(__dirname, './src'),\n    },\n  },\n  server: {\n    port: 3000,\n    proxy: {\n      '/api': {\n        target: 'http://localhost:8000',\n        changeOrigin: true,\n      },\n    },\n  },\n  build: {\n    outDir: 'dist',\n    sourcemap: true,\n  },\n});"
                }
            }),
            _ => json!({"build_config": "Generated for specified framework"})
        }
    }

    /// Generate package dependencies
    fn generate_package_dependencies(&self, framework: &str) -> Value {
        match framework {
            "React" => json!({
                "package_json": {
                    "dependencies": {
                        "react": "^18.2.0",
                        "react-dom": "^18.2.0",
                        "react-router-dom": "^6.8.0",
                        "axios": "^1.3.0",
                        "clsx": "^1.2.1"
                    },
                    "devDependencies": {
                        "@types/react": "^18.0.0",
                        "@types/react-dom": "^18.0.0",
                        "@vitejs/plugin-react": "^3.1.0",
                        "vite": "^4.1.0",
                        "typescript": "^4.9.0",
                        "tailwindcss": "^3.2.0",
                        "@tailwindcss/forms": "^0.5.0",
                        "@tailwindcss/typography": "^0.5.0",
                        "autoprefixer": "^10.4.0",
                        "postcss": "^8.4.0"
                    }
                }
            }),
            "Vue 3" => json!({
                "package_json": {
                    "dependencies": {
                        "vue": "^3.2.0",
                        "vue-router": "^4.1.0",
                        "pinia": "^2.0.0",
                        "axios": "^1.3.0"
                    },
                    "devDependencies": {
                        "@vitejs/plugin-vue": "^4.0.0",
                        "vite": "^4.1.0",
                        "typescript": "^4.9.0",
                        "vue-tsc": "^1.0.0",
                        "tailwindcss": "^3.2.0",
                        "@tailwindcss/forms": "^0.5.0",
                        "@tailwindcss/typography": "^0.5.0",
                        "autoprefixer": "^10.4.0",
                        "postcss": "^8.4.0"
                    }
                }
            }),
            _ => json!({"dependencies": "Generated for specified framework"})
        }
    }
}

#[async_trait]
impl BrainAgent for FrontendCoder {
    async fn execute(
        &self,
        input: AgentInput,
        context: &CognitiveContext,
    ) -> BrainResult<AgentOutput> {
        let start_time = std::time::Instant::now();
        
        // Parse input based on content type
        let parsed_input = match serde_json::from_str::<Value>(&input.content) {
            Ok(value) => value,
            Err(_) => {
                // Fallback: try to parse as simple string and wrap in object
                json!({ "content": input.content })
            }
        };

        // Extract UI design and API specifications from input
        let empty_json = json!({});
        let ui_design_specs = parsed_input.get("ui_design_specifications")
            .or_else(|| parsed_input.get("ui_design"))
            .or_else(|| parsed_input.get("design"))
            .unwrap_or(&empty_json);
            
        let api_specifications = parsed_input.get("api_specifications")
            .or_else(|| parsed_input.get("api_specs"))
            .or_else(|| parsed_input.get("api"))
            .unwrap_or(&empty_json);

        // Generate comprehensive frontend codebase
        let frontend_codebase = self.generate_frontend_codebase(
            ui_design_specs,
            api_specifications,
            context
        ).await?;

        // Generate testing implementation
        let testing_implementation = self.generate_testing_implementation(ui_design_specs, api_specifications);
        
        // Generate performance optimization strategies
        let performance_optimization = self.generate_performance_optimization_strategies();
        
        // Calculate confidence based on input quality and completeness
        let mut confidence = self.metadata.base_confidence;
        
        // Adjust confidence based on input quality
        if !ui_design_specs.is_null() && ui_design_specs.as_object().map_or(false, |obj| !obj.is_empty()) {
            confidence += 0.05;
        }
        if !api_specifications.is_null() && api_specifications.as_object().map_or(false, |obj| !obj.is_empty()) {
            confidence += 0.05;
        }
        
        // Cap confidence at 0.95
        confidence = confidence.min(0.95);

        // Determine execution status
        let status = if confidence >= self.confidence_threshold() {
            ExecutionStatus::Success
        } else {
            ExecutionStatus::PartialSuccess
        };

        // Calculate execution metrics
        let execution_time = start_time.elapsed();
        let memory_usage = 20.0; // Estimated memory usage in MB

        let metadata = ExecutionMetadata {
            execution_time_ms: execution_time.as_millis() as u64,
            memory_usage_mb: memory_usage,
            api_calls: 0, // No external API calls
            status,
            warnings: vec![],
        };

        // Get framework name before moving frontend_codebase
        let framework_name = frontend_codebase.get("framework").and_then(|f| f.as_str()).unwrap_or("React");
        
        // Compile comprehensive output
        let mut output_data = HashMap::new();
        output_data.insert("frontend_codebase".to_string(), frontend_codebase.clone());
        output_data.insert("testing_implementation".to_string(), testing_implementation);
        output_data.insert("performance_optimization".to_string(), performance_optimization);
        output_data.insert("implementation_recommendations".to_string(), json!({
            "development_workflow": [
                "Set up hot module replacement for fast development",
                "Configure ESLint and Prettier for code quality",
                "Implement automated testing pipeline",
                "Use TypeScript for type safety and better developer experience"
            ],
            "deployment_strategy": "Build optimized production bundles with code splitting",
            "performance_monitoring": "Implement web vitals tracking and error boundary reporting",
            "accessibility_compliance": "Ensure WCAG 2.1 AA compliance with automated testing"
        }));
        output_data.insert("next_steps".to_string(), json!([
            "Review generated code structure and customize for specific requirements",
            "Set up development environment with recommended dependencies",
            "Implement unit tests for critical components",
            "Configure CI/CD pipeline for automated deployment"
        ]));

        let reasoning = format!(
            "Generated comprehensive frontend codebase with {} framework. \
             Included component library, routing, state management, API integration, \
             styling system, and accessibility features. Confidence: {:.1}%",
            framework_name,
            confidence * 100.0
        );

        let next_actions = vec![
            "Review generated code structure".to_string(),
            "Customize components for specific requirements".to_string(),
            "Set up development environment".to_string(),
            "Implement testing strategy".to_string(),
        ];

        Ok(AgentOutput {
            agent_id: self.metadata.id.clone(),
            output_type: "frontend_codebase".to_string(),
            content: reasoning.clone(),
            data: output_data,
            confidence,
            reasoning: Some(reasoning),
            next_actions,
            execution_metadata: metadata,
            timestamp: chrono::Utc::now(),
        })
    }

    fn metadata(&self) -> &AgentMetadata {
        &self.metadata
    }

    fn cognitive_preferences(&self) -> &CognitivePreferences {
        &self.preferences
    }

    fn confidence_threshold(&self) -> f32 {
        0.75
    }

    async fn assess_confidence(
        &self,
        _input: &AgentInput,
        _context: &CognitiveContext,
    ) -> BrainResult<f32> {
        Ok(self.metadata.base_confidence)
    }
}

impl FrontendCoder {
    /// Generate testing implementation strategies
    fn generate_testing_implementation(&self, _ui_design: &Value, _api_specs: &Value) -> Value {
        json!({
            "unit_testing": {
                "framework": "Jest + React Testing Library",
                "component_tests": {
                    "file": "src/components/__tests__/Button.test.tsx",
                    "code": "import React from 'react';\nimport { render, screen, fireEvent } from '@testing-library/react';\nimport { Button } from '../ui/Button';\n\ndescribe('Button Component', () => {\n  it('renders button with text', () => {\n    render(<Button>Click me</Button>);\n    expect(screen.getByRole('button', { name: /click me/i })).toBeInTheDocument();\n  });\n\n  it('handles click events', () => {\n    const handleClick = jest.fn();\n    render(<Button onClick={handleClick}>Click me</Button>);\n    \n    fireEvent.click(screen.getByRole('button'));\n    expect(handleClick).toHaveBeenCalledTimes(1);\n  });\n\n  it('applies correct variant styles', () => {\n    render(<Button variant=\"primary\">Primary Button</Button>);\n    const button = screen.getByRole('button');\n    expect(button).toHaveClass('bg-blue-600');\n  });\n\n  it('shows loading state', () => {\n    render(<Button loading>Loading Button</Button>);\n    expect(screen.getByRole('button')).toBeDisabled();\n    expect(screen.getByText('Loading Button')).toBeInTheDocument();\n  });\n});"
                }
            },
            "integration_testing": {
                "api_integration": {
                    "file": "src/__tests__/api-integration.test.ts",
                    "code": "import { api } from '../lib/api';\nimport { renderHook, waitFor } from '@testing-library/react';\nimport { useApi } from '../hooks/useApi';\n\n// Mock API responses\njest.mock('../lib/api');\nconst mockedApi = api as jest.Mocked<typeof api>;\n\ndescribe('API Integration', () => {\n  beforeEach(() => {\n    jest.clearAllMocks();\n  });\n\n  it('handles successful API call', async () => {\n    const mockData = { id: 1, name: 'Test User' };\n    mockedApi.get.mockResolvedValue({ data: mockData });\n\n    const { result } = renderHook(() => useApi(() => api.get('/users/1')));\n    \n    await waitFor(() => {\n      expect(result.current.data).toEqual(mockData);\n      expect(result.current.loading).toBe(false);\n      expect(result.current.error).toBeNull();\n    });\n  });\n\n  it('handles API error', async () => {\n    const errorMessage = 'Network Error';\n    mockedApi.get.mockRejectedValue(new Error(errorMessage));\n\n    const { result } = renderHook(() => useApi(() => api.get('/users/1')));\n    result.current.execute();\n\n    await waitFor(() => {\n      expect(result.current.data).toBeNull();\n      expect(result.current.loading).toBe(false);\n      expect(result.current.error).toBe(errorMessage);\n    });\n  });\n});"
                }
            },
            "e2e_testing": {
                "framework": "Playwright",
                "user_flows": {
                    "file": "tests/auth-flow.spec.ts",
                    "code": "import { test, expect } from '@playwright/test';\n\ntest.describe('Authentication Flow', () => {\n  test('user can login successfully', async ({ page }) => {\n    await page.goto('/login');\n    \n    // Fill login form\n    await page.fill('[data-testid=\"email-input\"]', 'test@example.com');\n    await page.fill('[data-testid=\"password-input\"]', 'password123');\n    \n    // Submit form\n    await page.click('[data-testid=\"login-button\"]');\n    \n    // Verify redirect to dashboard\n    await expect(page).toHaveURL('/dashboard');\n    await expect(page.locator('[data-testid=\"user-welcome\"]')).toBeVisible();\n  });\n\n  test('displays error for invalid credentials', async ({ page }) => {\n    await page.goto('/login');\n    \n    await page.fill('[data-testid=\"email-input\"]', 'invalid@example.com');\n    await page.fill('[data-testid=\"password-input\"]', 'wrongpassword');\n    await page.click('[data-testid=\"login-button\"]');\n    \n    await expect(page.locator('[data-testid=\"error-message\"]')).toBeVisible();\n    await expect(page.locator('[data-testid=\"error-message\"]')).toContainText('Invalid credentials');\n  });\n});"
                }
            },
            "accessibility_testing": {
                "automated_a11y": {
                    "file": "src/__tests__/accessibility.test.tsx",
                    "code": "import React from 'react';\nimport { render } from '@testing-library/react';\nimport { axe, toHaveNoViolations } from 'jest-axe';\nimport { App } from '../App';\n\nexpect.extend(toHaveNoViolations);\n\ndescribe('Accessibility Tests', () => {\n  it('should not have accessibility violations', async () => {\n    const { container } = render(<App />);\n    const results = await axe(container);\n    expect(results).toHaveNoViolations();\n  });\n});"
                }
            }
        })
    }

    /// Generate performance optimization strategies
    fn generate_performance_optimization_strategies(&self) -> Value {
        json!({
            "code_splitting": {
                "description": "Implement route-based code splitting for faster initial load",
                "implementation": "Use React.lazy() or Vue's defineAsyncComponent for route components"
            },
            "bundle_optimization": {
                "description": "Optimize bundle size with tree shaking and minification",
                "webpack_config": "Configure webpack to split vendor and app bundles"
            },
            "image_optimization": {
                "description": "Implement responsive images with modern formats",
                "techniques": ["WebP format", "Lazy loading", "Image compression", "Responsive sizing"]
            },
            "caching_strategy": {
                "description": "Implement effective caching for static assets and API responses",
                "implementation": [
                    "Service Worker for offline functionality",
                    "HTTP caching headers for static assets",
                    "React Query/SWR for API response caching"
                ]
            },
            "performance_monitoring": {
                "metrics": ["First Contentful Paint", "Largest Contentful Paint", "Cumulative Layout Shift"],
                "tools": ["Web Vitals API", "Lighthouse CI", "Performance Observer"]
            }
        })
    }
} 