use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use brain_types::error::BrainError;
use crate::agents::traits::{BrainResult, CognitivePreferenceProfile};
use super::{
    CognitiveProfileManager, ProfileUpdates, ProfilePreset, ProfileAnalytics,
    ProfileUsageStats, PreferenceSnapshot, SatisfactionMetrics,
    OptimizationRecommendation, RecommendationType
};

/// In-memory implementation of cognitive profile manager
pub struct InMemoryProfileManager {
    /// User profiles storage
    profiles: Arc<RwLock<HashMap<String, CognitivePreferenceProfile>>>,
    
    /// Profile analytics storage
    analytics: Arc<RwLock<HashMap<String, ProfileAnalytics>>>,
    
    /// Available presets
    presets: Arc<RwLock<Vec<ProfilePreset>>>,
    
    /// Profile usage tracking
    usage_tracking: Arc<RwLock<HashMap<String, Vec<ProfileEvent>>>>,
}

/// Profile event for tracking usage patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileEvent {
    /// Event timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    
    /// Event type
    pub event_type: ProfileEventType,
    
    /// Event data
    pub data: HashMap<String, serde_json::Value>,
    
    /// Session identifier
    pub session_id: Option<String>,
    
    /// Agent involved (if applicable)
    pub agent_id: Option<String>,
}

/// Type of profile event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProfileEventType {
    ProfileCreated,
    ProfileUpdated,
    ProfileAccessed,
    PreferenceChanged,
    InteractionCompleted,
    SatisfactionRecorded,
    PresetApplied,
    OptimizationApplied,
}

