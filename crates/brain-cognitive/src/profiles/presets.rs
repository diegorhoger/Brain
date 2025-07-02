use std::collections::HashMap;
use crate::agents::traits::{
    CognitivePreferenceProfile, InteractionMode, DetailLevel, EmotionalSensitivity,
    AutonomyLevel, CommunicationStyle, CognitiveLoadSettings, PacingPreference
};
use super::ProfilePreset;

/// Preset manager for cognitive preference profiles
pub struct PresetManager {
    /// Available presets
    presets: Vec<ProfilePreset>,
    
    /// Preset categories
    categories: HashMap<String, Vec<String>>,
}

impl PresetManager {
    /// Create a new preset manager with default presets
    pub fn new() -> Self {
        let presets = Self::create_default_presets();
        let categories = Self::create_preset_categories(&presets);
        
        Self {
            presets,
            categories,
        }
    }
    
    /// Get all available presets
    pub fn get_all_presets(&self) -> &[ProfilePreset] {
        &self.presets
    }
    
    /// Get presets by category
    pub fn get_presets_by_category(&self, category: &str) -> Vec<&ProfilePreset> {
        if let Some(preset_ids) = self.categories.get(category) {
            preset_ids.iter()
                .filter_map(|id| self.presets.iter().find(|p| p.id == *id))
                .collect()
        } else {
            Vec::new()
        }
    }
    
    /// Get preset by ID
    pub fn get_preset(&self, preset_id: &str) -> Option<&ProfilePreset> {
        self.presets.iter().find(|p| p.id == preset_id)
    }
    
    /// Get available categories
    pub fn get_categories(&self) -> Vec<&str> {
        self.categories.keys().map(|s| s.as_str()).collect()
    }
    
    /// Add a custom preset
    pub fn add_preset(&mut self, preset: ProfilePreset) {
        // Update categories if needed
        for tag in &preset.tags {
            self.categories.entry(tag.clone()).or_insert_with(Vec::new).push(preset.id.clone());
        }
        
        self.presets.push(preset);
    }
    
    /// Remove a preset
    pub fn remove_preset(&mut self, preset_id: &str) -> bool {
        if let Some(pos) = self.presets.iter().position(|p| p.id == preset_id) {
            let preset = self.presets.remove(pos);
            
            // Update categories
            for tag in &preset.tags {
                if let Some(category) = self.categories.get_mut(tag) {
                    category.retain(|id| id != preset_id);
                    if category.is_empty() {
                        self.categories.remove(tag);
                    }
                }
            }
            
            true
        } else {
            false
        }
    }
    
    /// Create default preset collection
    fn create_default_presets() -> Vec<ProfilePreset> {
        vec![
            // Beginner presets
            Self::beginner_guided_preset(),
            Self::beginner_safe_preset(),
            Self::beginner_learning_preset(),
            
            // Developer presets
            Self::developer_focused_preset(),
            Self::developer_collaborative_preset(),
            Self::developer_rapid_preset(),
            
            // Power user presets
            Self::power_user_autonomous_preset(),
            Self::power_user_efficient_preset(),
            Self::power_user_expert_preset(),
            
            // Explorer presets
            Self::explorer_curious_preset(),
            Self::explorer_experimental_preset(),
            Self::explorer_discovery_preset(),
            
            // Specialized presets
            Self::security_focused_preset(),
            Self::data_analyst_preset(),
            Self::devops_engineer_preset(),
            Self::ui_designer_preset(),
            Self::system_architect_preset(),
            
            // Accessibility presets
            Self::accessibility_friendly_preset(),
            Self::cognitive_assistance_preset(),
            Self::minimal_distraction_preset(),
            
            // Context-specific presets
            Self::mobile_optimized_preset(),
            Self::time_constrained_preset(),
            Self::teaching_mode_preset(),
        ]
    }
    
    /// Create preset categories mapping
    fn create_preset_categories(presets: &[ProfilePreset]) -> HashMap<String, Vec<String>> {
        let mut categories = HashMap::new();
        
        for preset in presets {
            for tag in &preset.tags {
                categories.entry(tag.clone()).or_insert_with(Vec::new).push(preset.id.clone());
            }
        }
        
        categories
    }
    
