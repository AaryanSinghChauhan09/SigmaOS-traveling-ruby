// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-traveling-ruby: traveling ruby component model.
//!
//! Real implementation: rubys, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ruby {
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
pub enum TravelingRubyError {
    EmptyRubyName,
    DuplicateRuby(String),
    EmptyEventSubject,
}

impl fmt::Display for TravelingRubyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyRubyName => write!(f, "ruby name must not be empty"),
            Self::DuplicateRuby(id) => write!(f, "duplicate ruby {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for TravelingRubyError {}

pub struct TravelingRuby {
    pub name: String,
    rubys: BTreeMap<String, Ruby>,
    events: BTreeMap<String, Event>,
}

impl TravelingRuby {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            rubys: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_ruby(&mut self, item: Ruby) -> Result<(), TravelingRubyError> {
        if item.name.trim().is_empty() {
            return Err(TravelingRubyError::EmptyRubyName);
        }
        let id = item.id.clone();
        if self.rubys.contains_key(&id) {
            return Err(TravelingRubyError::DuplicateRuby(id.clone()));
        }
        self.rubys.insert(id, item);
        Ok(())
    }

    pub fn ruby_count(&self) -> usize {
        self.rubys.len()
    }

    pub fn get_ruby(&self, id: &str) -> Option<&Ruby> {
        self.rubys.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), TravelingRubyError> {
        if event.subject.trim().is_empty() {
            return Err(TravelingRubyError::EmptyEventSubject);
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

    fn sample_ruby(id: &str, name: &str) -> Ruby {
        Ruby {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = TravelingRuby::new("TravelingRuby");
        assert_eq!(m.name, "TravelingRuby");
        assert_eq!(m.ruby_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_ruby() {
        let mut m = TravelingRuby::new("TravelingRuby");
        m.add_ruby(sample_ruby("a1", "Alpha")).unwrap();
        assert_eq!(m.ruby_count(), 1);
        assert!(m.get_ruby("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = TravelingRuby::new("TravelingRuby");
        assert!(m.add_ruby(sample_ruby("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = TravelingRuby::new("TravelingRuby");
        m.add_ruby(sample_ruby("x", "One")).unwrap();
        assert!(m.add_ruby(sample_ruby("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = TravelingRuby::new("TravelingRuby");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
