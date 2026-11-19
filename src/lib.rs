// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-skills: Universal Agent Skill Registry and Execution Engine.
//!
//! Provides deterministic cataloging, capability matching, trigger resolution,
//! and event lifecycle tracking for AI and developer agent skills.
#![forbid(unsafe_code)]
#![warn(missing_docs)]

use std::collections::BTreeMap;
use std::fmt;

/// Categories of agent skills.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SkillCategory {
    /// Code review and static analysis.
    CodeReview,
    /// Data science and machine learning.
    DataAnalysis,
    /// Software architecture and design.
    Architecture,
    /// Low-level systems and compiler engineering.
    LowLevelEngineering,
    /// Web development and UI/UX design.
    WebAndDesign,
    /// DevOps, automation, and workflow orchestration.
    DevOpsAndWorkflows,
    /// General developer productivity and documentation.
    Productivity,
}

impl SkillCategory {
    /// Return category name as static string.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::CodeReview => "Code Review",
            Self::DataAnalysis => "Data Analysis",
            Self::Architecture => "Architecture",
            Self::LowLevelEngineering => "Low-Level Engineering",
            Self::WebAndDesign => "Web & Design",
            Self::DevOpsAndWorkflows => "DevOps & Workflows",
            Self::Productivity => "Productivity",
        }
    }
}

/// A registered agent skill declaration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Skill {
    /// Qualified identifier (e.g. "anthropics/skills").
    pub id: String,
    /// Skill author or organization.
    pub author: String,
    /// Human-readable skill name.
    pub name: String,
    /// Skill functional category.
    pub category: SkillCategory,
    /// Detailed summary of skill capability.
    pub description: String,
    /// Capabilities or tools provided by this skill.
    pub capabilities: Vec<String>,
    /// Triggers or slash commands that activate this skill.
    pub triggers: Vec<String>,
}

impl Skill {
    /// Construct a new Skill entry with validation.
    pub fn new(
        id: impl Into<String>,
        author: impl Into<String>,
        name: impl Into<String>,
        category: SkillCategory,
        description: impl Into<String>,
    ) -> Result<Self, SkillsError> {
        let id = id.into();
        let name = name.into();
        if id.trim().is_empty() {
            return Err(SkillsError::EmptySkillId);
        }
        if name.trim().is_empty() {
            return Err(SkillsError::EmptySkillName);
        }
        Ok(Self {
            id,
            author: author.into(),
            name,
            category,
            description: description.into(),
            capabilities: Vec::new(),
            triggers: Vec::new(),
        })
    }

    /// Add a capability tag to the skill.
    pub fn with_capability(mut self, cap: impl Into<String>) -> Self {
        self.capabilities.push(cap.into());
        self
    }

    /// Add a trigger phrase to the skill.
    pub fn with_trigger(mut self, trig: impl Into<String>) -> Self {
        self.triggers.push(trig.into());
        self
    }
}

/// Recorded skill invocation or lifecycle event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    /// Unique event identifier.
    pub id: String,
    /// Subject or skill ID being invoked.
    pub subject: String,
    /// Contextual event detail or outcome.
    pub detail: String,
    /// Milliseconds timestamp.
    pub timestamp: u64,
}

/// Error types for skills registry operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SkillsError {
    /// Skill ID must not be empty.
    EmptySkillId,
    /// Skill name must not be empty.
    EmptySkillName,
    /// Skill already registered in catalog.
    DuplicateSkill(String),
    /// Event subject must not be empty.
    EmptyEventSubject,
    /// Skill not found in registry.
    SkillNotFound(String),
}

impl fmt::Display for SkillsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptySkillId => write!(f, "skill id must not be empty"),
            Self::EmptySkillName => write!(f, "skill name must not be empty"),
            Self::DuplicateSkill(id) => write!(f, "duplicate skill '{id}'"),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
            Self::SkillNotFound(id) => write!(f, "skill '{id}' not found"),
        }
    }
}