    // Beginner presets
    fn beginner_guided_preset() -> ProfilePreset {
        ProfilePreset {
            id: "beginner_guided".to_string(),
            name: "Beginner - Guided".to_string(),
            description: "Perfect for newcomers who need step-by-step guidance and explanation".to_string(),
            target_persona: "New users learning development concepts".to_string(),
            profile: CognitivePreferenceProfile {
                interaction_mode: InteractionMode::Collaborative,
                detail_level: DetailLevel::Comprehensive,
                emotional_sensitivity: EmotionalSensitivity::High,
                autonomy_level: AutonomyLevel::Manual,
                communication_style: CommunicationStyle::Casual,
                cognitive_load_settings: CognitiveLoadSettings {
                    max_items_per_chunk: 3,
                    pacing_preference: PacingPreference::Slow,
                    progressive_disclosure: true,
                },
            },
            tags: vec![
                "beginner".to_string(),
                "guided".to_string(),
                "learning".to_string(),
                "collaborative".to_string(),
            ],
            popularity_score: 0.85,
        }
    }
    
    fn beginner_safe_preset() -> ProfilePreset {
        ProfilePreset {
            id: "beginner_safe".to_string(),
            name: "Beginner - Safe Mode".to_string(),
            description: "Extra safety measures and confirmations for new users".to_string(),
            target_persona: "Cautious beginners who want maximum safety".to_string(),
            profile: CognitivePreferenceProfile {
                interaction_mode: InteractionMode::Collaborative,
                detail_level: DetailLevel::Detailed,
                emotional_sensitivity: EmotionalSensitivity::High,
                autonomy_level: AutonomyLevel::Manual,
                communication_style: CommunicationStyle::Formal,
                cognitive_load_settings: CognitiveLoadSettings {
                    max_items_per_chunk: 2,
                    pacing_preference: PacingPreference::Slow,
                    progressive_disclosure: true,
                },
            },
            tags: vec![
                "beginner".to_string(),
                "safe".to_string(),
                "cautious".to_string(),
                "manual".to_string(),
            ],
            popularity_score: 0.72,
        }
    }
    
    fn beginner_learning_preset() -> ProfilePreset {
        ProfilePreset {
            id: "beginner_learning".to_string(),
            name: "Beginner - Learning Focus".to_string(),
            description: "Optimized for educational interactions and skill building".to_string(),
            target_persona: "Students and learners developing new skills".to_string(),
            profile: CognitivePreferenceProfile {
                interaction_mode: InteractionMode::Exploratory,
                detail_level: DetailLevel::Comprehensive,
                emotional_sensitivity: EmotionalSensitivity::Medium,
                autonomy_level: AutonomyLevel::ConfirmFirst,
                communication_style: CommunicationStyle::Adaptive,
                cognitive_load_settings: CognitiveLoadSettings {
                    max_items_per_chunk: 4,
                    pacing_preference: PacingPreference::Medium,
                    progressive_disclosure: true,
                },
            },
            tags: vec![
                "beginner".to_string(),
                "learning".to_string(),
                "educational".to_string(),
                "exploratory".to_string(),
            ],
            popularity_score: 0.78,
        }
    }
    
    // Developer presets
    fn developer_focused_preset() -> ProfilePreset {
        ProfilePreset {
            id: "developer_focused".to_string(),
            name: "Developer - Focused".to_string(),
            description: "Optimized for deep focus work with minimal interruptions".to_string(),
            target_persona: "Experienced developers working on complex projects".to_string(),
            profile: CognitivePreferenceProfile {
                interaction_mode: InteractionMode::Focused,
                detail_level: DetailLevel::Detailed,
                emotional_sensitivity: EmotionalSensitivity::Low,
                autonomy_level: AutonomyLevel::SemiAuto,
                communication_style: CommunicationStyle::Technical,
                cognitive_load_settings: CognitiveLoadSettings {
                    max_items_per_chunk: 7,
                    pacing_preference: PacingPreference::Fast,
                    progressive_disclosure: false,
                },
            },
            tags: vec![
                "developer".to_string(),
                "focused".to_string(),
                "technical".to_string(),
                "experienced".to_string(),
            ],
            popularity_score: 0.88,
        }
    }
    
