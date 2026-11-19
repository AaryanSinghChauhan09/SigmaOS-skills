use skills::{Event, Skill, SkillCategory, Skills};

#[test]
fn test_skills_lifecycle_and_defaults_aaa() {
    // Arrange
    let registry = Skills::with_defaults("UniversalSkills");

    // Act
    let count = registry.skill_count();
    let low_level = registry.get_skill("mohitmishra786/low-level-dev-skills").unwrap();

    // Assert
    assert!(count >= 9);
    assert_eq!(low_level.category, SkillCategory::LowLevelEngineering);
    assert!(low_level.capabilities.contains(&"low-level-dev".to_string()));
}

#[test]
fn test_skills_capability_resolution_and_filtering_aaa() {
    // Arrange
    let registry = Skills::with_defaults("UniversalSkills");

    // Act
    let review_skills = registry.find_by_capability("code-review");
    let analysis_skills = registry.get_by_category(SkillCategory::DataAnalysis);

    // Assert
    assert_eq!(review_skills.len(), 2);
    assert_eq!(analysis_skills.len(), 1);
    assert_eq!(analysis_skills[0].id, "aiopshwang/data-analysis-ml-agent-skills");
}

#[test]
fn test_custom_skill_registration_and_event_auditing_aaa() {
    // Arrange
    let mut registry = Skills::new("AuditedSkills");
    let skill = Skill::new(
        "zjunlp/SkillNet",
        "zjunlp",
        "SkillNet Knowledge Graph",
        SkillCategory::DataAnalysis,
        "Deep skill network representation",
    )
    .unwrap()
    .with_capability("knowledge-graph")
    .with_trigger("/skillnet");

    // Act
    registry.add_skill(skill).unwrap();
    let event = Event {
        id: "audit-01".into(),
        subject: "zjunlp/SkillNet".into(),
        detail: "Registered with verified capabilities".into(),
        timestamp: 1732000100,
    };
    registry.add_event(event).unwrap();

    // Assert
    assert_eq!(registry.skill_count(), 1);
    assert_eq!(registry.event_count(), 1);
    let s = registry.get_skill("zjunlp/SkillNet").unwrap();
    assert_eq!(s.author, "zjunlp");
    assert!(s.capabilities.contains(&"knowledge-graph".to_string()));
}
