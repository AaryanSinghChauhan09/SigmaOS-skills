// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-skills: skills component model.
//!
//! Real implementation: skills, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Skill {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SkillsError {
    EmptySkillName,
    DuplicateSkill(String),
    EmptyEventSubject,
}

impl fmt::Display for SkillsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptySkillName => write!(f, "skill name must not be empty"),
            Self::DuplicateSkill(id) => write!(f, "duplicate skill {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for SkillsError {}

pub struct Skills {
    pub name: String,
    skills: BTreeMap<String, Skill>,
    events: BTreeMap<String, Event>,
}

impl Skills {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            skills: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_skill(&mut self, item: Skill) -> Result<(), SkillsError> {
        if item.name.trim().is_empty() {
            return Err(SkillsError::EmptySkillName);
        }
        let id = item.id.clone();
        if self.skills.contains_key(&id) {
            return Err(SkillsError::DuplicateSkill(id.clone()));
        }
        self.skills.insert(id, item);
        Ok(())
    }

    pub fn skill_count(&self) -> usize {
        self.skills.len()
    }

    pub fn get_skill(&self, id: &str) -> Option<&Skill> {
        self.skills.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), SkillsError> {
        if event.subject.trim().is_empty() {
            return Err(SkillsError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_skill(id: &str, name: &str) -> Skill {
        Skill {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Skills::new("Skills");
        assert_eq!(m.name, "Skills");
        assert_eq!(m.skill_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_skill() {
        let mut m = Skills::new("Skills");
        m.add_skill(sample_skill("a1", "Alpha")).unwrap();
        assert_eq!(m.skill_count(), 1);
        assert!(m.get_skill("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Skills::new("Skills");
        assert!(m.add_skill(sample_skill("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Skills::new("Skills");
        m.add_skill(sample_skill("x", "One")).unwrap();
        assert!(m.add_skill(sample_skill("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Skills::new("Skills");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