    fn developer_collaborative_preset() -> ProfilePreset {
        ProfilePreset {
            id: "developer_collaborative".to_string(),
            name: "Developer - Collaborative".to_string(),
            description: "Balanced approach for team development and pair programming".to_string(),
            target_persona: "Developers working in collaborative environments".to_string(),
            profile: CognitivePreferenceProfile {
                interaction_mode: InteractionMode::Collaborative,
                detail_level: DetailLevel::Standard,
                emotional_sensitivity: EmotionalSensitivity::Medium,
                autonomy_level: AutonomyLevel::ConfirmFirst,
                communication_style: CommunicationStyle::Technical,
                cognitive_load_settings: CognitiveLoadSettings {
                    max_items_per_chunk: 5,
                    pacing_preference: PacingPreference::Medium,
                    progressive_disclosure: true,
                },
            },
            tags: vec![
                "developer".to_string(),
                "collaborative".to_string(),
                "team".to_string(),
                "balanced".to_string(),
            ],
            popularity_score: 0.82,
        }
    }
    
    fn developer_rapid_preset() -> ProfilePreset {
        ProfilePreset {
            id: "developer_rapid".to_string(),
            name: "Developer - Rapid Development".to_string(),
            description: "High-speed development with quick iterations and feedback".to_string(),
            target_persona: "Developers working on prototypes and rapid iterations".to_string(),
            profile: CognitivePreferenceProfile {
                interaction_mode: InteractionMode::Autonomous,
                detail_level: DetailLevel::Minimal,
                emotional_sensitivity: EmotionalSensitivity::Low,
                autonomy_level: AutonomyLevel::FullAuto,
                communication_style: CommunicationStyle::Technical,
                cognitive_load_settings: CognitiveLoadSettings {
                    max_items_per_chunk: 10,
                    pacing_preference: PacingPreference::Fast,
                    progressive_disclosure: false,
                },
            },
            tags: vec![
                "developer".to_string(),
                "rapid".to_string(),
                "autonomous".to_string(),
                "prototype".to_string(),
            ],
            popularity_score: 0.75,
        }
    }
    
    // Power user presets
    fn power_user_autonomous_preset() -> ProfilePreset {
        ProfilePreset {
            id: "power_user_autonomous".to_string(),
            name: "Power User - Autonomous".to_string(),
            description: "Maximum autonomy for expert users who trust the system".to_string(),
            target_persona: "Expert users with deep system knowledge".to_string(),
            profile: CognitivePreferenceProfile {
                interaction_mode: InteractionMode::Autonomous,
                detail_level: DetailLevel::Standard,
                emotional_sensitivity: EmotionalSensitivity::Low,
                autonomy_level: AutonomyLevel::FullAuto,
                communication_style: CommunicationStyle::Formal,
                cognitive_load_settings: CognitiveLoadSettings {
                    max_items_per_chunk: 15,
                    pacing_preference: PacingPreference::Fast,
                    progressive_disclosure: false,
                },
            },
            tags: vec![
                "power_user".to_string(),
                "autonomous".to_string(),
                "expert".to_string(),
                "efficient".to_string(),
            ],
            popularity_score: 0.70,
        }
    }
    
