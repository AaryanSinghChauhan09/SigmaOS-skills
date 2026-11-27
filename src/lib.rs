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
        const STANDARD_SKILLS: &[(&str, &str, &str, SkillCategory, &str, &str, &str)] = &[
            (
                "anthropics/skills",
                "anthropics",
                "Anthropic Agent Skills",
                SkillCategory::Productivity,
                "Core agent skill primitives and tools",
                "prompt-eval",
                "/anthropic",
            ),
            (
                "addyosmani/agent-skills",
                "addyosmani",
                "Web Performance & Code Review",
                SkillCategory::WebAndDesign,
                "Modern web architecture and performance analysis",
                "web-perf",
                "/web-perf",
            ),
            (
                "alibaba/open-code-review",
                "alibaba",
                "Open Code Review Assistant",
                SkillCategory::CodeReview,
                "Automated code inspection and vulnerability detection",
                "code-review",
                "/review",
            ),
            (
                "akkie76/code-review-skills",
                "akkie76",
                "Code Review Essentials",
                SkillCategory::CodeReview,
                "Systematic PR hygiene and review guidelines",
                "code-review",
                "/audit",
            ),
            (
                "aiopshwang/data-analysis-ml-agent-skills",
                "aiopshwang",
                "ML & Data Analysis Suite",
                SkillCategory::DataAnalysis,
                "Data science pipelines, model evaluation, and EDA",
                "data-analysis",
                "/data-analysis",
            ),
            (
                "mohitmishra786/low-level-dev-skills",
                "mohitmishra786",
                "Low-Level Systems Engineering",
                SkillCategory::LowLevelEngineering,
                "Kernel internals, memory safety, and assembly debugging",
                "low-level-dev",
                "/systems",
            ),
            (
                "cathrynlavery/diagram-design",
                "cathrynlavery",
                "Diagram & Architecture Visualizer",
                SkillCategory::Architecture,
                "Mermaid flowcharts, system diagrams, and UI wireframes",
                "diagram-design",
                "/diagram",
            ),
            (
                "EtienneLescot/n8n-as-code",
                "EtienneLescot",
                "Workflow & Automation Orchestrator",
                SkillCategory::DevOpsAndWorkflows,
                "Workflow pipelines, webhook automation, and integration DAGs",
                "workflows",
                "/workflow",
            ),
            (
                "docmd-io/docmd",
                "docmd-io",
                "Technical Documentation Engine",
                SkillCategory::Productivity,
                "Markdown documentation generator and spec validator",
                "documentation",
                "/doc",
            ),
            (
                "affaan-m/ECC",
                "affaan-m",
                "ECC Agent Workflows",
                SkillCategory::DevOpsAndWorkflows,
                "Structured engineering workflow skills",
                "workflows",
                "/ecc",
            ),
            (
                "AgainstStoneGold/rpocket-team-antigravity-skill-engine",
                "AgainstStoneGold",
                "RPocket Antigravity Skill Engine",
                SkillCategory::DevOpsAndWorkflows,
                "Team skill orchestration engine",
                "skill-engine",
                "/rpocket",
            ),
            (
                "agentskills/agentskills",
                "agentskills",
                "AgentSkills Standard Catalog",
                SkillCategory::Productivity,
                "Open specification and catalog for agent skills",
                "skill-spec",
                "/agentskills",
            ),
            (
                "aklofas/kicad-happy",
                "aklofas",
                "KiCad Happy PCB Design",
                SkillCategory::LowLevelEngineering,
                "Schematic and PCB layout automation skills",
                "eda-pcb",
                "/kicad",
            ),
            (
                "Bhanunamikaze/Agentic-SEO-Skill",
                "Bhanunamikaze",
                "Agentic SEO Skill",
                SkillCategory::WebAndDesign,
                "Technical SEO auditing and structured metadata optimization",
                "seo",
                "/seo",
            ),
            (
                "blader/humanizer",
                "blader",
                "Humanizer Writing Polish",
                SkillCategory::Productivity,
                "Natural prose editing and clarity refinement",
                "writing",
                "/humanize",
            ),
            (
                "borghei/Claude-Skills",
                "borghei",
                "Claude Skills Collection",
                SkillCategory::Productivity,
                "Reusable developer and reasoning skill templates",
                "prompting",
                "/claude-skills",
            ),
            (
                "buiquangvu235/antigravity-skills-engine",
                "buiquangvu235",
                "Antigravity Skills Engine",
                SkillCategory::DevOpsAndWorkflows,
                "Modular skill execution and routing framework",
                "skill-engine",
                "/ag-engine",
            ),
            (
                "career-ops-hq/career-ops",
                "career-ops-hq",
                "Career Ops Assistant",
                SkillCategory::Productivity,
                "Resume, role matching, and technical interview preparation",
                "career-ops",
                "/career",
            ),
            (
                "ciembor/agent-rules-books",
                "ciembor",
                "Agent Rules & Playbooks",
                SkillCategory::Architecture,
                "Engineering rulebooks and coding standards for agents",
                "rulebooks",
                "/rules",
            ),
            (
                "ComposioHQ/awesome-claude-skills",
                "ComposioHQ",
                "Awesome Claude Skills",
                SkillCategory::Productivity,
                "Curated tool integrations and agent skill recipes",
                "integrations",
                "/composio",
            ),
            (
                "coreyhaines31/marketingskills",
                "coreyhaines31",
                "Marketing Skills Suite",
                SkillCategory::Productivity,
                "Product positioning, copywriting, and growth workflows",
                "marketing",
                "/marketing",
            ),
            (
                "CreminiAI/skillpack",
                "CreminiAI",
                "CreminiAI SkillPack",
                SkillCategory::DevOpsAndWorkflows,
                "Composable agent skill bundles",
                "skill-bundle",
                "/skillpack",
            ),
            (
                "DietrichGebert/ponytail",
                "DietrichGebert",
                "Ponytail Agent Toolkit",
                SkillCategory::Productivity,
                "Lightweight developer automation skills",
                "automation",
                "/ponytail",
            ),
            (
                "EliasOulkadi/shokunin",
                "EliasOulkadi",
                "Shokunin Craftsmanship",
                SkillCategory::CodeReview,
                "Clean code craftsmanship and refactoring discipline",
                "refactoring",
                "/shokunin",
            ),
            (
                "emilkowalski/skills",
                "emilkowalski",
                "UI Motion & Design Skills",
                SkillCategory::WebAndDesign,
                "Polished UI interactions, transitions, and component craft",
                "ui-motion",
                "/emil-ui",
            ),
            (
                "foryourhealth111-pixel/Vibe-Skills",
                "foryourhealth111-pixel",
                "Vibe Skills Pack",
                SkillCategory::WebAndDesign,
                "Rapid prototyping and creative coding skills",
                "prototyping",
                "/vibe",
            ),
            (
                "gabelul/slopbuster",
                "gabelul",
                "Slopbuster Quality Gate",
                SkillCategory::CodeReview,
                "Eliminates boilerplate bloat and enforces concise code",
                "code-quality",
                "/slopbuster",
            ),
            (
                "gabelul/stitch-kit",
                "gabelul",
                "Stitch Kit UI Builder",
                SkillCategory::WebAndDesign,
                "UI composition and design-to-code toolkit",
                "ui-design",
                "/stitch",
            ),
            (
                "google/skills",
                "google",
                "Google Agent Skills",
                SkillCategory::Architecture,
                "Foundational agent capabilities and best-practice skills",
                "agent-core",
                "/google-skills",
            ),
            (
                "guanyang/open-agent-hub",
                "guanyang",
                "Open Agent Hub",
                SkillCategory::DevOpsAndWorkflows,
                "Multi-agent skill registry and workflow hub",
                "agent-hub",
                "/agent-hub",
            ),
            (
                "hesreallyhim/awesome-claude-code",
                "hesreallyhim",
                "Awesome Claude Code",
                SkillCategory::Productivity,
                "Curated slash commands, hooks, and coding workflows",
                "workflows",
                "/awesome-cc",
            ),
            (
                "jame581/GodotPrompter",
                "jame581",
                "Godot Prompter",
                SkillCategory::WebAndDesign,
                "Godot game engine scene and GDScript assistant",
                "gamedev",
                "/godot",
            ),
            (
                "jomeswang/agnes-ai-skill",
                "jomeswang",
                "Agnes AI Skill",
                SkillCategory::DataAnalysis,
                "Analytical reasoning and structured research skills",
                "research",
                "/agnes",
            ),
            (
                "JPeetz/agent-skills",
                "JPeetz",
                "JPeetz Agent Skills",
                SkillCategory::Productivity,
                "Developer productivity and task automation skills",
                "automation",
                "/jpeetz",
            ),
            (
                "K-Dense-AI/scientific-agent-skills",
                "K-Dense-AI",
                "Scientific Agent Skills",
                SkillCategory::DataAnalysis,
                "Scientific computing, bioinformatics, and literature analysis",
                "scientific-computing",
                "/science",
            ),
            (
                "kangarooking/cangjie-skill",
                "kangarooking",
                "Cangjie Language Skill",
                SkillCategory::LowLevelEngineering,
                "Cangjie systems and application language support",
                "cangjie",
                "/cangjie",
            ),
            (
                "lee-to/ai-factory",
                "lee-to",
                "AI Factory Pipeline",
                SkillCategory::DevOpsAndWorkflows,
                "Automated software delivery and agent factory pipelines",
                "ci-cd",
                "/ai-factory",
            ),
            (
                "Leon-Drq/openagentskill",
                "Leon-Drq",
                "OpenAgentSkill",
                SkillCategory::Productivity,
                "Open cross-runtime agent skill definitions",
                "skill-spec",
                "/openagentskill",
            ),
            (
                "Leonxlnx/taste-skill",
                "Leonxlnx",
                "Taste UI & Frontend Skill",
                SkillCategory::WebAndDesign,
                "High-craft visual design and frontend aesthetics",
                "ui-design",
                "/taste",
            ),
            (
                "Light0305/Light-Science",
                "Light0305",
                "Light Science Suite",
                SkillCategory::DataAnalysis,
                "Quantitative modeling and scientific data analysis",
                "science",
                "/light-science",
            ),
            (
                "majiayu000/spellbook",
                "majiayu000",
                "Agent Spellbook",
                SkillCategory::Productivity,
                "Composable prompt and workflow spellbook",
                "prompting",
                "/spellbook",
            ),
            (
                "mattpocock/skills",
                "mattpocock",
                "TypeScript & Architecture Skills",
                SkillCategory::Architecture,
                "Type-safe TypeScript design, testing, and planning skills",
                "typescript",
                "/matt-skills",
            ),
            (
                "moonlight-lupin/agent-skills",
                "moonlight-lupin",
                "Moonlight Agent Skills",
                SkillCategory::Productivity,
                "General-purpose coding and review skill pack",
                "coding",
                "/moonlight",
            ),
            (
                "morluto/rea",
                "morluto",
                "REA Engineering Assistant",
                SkillCategory::Architecture,
                "Requirements engineering and architecture analysis",
                "architecture",
                "/rea",
            ),
            (
                "nexscope-ai/eCommerce-Skills",
                "nexscope-ai",
                "eCommerce Skills",
                SkillCategory::WebAndDesign,
                "Storefront, catalog, and checkout workflow skills",
                "ecommerce",
                "/ecommerce",
            ),
            (
                "nextlevelbuilder/ui-ux-pro-max-skill",
                "nextlevelbuilder",
                "UI/UX Pro Max Skill",
                SkillCategory::WebAndDesign,
                "Design systems, accessibility, and responsive layout skills",
                "ui-ux",
                "/ui-ux",
            ),
            (
                "nexu-io/open-design",
                "nexu-io",
                "Open Design System",
                SkillCategory::WebAndDesign,
                "Design tokens, component specs, and visual QA",
                "design-system",
                "/open-design",
            ),
            (
                "nisabmohd/aria-docs",
                "nisabmohd",
                "Aria Docs Generator",
                SkillCategory::Productivity,
                "Structured technical documentation and MDX authoring",
                "documentation",
                "/aria-docs",
            ),
            (
                "nledford/engineering-review-board",
                "nledford",
                "Engineering Review Board",
                SkillCategory::CodeReview,
                "Multi-perspective architecture and PR review board",
                "code-review",
                "/review-board",
            ),
            (
                "nolangz/pixel2svg-html",
                "nolangz",
                "Pixel to SVG/HTML Converter",
                SkillCategory::WebAndDesign,
                "Raster-to-vector SVG and semantic HTML conversion",
                "svg-html",
                "/pixel2svg",
            ),
            (
                "NVIDIA/SkillSpector",
                "NVIDIA",
                "NVIDIA SkillSpector",
                SkillCategory::LowLevelEngineering,
                "GPU compute profiling and CUDA kernel inspection skills",
                "gpu-profiling",
                "/skillspector",
            ),
            (
                "opensite-ai/opensite-skills",
                "opensite-ai",
                "OpenSite Web Skills",
                SkillCategory::WebAndDesign,
                "Static site generation and web publishing skills",
                "web-publishing",
                "/opensite",
            ),
            (
                "paperclipai/paperclip",
                "paperclipai",
                "Paperclip Agent Orchestrator",
                SkillCategory::DevOpsAndWorkflows,
                "Task delegation and agent coordination workflows",
                "orchestration",
                "/paperclip",
            ),
            (
                "pbakaus/impeccable",
                "pbakaus",
                "Impeccable Craft & UX",
                SkillCategory::WebAndDesign,
                "Frontend polish, typography, and interaction quality",
                "ui-craft",
                "/impeccable",
            ),
            (
                "quysto/QUYQUY-AntigravityAwesomeSkills-GodVersion-bumbum",
                "quysto",
                "Antigravity Awesome Skills",
                SkillCategory::Productivity,
                "Comprehensive community skill collection",
                "skill-pack",
                "/quyquy",
            ),
            (
                "rebelytics/one-skill-to-rule-them-all",
                "rebelytics",
                "One Skill Meta-Router",
                SkillCategory::Architecture,
                "Unified skill routing and context-aware dispatch",
                "routing",
                "/one-skill",
            ),
            (
                "rorkai/app-store-connect-cli-skills",
                "rorkai",
                "App Store Connect CLI Skills",
                SkillCategory::DevOpsAndWorkflows,
                "Mobile release automation and store metadata management",
                "mobile-release",
                "/asc-cli",
            ),
            (
                "seb1n/awesome-ai-agent-skills",
                "seb1n",
                "Awesome AI Agent Skills",
                SkillCategory::Productivity,
                "Curated directory of AI agent skills and tools",
                "skill-catalog",
                "/awesome-ai",
            ),
            (
                "Shubhamsaboo/awesome-llm-apps",
                "Shubhamsaboo",
                "Awesome LLM Apps",
                SkillCategory::DataAnalysis,
                "LLM application patterns, RAG, and agentic workflows",
                "llm-apps",
                "/llm-apps",
            ),
            (
                "sickn33/agentic-awesome-skills",
                "sickn33",
                "Agentic Awesome Skills",
                SkillCategory::DevOpsAndWorkflows,
                "Autonomous agent workflows and engineering skills",
                "agentic-workflows",
                "/agentic",
            ),
            (
                "SkillsCat/skillscat",
                "SkillsCat",
                "SkillsCat Registry",
                SkillCategory::Productivity,
                "Searchable skill taxonomy and package index",
                "skill-catalog",
                "/skillscat",
            ),
            (
                "taracodlabs/aiden",
                "taracodlabs",
                "Aiden Engineering Agent",
                SkillCategory::CodeReview,
                "Automated debugging, test generation, and code review",
                "code-review",
                "/aiden",
            ),
            (
                "taxueseek/plugin-fast-adapt",
                "taxueseek",
                "Plugin Fast Adapt",
                SkillCategory::DevOpsAndWorkflows,
                "Rapid plugin adaptation and schema migration",
                "plugin-adapt",
                "/fast-adapt",
            ),
            (
                "team-telnyx/ai",
                "team-telnyx",
                "Telnyx AI Telephony Skills",
                SkillCategory::DevOpsAndWorkflows,
                "Voice, messaging, and real-time communications AI skills",
                "telephony-ai",
                "/telnyx",
            ),
            (
                "tt-a1i/archify",
                "tt-a1i",
                "Archify System Architect",
                SkillCategory::Architecture,
                "System architecture blueprints and dependency mapping",
                "architecture",
                "/archify",
            ),
            (
                "vercel-labs/agent-skills",
                "vercel-labs",
                "Vercel Agent Skills",
                SkillCategory::WebAndDesign,
                "Next.js, React, and edge deployment best practices",
                "web-framework",
                "/vercel",
            ),
            (
                "VidyFoo/antigravity-skill-engine",
                "VidyFoo",
                "VidyFoo Antigravity Skill Engine",
                SkillCategory::DevOpsAndWorkflows,
                "Deterministic skill execution and chaining",
                "skill-engine",
                "/vidyfoo",
            ),
            (
                "VoltAgent/awesome-agent-skills",
                "VoltAgent",
                "VoltAgent Awesome Skills",
                SkillCategory::Productivity,
                "Curated multi-framework agent skills",
                "skill-catalog",
                "/voltagent",
            ),
            (
                "VoltAgent/awesome-openclaw-skills",
                "VoltAgent",
                "Awesome OpenClaw Skills",
                SkillCategory::DevOpsAndWorkflows,
                "OpenClaw automation and tool-use skills",
                "openclaw",
                "/openclaw-skills",
            ),
            (
                "wondelai/skills",
                "wondelai",
                "Wondelai Skill Suite",
                SkillCategory::Productivity,
                "Focused task execution and synthesis skills",
                "synthesis",
                "/wondelai",
            ),
            (
                "zaoshangduziteng/web-reconstruction",
                "zaoshangduziteng",
                "Web Reconstruction Skill",
                SkillCategory::WebAndDesign,
                "DOM analysis and frontend layout reconstruction",
                "web-reconstruction",
                "/web-recon",
            ),
            (
                "zjunlp/SkillNet",
                "zjunlp",
                "ZJU NLP SkillNet",
                SkillCategory::DataAnalysis,
                "Structured skill graph and compositional reasoning",
                "skill-graph",
                "/skillnet",
            ),
        ];

        for &(id, author, name, category, description, cap, trig) in STANDARD_SKILLS {
            if let Ok(skill) = Skill::new(id, author, name, category, description) {
                let _ = self.add_skill(skill.with_capability(cap).with_trigger(trig));
            }
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
        self.skills
            .values()
            .filter(|s| s.category == category)
            .collect()
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
        let s1 = Skill::new(
            "test/dup",
            "author",
            "Name",
            SkillCategory::CodeReview,
            "desc",
        )
        .unwrap();
        let s2 = Skill::new(
            "test/dup",
            "author",
            "Name 2",
            SkillCategory::CodeReview,
            "desc",
        )
        .unwrap();
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
        assert!(registry.skill_count() >= 72);
        assert!(registry
            .get_skill("mohitmishra786/low-level-dev-skills")
            .is_some());
        assert!(registry.get_skill("alibaba/open-code-review").is_some());
        assert!(registry.get_skill("NVIDIA/SkillSpector").is_some());
        assert!(registry.get_skill("zjunlp/SkillNet").is_some());
    }

    #[test]
    fn test_with_defaults_covers_all_categories_aaa() {
        // Arrange
        let registry = Skills::with_defaults("DefaultRegistry");
        let categories = [
            SkillCategory::CodeReview,
            SkillCategory::DataAnalysis,
            SkillCategory::Architecture,
            SkillCategory::LowLevelEngineering,
            SkillCategory::WebAndDesign,
            SkillCategory::DevOpsAndWorkflows,
            SkillCategory::Productivity,
        ];

        // Act & Assert
        for cat in categories {
            assert!(
                !registry.get_by_category(cat).is_empty(),
                "expected at least one skill in category {}",
                cat.as_str()
            );
        }
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
        assert!(search_results
            .iter()
            .any(|s| s.id == "mohitmishra786/low-level-dev-skills"));
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