impl std::error::Error for SkillsError {}

/// Universal Skill Registry managing cataloged agent skills and execution events.
pub struct Skills {
    /// Name of this skill registry instance.
    pub name: String,
    skills: BTreeMap<String, Skill>,
    events: BTreeMap<String, Event>,
}

impl Skills {
    /// Create an empty skill registry instance.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            skills: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    /// Create and initialize registry populated with standard SigmaOS flagship skills.
    pub fn with_defaults(name: impl Into<String>) -> Self {
        let mut reg = Self::new(name);
        reg.populate_standard_skills();
        reg
    }

    fn populate_standard_skills(&mut self) {
        let defaults = vec![
            Skill::new("anthropics/skills", "anthropics", "Anthropic Agent Skills", SkillCategory::Productivity, "Core agent skill primitives and tools")
                .unwrap().with_capability("prompt-eval").with_trigger("/anthropic"),
            Skill::new("addyosmani/agent-skills", "addyosmani", "Web Performance & Code Review", SkillCategory::WebAndDesign, "Modern web architecture and performance analysis")
                .unwrap().with_capability("web-perf").with_trigger("/web-perf"),
            Skill::new("alibaba/open-code-review", "alibaba", "Open Code Review Assistant", SkillCategory::CodeReview, "Automated code inspection and vulnerability detection")
                .unwrap().with_capability("code-review").with_trigger("/review"),
            Skill::new("akkie76/code-review-skills", "akkie76", "Code Review Essentials", SkillCategory::CodeReview, "Systematic PR hygiene and review guidelines")
                .unwrap().with_capability("code-review").with_trigger("/audit"),
            Skill::new("aiopshwang/data-analysis-ml-agent-skills", "aiopshwang", "ML & Data Analysis Suite", SkillCategory::DataAnalysis, "Data science pipelines, model evaluation, and EDA")
                .unwrap().with_capability("data-analysis").with_trigger("/data-analysis"),
            Skill::new("mohitmishra786/low-level-dev-skills", "mohitmishra786", "Low-Level Systems Engineering", SkillCategory::LowLevelEngineering, "Kernel internals, memory safety, and assembly debugging")
                .unwrap().with_capability("low-level-dev").with_trigger("/systems"),
            Skill::new("cathrynlavery/diagram-design", "cathrynlavery", "Diagram & Architecture Visualizer", SkillCategory::Architecture, "Mermaid flowcharts, system diagrams, and UI wireframes")
                .unwrap().with_capability("diagram-design").with_trigger("/diagram"),
            Skill::new("EtienneLescot/n8n-as-code", "EtienneLescot", "Workflow & Automation Orchestrator", SkillCategory::DevOpsAndWorkflows, "Workflow pipelines, webhook automation, and integration DAGs")
                .unwrap().with_capability("workflows").with_trigger("/workflow"),
            Skill::new("docmd-io/docmd", "docmd-io", "Technical Documentation Engine", SkillCategory::Productivity, "Markdown documentation generator and spec validator")
                .unwrap().with_capability("documentation").with_trigger("/doc"),
        ];

        for s in defaults {
            let _ = self.add_skill(s);
        }
    }

    /// Add a skill to the registry.
    pub fn add_skill(&mut self, item: Skill) -> Result<(), SkillsError> {
        let id = item.id.clone();
        if self.skills.contains_key(&id) {
            return Err(SkillsError::DuplicateSkill(id));
        }
        self.skills.insert(id, item);
        Ok(())
    }

    /// Total count of registered skills.
    pub fn skill_count(&self) -> usize {
        self.skills.len()
    }

    /// Retrieve skill reference by ID.
    pub fn get_skill(&self, id: &str) -> Option<&Skill> {
        self.skills.get(id)
    }