    fn power_user_efficient_preset() -> ProfilePreset {
        ProfilePreset {
            id: "power_user_efficient".to_string(),
            name: "Power User - Maximum Efficiency".to_string(),
            description: "Streamlined for maximum productivity and minimal overhead".to_string(),
            target_persona: "Productivity-focused power users".to_string(),
            profile: CognitivePreferenceProfile {
                interaction_mode: InteractionMode::Focused,
                detail_level: DetailLevel::Minimal,
                emotional_sensitivity: EmotionalSensitivity::Low,
                autonomy_level: AutonomyLevel::FullAuto,
                communication_style: CommunicationStyle::Formal,
                cognitive_load_settings: CognitiveLoadSettings {
                    max_items_per_chunk: 20,
                    pacing_preference: PacingPreference::Fast,
                    progressive_disclosure: false,
                },
            },
            tags: vec![
                "power_user".to_string(),
                "efficient".to_string(),
                "productivity".to_string(),
                "minimal".to_string(),
            ],
            popularity_score: 0.68,
        }
    }
    
    fn power_user_expert_preset() -> ProfilePreset {
        ProfilePreset {
            id: "power_user_expert".to_string(),
            name: "Power User - Expert Mode".to_string(),
            description: "Full technical detail for system experts and administrators".to_string(),
            target_persona: "System administrators and technical experts".to_string(),
            profile: CognitivePreferenceProfile {
                interaction_mode: InteractionMode::Autonomous,
                detail_level: DetailLevel::Comprehensive,
                emotional_sensitivity: EmotionalSensitivity::Low,
                autonomy_level: AutonomyLevel::SemiAuto,
                communication_style: CommunicationStyle::Technical,
                cognitive_load_settings: CognitiveLoadSettings {
                    max_items_per_chunk: 12,
                    pacing_preference: PacingPreference::Fast,
                    progressive_disclosure: true,
                },
            },
            tags: vec![
                "power_user".to_string(),
                "expert".to_string(),
                "technical".to_string(),
                "comprehensive".to_string(),
            ],
            popularity_score: 0.73,
        }
    }
    
    // Explorer presets
    fn explorer_curious_preset() -> ProfilePreset {
        ProfilePreset {
            id: "explorer_curious".to_string(),
            name: "Explorer - Curious Learner".to_string(),
            description: "Perfect for users who love to discover and experiment".to_string(),
            target_persona: "Curious users exploring system capabilities".to_string(),
            profile: CognitivePreferenceProfile {
                interaction_mode: InteractionMode::Exploratory,
                detail_level: DetailLevel::Detailed,
                emotional_sensitivity: EmotionalSensitivity::Medium,
                autonomy_level: AutonomyLevel::ConfirmFirst,
                communication_style: CommunicationStyle::Adaptive,
                cognitive_load_settings: CognitiveLoadSettings {
                    max_items_per_chunk: 6,
                    pacing_preference: PacingPreference::Medium,
                    progressive_disclosure: true,
                },
            },
            tags: vec![
                "explorer".to_string(),
                "curious".to_string(),
                "discovery".to_string(),
                "adaptive".to_string(),
            ],
            popularity_score: 0.80,
        }
    }
    
    fn explorer_experimental_preset() -> ProfilePreset {
        ProfilePreset {
            id: "explorer_experimental".to_string(),
            name: "Explorer - Experimental".to_string(),
            description: "For users who want to try new features and push boundaries".to_string(),
            target_persona: "Adventurous users who like trying new things".to_string(),
            profile: CognitivePreferenceProfile {
                interaction_mode: InteractionMode::Exploratory,
                detail_level: DetailLevel::Comprehensive,
                emotional_sensitivity: EmotionalSensitivity::Low,
                autonomy_level: AutonomyLevel::SemiAuto,
                communication_style: CommunicationStyle::Casual,
                cognitive_load_settings: CognitiveLoadSettings {
                    max_items_per_chunk: 8,
                    pacing_preference: PacingPreference::Medium,
                    progressive_disclosure: true,
                },
            },
            tags: vec![
                "explorer".to_string(),
                "experimental".to_string(),
                "adventurous".to_string(),
                "boundaries".to_string(),
            ],
            popularity_score: 0.65,
        }
    }
    
