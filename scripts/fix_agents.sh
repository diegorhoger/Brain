#!/bin/bash

# Fix designer.rs
sed -i '' '/persona: "A creative UI\/UX designer/a\
            description: "UI/UX design agent specializing in user interface design, user experience optimization, and design system creation.".to_string(),' crates/brain-cognitive/src/agents/development/designer.rs

sed -i '' '/adaptation_rate: 0\.2/a\
            creativity_level: 0.95, // Very high creativity for innovative design solutions\
            detail_level: 0.8, // High detail level for design specifications\
            collaboration_style: "creative".to_string(), // Creative collaboration style for design work' crates/brain-cognitive/src/agents/development/designer.rs

# Fix schema.rs
sed -i '' '/persona: "A database design specialist/a\
            description: "Database schema design agent specializing in data modeling, database optimization, and schema evolution strategies.".to_string(),' crates/brain-cognitive/src/agents/development/schema.rs

sed -i '' '/adaptation_rate: 0\.1/a\
            creativity_level: 0.6, // Moderate creativity for balanced schema design\
            detail_level: 0.9, // High detail level for precise schema specifications\
            collaboration_style: "analytical".to_string(), // Analytical approach for data modeling' crates/brain-cognitive/src/agents/development/schema.rs

# Fix api.rs
sed -i '' '/persona: "An API development specialist/a\
            description: "API development agent specializing in RESTful API design, GraphQL implementation, and API documentation.".to_string(),' crates/brain-cognitive/src/agents/development/api.rs

sed -i '' '/adaptation_rate: 0\.15/a\
            creativity_level: 0.7, // Balanced creativity for API design\
            detail_level: 0.85, // High detail level for API specifications\
            collaboration_style: "technical".to_string(), // Technical collaboration for API development' crates/brain-cognitive/src/agents/development/api.rs

# Fix frontend_coder.rs
sed -i '' '/persona: "A frontend development specialist/a\
            description: "Frontend development agent specializing in React, Vue.js, responsive design, and modern frontend technologies.".to_string(),' crates/brain-cognitive/src/agents/development/frontend_coder.rs

sed -i '' '/adaptation_rate: 0\.25/a\
            creativity_level: 0.85, // High creativity for frontend solutions\
            detail_level: 0.75, // Good detail level for implementation\
            collaboration_style: "iterative".to_string(), // Iterative collaboration for frontend development' crates/brain-cognitive/src/agents/development/frontend_coder.rs

# Fix backend_coder.rs
sed -i '' '/persona: "A backend development specialist/a\
            description: "Backend development agent specializing in server-side logic, database integration, and scalable backend architectures.".to_string(),' crates/brain-cognitive/src/agents/development/backend_coder.rs

sed -i '' '/adaptation_rate: 0\.2/a\
            creativity_level: 0.7, // Balanced creativity for backend solutions\
            detail_level: 0.8, // High detail level for backend implementation\
            collaboration_style: "systematic".to_string(), // Systematic approach for backend development' crates/brain-cognitive/src/agents/development/backend_coder.rs

# Fix maintainer.rs
sed -i '' '/persona: "A software maintenance specialist/a\
            description: "Software maintenance agent specializing in code maintenance, bug fixes, performance optimization, and system monitoring.".to_string(),' crates/brain-cognitive/src/agents/development/maintainer.rs

sed -i '' '/adaptation_rate: 0\.1/a\
            creativity_level: 0.5, // Moderate creativity for maintenance solutions\
            detail_level: 0.85, // High detail level for thorough maintenance\
            collaboration_style: "methodical".to_string(), // Methodical approach for maintenance work' crates/brain-cognitive/src/agents/development/maintainer.rs

echo "Fixed all development agents" 