    /// Search skills by query matching ID, name, or description.
    pub fn search(&self, query: &str) -> Vec<&Skill> {
        let q = query.to_lowercase();
        self.skills
            .values()
            .filter(|s| {
                s.id.to_lowercase().contains(&q)
                    || s.name.to_lowercase().contains(&q)
                    || s.description.to_lowercase().contains(&q)
                    || s.capabilities.iter().any(|c| c.to_lowercase().contains(&q))
            })
            .collect()
    }

    /// Find all skills belonging to a specific category.
    pub fn get_by_category(&self, category: SkillCategory) -> Vec<&Skill> {
        self.skills.values().filter(|s| s.category == category).collect()
    }

    /// Find skills that provide a specific capability tag.
    pub fn find_by_capability(&self, capability: &str) -> Vec<&Skill> {
        let cap = capability.to_lowercase();
        self.skills
            .values()
            .filter(|s| s.capabilities.iter().any(|c| c.to_lowercase() == cap))
            .collect()
    }

    /// Record an execution or lifecycle event.
    pub fn add_event(&mut self, event: Event) -> Result<(), SkillsError> {
        if event.subject.trim().is_empty() {
            return Err(SkillsError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    /// Total count of recorded events.
    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    /// Retrieve event reference by ID.
    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_registry_aaa() {
        // Arrange & Act
        let m = Skills::new("TestRegistry");

        // Assert
        assert_eq!(m.name, "TestRegistry");
        assert_eq!(m.skill_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_skill_and_retrieve_aaa() {
        // Arrange
        let mut m = Skills::new("TestRegistry");
        let skill = Skill::new(
            "anthropics/skills",
            "anthropics",
            "Anthropic Skills",
            SkillCategory::Productivity,
            "Agent tools",
        )
        .unwrap()
        .with_capability("prompting");

        // Act
        m.add_skill(skill).unwrap();

        // Assert
        assert_eq!(m.skill_count(), 1);
        let retrieved = m.get_skill("anthropics/skills").unwrap();
        assert_eq!(retrieved.name, "Anthropic Skills");
        assert_eq!(retrieved.capabilities, vec!["prompting"]);
    }

    #[test]
    fn test_duplicate_skill_rejected_aaa() {
        // Arrange
        let mut m = Skills::new("TestRegistry");
        let s1 = Skill::new("test/dup", "author", "Name", SkillCategory::CodeReview, "desc").unwrap();
        let s2 = Skill::new("test/dup", "author", "Name 2", SkillCategory::CodeReview, "desc").unwrap();
        m.add_skill(s1).unwrap();

        // Act
        let res = m.add_skill(s2);

        // Assert
        assert_eq!(res, Err(SkillsError::DuplicateSkill("test/dup".into())));
    }

    #[test]
    fn test_with_defaults_populated_aaa() {
        // Arrange & Act
        let registry = Skills::with_defaults("DefaultRegistry");

        // Assert
        assert!(registry.skill_count() >= 9);
        assert!(registry.get_skill("mohitmishra786/low-level-dev-skills").is_some());
        assert!(registry.get_skill("alibaba/open-code-review").is_some());
    }

    #[test]
    fn test_search_and_capability_matching_aaa() {
        // Arrange
        let registry = Skills::with_defaults("DefaultRegistry");

        // Act
        let code_review_skills = registry.find_by_capability("code-review");
        let search_results = registry.search("systems");

        // Assert
        assert!(!code_review_skills.is_empty());
        assert!(search_results.iter().any(|s| s.id == "mohitmishra786/low-level-dev-skills"));
    }

    #[test]
    fn test_event_lifecycle_logging_aaa() {
        // Arrange
        let mut m = Skills::new("EventRegistry");
        let event = Event {
            id: "evt-01".into(),
            subject: "anthropics/skills".into(),
            detail: "Skill invoked successfully".into(),
            timestamp: 1732000000,
        };

        // Act
        m.add_event(event).unwrap();

        // Assert
        assert_eq!(m.event_count(), 1);
        let retrieved = m.get_event("evt-01").unwrap();
        assert_eq!(retrieved.subject, "anthropics/skills");
    }
}