    fn explorer_discovery_preset() -> ProfilePreset {
        ProfilePreset {
            id: "explorer_discovery".to_string(),
            name: "Explorer - Discovery Mode".to_string(),
            description: "Guided exploration with helpful hints and suggestions".to_string(),
            target_persona: "Users who want structured exploration".to_string(),
            profile: CognitivePreferenceProfile {
                interaction_mode: InteractionMode::Exploratory,
                detail_level: DetailLevel::Standard,
                emotional_sensitivity: EmotionalSensitivity::Medium,
                autonomy_level: AutonomyLevel::ConfirmFirst,
                communication_style: CommunicationStyle::Casual,
                cognitive_load_settings: CognitiveLoadSettings {
                    max_items_per_chunk: 5,
                    pacing_preference: PacingPreference::Medium,
                    progressive_disclosure: true,
                },
            },
            tags: vec![
                "explorer".to_string(),
                "discovery".to_string(),
                "guided".to_string(),
                "structured".to_string(),
            ],
            popularity_score: 0.77,
        }
    }
    
    // Specialized presets
    fn security_focused_preset() -> ProfilePreset {
        ProfilePreset {
            id: "security_focused".to_string(),
            name: "Security Professional".to_string(),
            description: "Optimized for security work with emphasis on caution and verification".to_string(),
            target_persona: "Security professionals and auditors".to_string(),
            profile: CognitivePreferenceProfile {
                interaction_mode: InteractionMode::Focused,
                detail_level: DetailLevel::Comprehensive,
                emotional_sensitivity: EmotionalSensitivity::Low,
                autonomy_level: AutonomyLevel::Manual,
                communication_style: CommunicationStyle::Technical,
                cognitive_load_settings: CognitiveLoadSettings {
                    max_items_per_chunk: 4,
                    pacing_preference: PacingPreference::Slow,
                    progressive_disclosure: true,
                },
            },
            tags: vec![
                "security".to_string(),
                "professional".to_string(),
                "verification".to_string(),
                "cautious".to_string(),
            ],
            popularity_score: 0.74,
        }
    }
    
    fn data_analyst_preset() -> ProfilePreset {
        ProfilePreset {
            id: "data_analyst".to_string(),
            name: "Data Analyst".to_string(),
            description: "Configured for data analysis with detailed insights and visualizations".to_string(),
            target_persona: "Data analysts and researchers".to_string(),
            profile: CognitivePreferenceProfile {
                interaction_mode: InteractionMode::Collaborative,
                detail_level: DetailLevel::Comprehensive,
                emotional_sensitivity: EmotionalSensitivity::Medium,
                autonomy_level: AutonomyLevel::ConfirmFirst,
                communication_style: CommunicationStyle::Technical,
                cognitive_load_settings: CognitiveLoadSettings {
                    max_items_per_chunk: 8,
                    pacing_preference: PacingPreference::Medium,
                    progressive_disclosure: true,
                },
            },
            tags: vec![
                "data_analyst".to_string(),
                "analysis".to_string(),
                "insights".to_string(),
                "research".to_string(),
            ],
            popularity_score: 0.71,
        }
    }
    
    fn devops_engineer_preset() -> ProfilePreset {
        ProfilePreset {
            id: "devops_engineer".to_string(),
            name: "DevOps Engineer".to_string(),
            description: "Balanced automation with necessary confirmations for infrastructure work".to_string(),
            target_persona: "DevOps engineers and system operators".to_string(),
            profile: CognitivePreferenceProfile {
                interaction_mode: InteractionMode::Focused,
                detail_level: DetailLevel::Detailed,
                emotional_sensitivity: EmotionalSensitivity::Low,
                autonomy_level: AutonomyLevel::SemiAuto,
                communication_style: CommunicationStyle::Technical,
                cognitive_load_settings: CognitiveLoadSettings {
                    max_items_per_chunk: 6,
                    pacing_preference: PacingPreference::Fast,
                    progressive_disclosure: false,
                },
            },
            tags: vec![
                "devops".to_string(),
                "engineer".to_string(),
                "infrastructure".to_string(),
                "automation".to_string(),
            ],
            popularity_score: 0.76,
        }
    }
    