impl InMemoryProfileManager {
    /// Create a new in-memory profile manager
    pub fn new() -> Self {
        Self {
            profiles: Arc::new(RwLock::new(HashMap::new())),
            analytics: Arc::new(RwLock::new(HashMap::new())),
            presets: Arc::new(RwLock::new(Self::default_presets())),
            usage_tracking: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Initialize with default presets
    fn default_presets() -> Vec<ProfilePreset> {
        vec![
            ProfilePreset {
                id: "developer_focused".to_string(),
                name: "Developer - Focused".to_string(),
                description: "Optimized for focused development work with minimal distractions".to_string(),
                target_persona: "Experienced developer working on complex projects".to_string(),
                profile: CognitivePreferenceProfile {
                    interaction_mode: crate::agents::traits::InteractionMode::Focused,
                    detail_level: crate::agents::traits::DetailLevel::Detailed,
                    emotional_sensitivity: crate::agents::traits::EmotionalSensitivity::Low,
                    autonomy_level: crate::agents::traits::AutonomyLevel::SemiAuto,
                    communication_style: crate::agents::traits::CommunicationStyle::Technical,
                    cognitive_load_settings: crate::agents::traits::CognitiveLoadSettings {
                        max_items_per_chunk: 7,
                        pacing_preference: crate::agents::traits::PacingPreference::Fast,
                        progressive_disclosure: true,
                    },
                },
                tags: vec!["developer".to_string(), "focused".to_string(), "technical".to_string()],
                popularity_score: 0.85,
            },
            ProfilePreset {
                id: "beginner_collaborative".to_string(),
                name: "Beginner - Collaborative".to_string(),
                description: "Perfect for newcomers who prefer guided, step-by-step interactions".to_string(),
                target_persona: "New developers or users learning the system".to_string(),
                profile: CognitivePreferenceProfile {
                    interaction_mode: crate::agents::traits::InteractionMode::Collaborative,
                    detail_level: crate::agents::traits::DetailLevel::Comprehensive,
                    emotional_sensitivity: crate::agents::traits::EmotionalSensitivity::High,
                    autonomy_level: crate::agents::traits::AutonomyLevel::Manual,
                    communication_style: crate::agents::traits::CommunicationStyle::Casual,
                    cognitive_load_settings: crate::agents::traits::CognitiveLoadSettings {
                        max_items_per_chunk: 3,
                        pacing_preference: crate::agents::traits::PacingPreference::Slow,
                        progressive_disclosure: true,
                    },
                },
                tags: vec!["beginner".to_string(), "collaborative".to_string(), "learning".to_string()],
                popularity_score: 0.78,
            },
            ProfilePreset {
                id: "power_user_autonomous".to_string(),
                name: "Power User - Autonomous".to_string(),
                description: "For experienced users who prefer maximum autonomy and minimal interruptions".to_string(),
                target_persona: "Expert users who trust the system to make decisions".to_string(),
                profile: CognitivePreferenceProfile {
                    interaction_mode: crate::agents::traits::InteractionMode::Autonomous,
                    detail_level: crate::agents::traits::DetailLevel::Standard,
                    emotional_sensitivity: crate::agents::traits::EmotionalSensitivity::Low,
                    autonomy_level: crate::agents::traits::AutonomyLevel::FullAuto,
                    communication_style: crate::agents::traits::CommunicationStyle::Formal,
                    cognitive_load_settings: crate::agents::traits::CognitiveLoadSettings {
                        max_items_per_chunk: 10,
                        pacing_preference: crate::agents::traits::PacingPreference::Fast,
                        progressive_disclosure: false,
                    },
                },
                tags: vec!["expert".to_string(), "autonomous".to_string(), "efficient".to_string()],
                popularity_score: 0.72,
            },
            ProfilePreset {
                id: "explorer_adaptive".to_string(),
                name: "Explorer - Adaptive".to_string(),
                description: "Ideal for users who like to experiment and discover new features".to_string(),
                target_persona: "Curious users exploring system capabilities".to_string(),
                profile: CognitivePreferenceProfile {
                    interaction_mode: crate::agents::traits::InteractionMode::Exploratory,
                    detail_level: crate::agents::traits::DetailLevel::Detailed,
                    emotional_sensitivity: crate::agents::traits::EmotionalSensitivity::Medium,
                    autonomy_level: crate::agents::traits::AutonomyLevel::ConfirmFirst,
                    communication_style: crate::agents::traits::CommunicationStyle::Adaptive,
                    cognitive_load_settings: crate::agents::traits::CognitiveLoadSettings {
                        max_items_per_chunk: 5,
                        pacing_preference: crate::agents::traits::PacingPreference::Medium,
                        progressive_disclosure: true,
                    },
                },
                tags: vec!["exploration".to_string(), "adaptive".to_string(), "discovery".to_string()],
                popularity_score: 0.68,
            },
        ]
    }
    
    /// Record a profile event
    async fn record_event(&self, user_id: &str, event: ProfileEvent) -> BrainResult<()> {
        let mut tracking = self.usage_tracking.write().await;
        let events = tracking.entry(user_id.to_string()).or_insert_with(Vec::new);
        events.push(event);
        
        // Keep only the last 1000 events per user to prevent unbounded growth
        if events.len() > 1000 {
            events.drain(0..events.len() - 1000);
        }
        
        Ok(())
    }
    
    /// Update analytics for a user
    async fn update_analytics(&self, user_id: &str) -> BrainResult<()> {
        let events = {
            let tracking = self.usage_tracking.read().await;
            tracking.get(user_id).cloned().unwrap_or_default()
        };
        
        let profile = {
            let profiles = self.profiles.read().await;
            profiles.get(user_id).cloned()
        };
        
        if let Some(profile) = profile {
            let analytics = self.compute_analytics(user_id, &profile, &events).await?;
            let mut analytics_store = self.analytics.write().await;
            analytics_store.insert(user_id.to_string(), analytics);
        }
        
        Ok(())
    }
    
    /// Compute analytics from events and profile
    async fn compute_analytics(
        &self,
        user_id: &str,
        profile: &CognitivePreferenceProfile,
        events: &[ProfileEvent]
    ) -> BrainResult<ProfileAnalytics> {
        let total_sessions = events.iter()
            .filter(|e| matches!(e.event_type, ProfileEventType::ProfileAccessed))
            .map(|e| e.session_id.as_ref())
            .filter_map(|s| s)
            .collect::<std::collections::HashSet<_>>()
            .len() as u64;
        
        let avg_session_duration = 45.0; // Placeholder calculation
        let most_used_mode = profile.interaction_mode.clone();
        let change_frequency = events.iter()
            .filter(|e| matches!(e.event_type, ProfileEventType::PreferenceChanged))
            .count() as f64 / (total_sessions as f64).max(1.0);
        
        let last_updated = events.iter()
            .filter(|e| matches!(e.event_type, ProfileEventType::ProfileUpdated))
            .map(|e| e.timestamp)
            .max()
            .unwrap_or_else(|| chrono::Utc::now());
        
        let usage_stats = ProfileUsageStats {
            total_sessions,
            avg_session_duration,
            most_used_mode,
            change_frequency,
            last_updated,
        };
        
        let preference_evolution = events.iter()
            .filter(|e| matches!(e.event_type, ProfileEventType::ProfileUpdated))
            .map(|e| PreferenceSnapshot {
                timestamp: e.timestamp,
                profile: profile.clone(),
                change_trigger: "user_update".to_string(),
                satisfaction_score: None,
            })
            .collect();
        
        let agent_interactions = HashMap::new(); // Placeholder
        
        let satisfaction_metrics = SatisfactionMetrics {
            overall_score: 0.8, // Placeholder
            mode_satisfaction: HashMap::new(),
            agent_satisfaction: HashMap::new(),
            cognitive_load_comfort: 0.75,
            stability_score: 0.85,
        };
        
        let optimization_recommendations = self.generate_optimization_recommendations(profile).await?;
        
        Ok(ProfileAnalytics {
            user_id: user_id.to_string(),
            usage_stats,
            preference_evolution,
            agent_interactions,
            satisfaction_metrics,
            optimization_recommendations,
        })
    }
    
    /// Generate optimization recommendations
    async fn generate_optimization_recommendations(
        &self,
        profile: &CognitivePreferenceProfile
    ) -> BrainResult<Vec<OptimizationRecommendation>> {
        let mut recommendations = Vec::new();
        
        // Example recommendation: suggest faster pacing for experienced users
        if matches!(profile.detail_level, crate::agents::traits::DetailLevel::Detailed | crate::agents::traits::DetailLevel::Comprehensive) 
            && matches!(profile.cognitive_load_settings.pacing_preference, crate::agents::traits::PacingPreference::Slow) {
            recommendations.push(OptimizationRecommendation {
                id: "pacing_optimization_1".to_string(),
                recommendation_type: RecommendationType::CognitiveLoadReduction,
                description: "Consider increasing pacing speed based on your detailed preference level".to_string(),
                expected_impact: 0.15,
                confidence: 0.8,
                proposed_changes: ProfileUpdates {
                    cognitive_load_settings: Some(crate::agents::traits::CognitiveLoadSettings {
                        max_items_per_chunk: profile.cognitive_load_settings.max_items_per_chunk,
                        pacing_preference: crate::agents::traits::PacingPreference::Medium,
                        progressive_disclosure: profile.cognitive_load_settings.progressive_disclosure,
                    }),
                    ..Default::default()
                },
            });
        }
        
        // Example recommendation: suggest higher autonomy for frequent users
        if matches!(profile.autonomy_level, crate::agents::traits::AutonomyLevel::Manual) {
            recommendations.push(OptimizationRecommendation {
                id: "autonomy_optimization_1".to_string(),
                recommendation_type: RecommendationType::AutonomyLevelTuning,
                description: "Consider enabling semi-automatic mode to reduce confirmation overhead".to_string(),
                expected_impact: 0.25,
                confidence: 0.7,
                proposed_changes: ProfileUpdates {
                    autonomy_level: Some(crate::agents::traits::AutonomyLevel::ConfirmFirst),
                    ..Default::default()
                },
            });
        }
        
        Ok(recommendations)
    }
}

impl Default for InMemoryProfileManager {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl CognitiveProfileManager for InMemoryProfileManager {
    /// Load a user's cognitive preference profile
    async fn load_profile(&self, user_id: &str) -> BrainResult<CognitivePreferenceProfile> {
        let profiles = self.profiles.read().await;
        let profile = profiles.get(user_id)
            .cloned()
            .unwrap_or_default();
        
        // Record access event
        let event = ProfileEvent {
            timestamp: chrono::Utc::now(),
            event_type: ProfileEventType::ProfileAccessed,
            data: HashMap::new(),
            session_id: None,
            agent_id: None,
        };
        self.record_event(user_id, event).await?;
        
        Ok(profile)
    }
    
    /// Save a user's cognitive preference profile
    async fn save_profile(&self, user_id: &str, profile: &CognitivePreferenceProfile) -> BrainResult<()> {
        let mut profiles = self.profiles.write().await;
        let is_new = !profiles.contains_key(user_id);
        profiles.insert(user_id.to_string(), profile.clone());
        
        // Record event
        let event = ProfileEvent {
            timestamp: chrono::Utc::now(),
            event_type: if is_new { 
                ProfileEventType::ProfileCreated 
            } else { 
                ProfileEventType::ProfileUpdated 
            },
            data: HashMap::new(),
            session_id: None,
            agent_id: None,
        };
        self.record_event(user_id, event).await?;
        
        // Update analytics
        self.update_analytics(user_id).await?;
        
        Ok(())
    }
    
    /// Update specific preferences within a profile
    async fn update_preferences(&self, user_id: &str, updates: ProfileUpdates) -> BrainResult<CognitivePreferenceProfile> {
        let mut profiles = self.profiles.write().await;
        let mut profile = profiles.get(user_id)
            .cloned()
            .unwrap_or_default();
        
        // Serialize updates first before we start moving fields
        let mut event_data = HashMap::new();
        if let Ok(updates_json) = serde_json::to_value(&updates) {
            if let Some(obj) = updates_json.as_object() {
                event_data = obj.iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect();
            }
        }
        
        // Apply updates
        if let Some(interaction_mode) = updates.interaction_mode {
            profile.interaction_mode = interaction_mode;
        }
        if let Some(detail_level) = updates.detail_level {
            profile.detail_level = detail_level;
        }
        if let Some(emotional_sensitivity) = updates.emotional_sensitivity {
            profile.emotional_sensitivity = emotional_sensitivity;
        }
        if let Some(autonomy_level) = updates.autonomy_level {
            profile.autonomy_level = autonomy_level;
        }
        if let Some(communication_style) = updates.communication_style {
            profile.communication_style = communication_style;
        }
        if let Some(cognitive_load_settings) = updates.cognitive_load_settings {
            profile.cognitive_load_settings = cognitive_load_settings;
        }
        
        profiles.insert(user_id.to_string(), profile.clone());
        
        let event = ProfileEvent {
            timestamp: chrono::Utc::now(),
            event_type: ProfileEventType::PreferenceChanged,
            data: event_data,
            session_id: None,
            agent_id: None,
        };
        self.record_event(user_id, event).await?;
        
        // Update analytics
        self.update_analytics(user_id).await?;
        
        Ok(profile)
    }
    
    /// Get available profile presets
    async fn get_presets(&self) -> BrainResult<Vec<ProfilePreset>> {
        let presets = self.presets.read().await;
        Ok(presets.clone())
    }
    
    /// Apply a preset to a user's profile
    async fn apply_preset(&self, user_id: &str, preset_id: &str) -> BrainResult<CognitivePreferenceProfile> {
        let presets = self.presets.read().await;
        let preset = presets.iter()
            .find(|p| p.id == preset_id)
            .ok_or_else(|| BrainError::NotFound(format!("Preset not found: {}", preset_id)))?;
        
        let profile = preset.profile.clone();
        drop(presets);
        
        // Save the profile
        self.save_profile(user_id, &profile).await?;
        
        // Record preset application event
        let event = ProfileEvent {
            timestamp: chrono::Utc::now(),
            event_type: ProfileEventType::PresetApplied,
            data: [("preset_id".to_string(), serde_json::Value::String(preset_id.to_string()))]
                .iter().cloned().collect(),
            session_id: None,
            agent_id: None,
        };
        self.record_event(user_id, event).await?;
        
        Ok(profile)
    }
    
    /// Get profile analytics and usage patterns
    async fn get_profile_analytics(&self, user_id: &str) -> BrainResult<ProfileAnalytics> {
        // Ensure analytics are up to date
        self.update_analytics(user_id).await?;
        
        let analytics = self.analytics.read().await;
        analytics.get(user_id)
            .cloned()
            .ok_or_else(|| BrainError::NotFound(format!("Analytics not found for user: {}", user_id)))
    }
}

/// File-based profile manager for persistence
pub struct FileBasedProfileManager {
    /// Directory for storing profiles
    profile_dir: std::path::PathBuf,
    
    /// In-memory cache
    memory_manager: InMemoryProfileManager,
}

impl FileBasedProfileManager {
    /// Create a new file-based profile manager
    pub fn new(profile_dir: std::path::PathBuf) -> BrainResult<Self> {
        std::fs::create_dir_all(&profile_dir)
            .map_err(|e| BrainError::Io { source: e })?;
        
        Ok(Self {
            profile_dir,
            memory_manager: InMemoryProfileManager::new(),
        })
    }
    
    /// Get profile file path
    fn profile_path(&self, user_id: &str) -> std::path::PathBuf {
        self.profile_dir.join(format!("{}.profile.json", user_id))
    }
    
    /// Load profile from file
    async fn load_profile_from_file(&self, user_id: &str) -> BrainResult<Option<CognitivePreferenceProfile>> {
        let path = self.profile_path(user_id);
        if !path.exists() {
            return Ok(None);
        }
        
        let content = tokio::fs::read_to_string(&path).await
            .map_err(|e| BrainError::Io { source: e })?;
        
        let profile: CognitivePreferenceProfile = serde_json::from_str(&content)
            .map_err(|e| BrainError::Serialization { source: Box::new(e) })?;
        
        Ok(Some(profile))
    }
    
    /// Save profile to file
    async fn save_profile_to_file(&self, user_id: &str, profile: &CognitivePreferenceProfile) -> BrainResult<()> {
        let path = self.profile_path(user_id);
        let content = serde_json::to_string_pretty(profile)
            .map_err(|e| BrainError::Serialization { source: Box::new(e) })?;
        
        tokio::fs::write(&path, content).await
            .map_err(|e| BrainError::Io { source: e })?;
        
        Ok(())
    }
}

#[async_trait]
impl CognitiveProfileManager for FileBasedProfileManager {
    async fn load_profile(&self, user_id: &str) -> BrainResult<CognitivePreferenceProfile> {
        // Try to load from file first
        if let Some(profile) = self.load_profile_from_file(user_id).await? {
            // Cache in memory
            self.memory_manager.save_profile(user_id, &profile).await?;
            Ok(profile)
        } else {
            // Return default profile
            let profile = CognitivePreferenceProfile::default();
            self.save_profile(user_id, &profile).await?;
            Ok(profile)
        }
    }
    
    async fn save_profile(&self, user_id: &str, profile: &CognitivePreferenceProfile) -> BrainResult<()> {
        // Save to both file and memory
        self.save_profile_to_file(user_id, profile).await?;
        self.memory_manager.save_profile(user_id, profile).await?;
        Ok(())
    }
    
    async fn update_preferences(&self, user_id: &str, updates: ProfileUpdates) -> BrainResult<CognitivePreferenceProfile> {
        let profile = self.memory_manager.update_preferences(user_id, updates).await?;
        self.save_profile_to_file(user_id, &profile).await?;
        Ok(profile)
    }
    
    async fn get_presets(&self) -> BrainResult<Vec<ProfilePreset>> {
        self.memory_manager.get_presets().await
    }
    
    async fn apply_preset(&self, user_id: &str, preset_id: &str) -> BrainResult<CognitivePreferenceProfile> {
        let profile = self.memory_manager.apply_preset(user_id, preset_id).await?;
        self.save_profile_to_file(user_id, &profile).await?;
        Ok(profile)
    }
    
    async fn get_profile_analytics(&self, user_id: &str) -> BrainResult<ProfileAnalytics> {
        self.memory_manager.get_profile_analytics(user_id).await
    }
} 