    fn ui_designer_preset() -> ProfilePreset {
        ProfilePreset {
            id: "ui_designer".to_string(),
            name: "UI/UX Designer".to_string(),
            description: "Visual-focused with emphasis on user experience and design patterns".to_string(),
            target_persona: "UI/UX designers and creative professionals".to_string(),
            profile: CognitivePreferenceProfile {
                interaction_mode: InteractionMode::Collaborative,
                detail_level: DetailLevel::Standard,
                emotional_sensitivity: EmotionalSensitivity::High,
                autonomy_level: AutonomyLevel::ConfirmFirst,
                communication_style: CommunicationStyle::Casual,
                cognitive_load_settings: CognitiveLoadSettings {
                    max_items_per_chunk: 5,
                    pacing_preference: PacingPreference::Medium,
                    progressive_disclosure: true,
                },
            },
            tags: vec![
                "designer".to_string(),
                "ui_ux".to_string(),
                "visual".to_string(),
                "creative".to_string(),
            ],
            popularity_score: 0.69,
        }
    }
    
    fn system_architect_preset() -> ProfilePreset {
        ProfilePreset {
            id: "system_architect".to_string(),
            name: "System Architect".to_string(),
            description: "High-level view with comprehensive technical details for architecture decisions".to_string(),
            target_persona: "System architects and technical leads".to_string(),
            profile: CognitivePreferenceProfile {
                interaction_mode: InteractionMode::Focused,
                detail_level: DetailLevel::Comprehensive,
                emotional_sensitivity: EmotionalSensitivity::Low,
                autonomy_level: AutonomyLevel::ConfirmFirst,
                communication_style: CommunicationStyle::Technical,
                cognitive_load_settings: CognitiveLoadSettings {
                    max_items_per_chunk: 10,
                    pacing_preference: PacingPreference::Medium,
                    progressive_disclosure: true,
                },
            },
            tags: vec![
                "architect".to_string(),
                "system".to_string(),
                "technical_lead".to_string(),
                "architecture".to_string(),
            ],
            popularity_score: 0.72,
        }
    }
    
    // Accessibility presets
    fn accessibility_friendly_preset() -> ProfilePreset {
        ProfilePreset {
            id: "accessibility_friendly".to_string(),
            name: "Accessibility Friendly".to_string(),
            description: "Optimized for users with accessibility needs and assistive technologies".to_string(),
            target_persona: "Users requiring accessibility accommodations".to_string(),
            profile: CognitivePreferenceProfile {
                interaction_mode: InteractionMode::Collaborative,
                detail_level: DetailLevel::Standard,
                emotional_sensitivity: EmotionalSensitivity::High,
                autonomy_level: AutonomyLevel::Manual,
                communication_style: CommunicationStyle::Formal,
                cognitive_load_settings: CognitiveLoadSettings {
                    max_items_per_chunk: 3,
                    pacing_preference: PacingPreference::Slow,
                    progressive_disclosure: true,
                },
            },
            tags: vec![
                "accessibility".to_string(),
                "assistive".to_string(),
                "inclusive".to_string(),
                "accommodations".to_string(),
            ],
            popularity_score: 0.67,
        }
    }
    
    fn cognitive_assistance_preset() -> ProfilePreset {
        ProfilePreset {
            id: "cognitive_assistance".to_string(),
            name: "Cognitive Assistance".to_string(),
            description: "Enhanced support for users who benefit from cognitive assistance".to_string(),
            target_persona: "Users needing additional cognitive support".to_string(),
            profile: CognitivePreferenceProfile {
                interaction_mode: InteractionMode::Collaborative,
                detail_level: DetailLevel::Detailed,
                emotional_sensitivity: EmotionalSensitivity::High,
                autonomy_level: AutonomyLevel::Manual,
                communication_style: CommunicationStyle::Casual,
                cognitive_load_settings: CognitiveLoadSettings {
                    max_items_per_chunk: 2,
                    pacing_preference: PacingPreference::Slow,
                    progressive_disclosure: true,
                },
            },
            tags: vec![
                "cognitive".to_string(),
                "assistance".to_string(),
                "support".to_string(),
                "gentle".to_string(),
            ],
            popularity_score: 0.64,
        }
    }
    
    fn minimal_distraction_preset() -> ProfilePreset {
        ProfilePreset {
            id: "minimal_distraction".to_string(),
            name: "Minimal Distraction".to_string(),
            description: "Reduced cognitive load for users who are easily overwhelmed".to_string(),
            target_persona: "Users sensitive to information overload".to_string(),
            profile: CognitivePreferenceProfile {
                interaction_mode: InteractionMode::Focused,
                detail_level: DetailLevel::Minimal,
                emotional_sensitivity: EmotionalSensitivity::High,
                autonomy_level: AutonomyLevel::Manual,
                communication_style: CommunicationStyle::Formal,
                cognitive_load_settings: CognitiveLoadSettings {
                    max_items_per_chunk: 1,
                    pacing_preference: PacingPreference::Slow,
                    progressive_disclosure: true,
                },
            },
            tags: vec![
                "minimal".to_string(),
                "distraction".to_string(),
                "overload".to_string(),
                "sensitive".to_string(),
            ],
            popularity_score: 0.61,
        }
    }
    
    // Context-specific presets
    fn mobile_optimized_preset() -> ProfilePreset {
        ProfilePreset {
            id: "mobile_optimized".to_string(),
            name: "Mobile Optimized".to_string(),
            description: "Configured for mobile devices with touch interfaces and smaller screens".to_string(),
            target_persona: "Users primarily working on mobile devices".to_string(),
            profile: CognitivePreferenceProfile {
                interaction_mode: InteractionMode::Collaborative,
                detail_level: DetailLevel::Standard,
                emotional_sensitivity: EmotionalSensitivity::Medium,
                autonomy_level: AutonomyLevel::ConfirmFirst,
                communication_style: CommunicationStyle::Casual,
                cognitive_load_settings: CognitiveLoadSettings {
                    max_items_per_chunk: 3,
                    pacing_preference: PacingPreference::Medium,
                    progressive_disclosure: true,
                },
            },
            tags: vec![
                "mobile".to_string(),
                "touch".to_string(),
                "optimized".to_string(),
                "responsive".to_string(),
            ],
            popularity_score: 0.79,
        }
    }
    
    fn time_constrained_preset() -> ProfilePreset {
        ProfilePreset {
            id: "time_constrained".to_string(),
            name: "Time Constrained".to_string(),
            description: "Quick interactions for users with limited time".to_string(),
            target_persona: "Busy users needing quick results".to_string(),
            profile: CognitivePreferenceProfile {
                interaction_mode: InteractionMode::Focused,
                detail_level: DetailLevel::Minimal,
                emotional_sensitivity: EmotionalSensitivity::Low,
                autonomy_level: AutonomyLevel::SemiAuto,
                communication_style: CommunicationStyle::Formal,
                cognitive_load_settings: CognitiveLoadSettings {
                    max_items_per_chunk: 8,
                    pacing_preference: PacingPreference::Fast,
                    progressive_disclosure: false,
                },
            },
            tags: vec![
                "time_constrained".to_string(),
                "quick".to_string(),
                "busy".to_string(),
                "efficient".to_string(),
            ],
            popularity_score: 0.74,
        }
    }
    
    fn teaching_mode_preset() -> ProfilePreset {
        ProfilePreset {
            id: "teaching_mode".to_string(),
            name: "Teaching Mode".to_string(),
            description: "Structured for educational contexts with clear explanations and examples".to_string(),
            target_persona: "Educators and students in learning environments".to_string(),
            profile: CognitivePreferenceProfile {
                interaction_mode: InteractionMode::Collaborative,
                detail_level: DetailLevel::Comprehensive,
                emotional_sensitivity: EmotionalSensitivity::Medium,
                autonomy_level: AutonomyLevel::Manual,
                communication_style: CommunicationStyle::Adaptive,
                cognitive_load_settings: CognitiveLoadSettings {
                    max_items_per_chunk: 4,
                    pacing_preference: PacingPreference::Slow,
                    progressive_disclosure: true,
                },
            },
            tags: vec![
                "teaching".to_string(),
                "educational".to_string(),
                "learning".to_string(),
                "structured".to_string(),
            ],
            popularity_score: 0.81,
        }
    }
}

impl Default for PresetManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Preset utilities for common operations
pub struct PresetUtils;

impl PresetUtils {
    /// Recommend presets based on user characteristics
    pub fn recommend_presets(
        experience_level: ExperienceLevel,
        work_context: WorkContext,
        preferences: UserPreferences,
    ) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        match (experience_level, work_context) {
            (ExperienceLevel::Beginner, _) => {
                recommendations.extend([
                    "beginner_guided".to_string(),
                    "beginner_safe".to_string(),
                    "beginner_learning".to_string(),
                ]);
            },
            (ExperienceLevel::Intermediate, WorkContext::Development) => {
                recommendations.extend([
                    "developer_collaborative".to_string(),
                    "developer_focused".to_string(),
                ]);
            },
            (ExperienceLevel::Advanced, WorkContext::Development) => {
                recommendations.extend([
                    "developer_focused".to_string(),
                    "developer_rapid".to_string(),
                    "power_user_autonomous".to_string(),
                ]);
            },
            (ExperienceLevel::Expert, _) => {
                recommendations.extend([
                    "power_user_autonomous".to_string(),
                    "power_user_expert".to_string(),
                    "power_user_efficient".to_string(),
                ]);
            },
            (_, WorkContext::Security) => {
                recommendations.push("security_focused".to_string());
            },
            (_, WorkContext::DataAnalysis) => {
                recommendations.push("data_analyst".to_string());
            },
            (_, WorkContext::DevOps) => {
                recommendations.push("devops_engineer".to_string());
            },
            (_, WorkContext::Design) => {
                recommendations.push("ui_designer".to_string());
            },
            (_, WorkContext::Architecture) => {
                recommendations.push("system_architect".to_string());
            },
            (_, WorkContext::Learning) => {
                recommendations.extend([
                    "explorer_curious".to_string(),
                    "teaching_mode".to_string(),
                ]);
            },
            _ => {
                recommendations.push("developer_collaborative".to_string());
            },
        }
        
        // Add context-specific recommendations
        if preferences.accessibility_needs {
            recommendations.push("accessibility_friendly".to_string());
        }
        
        if preferences.mobile_primary {
            recommendations.push("mobile_optimized".to_string());
        }
        
        if preferences.time_constrained {
            recommendations.push("time_constrained".to_string());
        }
        
        recommendations
    }
    
    /// Get preset compatibility score with user profile
    pub fn calculate_compatibility(
        preset: &ProfilePreset,
        user_profile: &CognitivePreferenceProfile,
    ) -> f32 {
        let mut score = 0.0;
        let mut factors = 0;
        
        // Compare interaction modes
        if preset.profile.interaction_mode == user_profile.interaction_mode {
            score += 1.0;
        }
        factors += 1;
        
        // Compare detail levels
        if preset.profile.detail_level == user_profile.detail_level {
            score += 1.0;
        }
        factors += 1;
        
        // Compare autonomy levels
        if preset.profile.autonomy_level == user_profile.autonomy_level {
            score += 1.0;
        }
        factors += 1;
        
        // Compare communication styles
        if preset.profile.communication_style == user_profile.communication_style {
            score += 1.0;
        }
        factors += 1;
        
        // Compare emotional sensitivity
        if preset.profile.emotional_sensitivity == user_profile.emotional_sensitivity {
            score += 1.0;
        }
        factors += 1;
        
        score / factors as f32
    }
}

/// User experience level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExperienceLevel {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}

/// Work context
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkContext {
    Development,
    Security,
    DataAnalysis,
    DevOps,
    Design,
    Architecture,
    Learning,
    General,
}

/// User preferences for preset recommendation
#[derive(Debug, Clone, Default)]
pub struct UserPreferences {
    pub accessibility_needs: bool,
    pub mobile_primary: bool,
    pub time_constrained: bool,
    pub collaborative_focus: bool,
    pub privacy_conscious: bool,
}
