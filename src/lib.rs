// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-keepalived: keepalived component model.
//!
//! Real implementation: keepalived_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeepalivedItem {
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
pub enum KeepalivedError {
    EmptyKeepalivedItemName,
    DuplicateKeepalivedItem(String),
    EmptyEventSubject,
}

impl fmt::Display for KeepalivedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyKeepalivedItemName => write!(f, "keepalived_item name must not be empty"),
            Self::DuplicateKeepalivedItem(id) => write!(f, "duplicate keepalived_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for KeepalivedError {}

pub struct Keepalived {
    pub name: String,
    keepalived_items: BTreeMap<String, KeepalivedItem>,
    events: BTreeMap<String, Event>,
}

impl Keepalived {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            keepalived_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_keepalived_item(&mut self, item: KeepalivedItem) -> Result<(), KeepalivedError> {
        if item.name.trim().is_empty() {
            return Err(KeepalivedError::EmptyKeepalivedItemName);
        }
        let id = item.id.clone();
        if self.keepalived_items.contains_key(&id) {
            return Err(KeepalivedError::DuplicateKeepalivedItem(id.clone()));
        }
        self.keepalived_items.insert(id, item);
        Ok(())
    }

    pub fn keepalived_item_count(&self) -> usize {
        self.keepalived_items.len()
    }

    pub fn get_keepalived_item(&self, id: &str) -> Option<&KeepalivedItem> {
        self.keepalived_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), KeepalivedError> {
        if event.subject.trim().is_empty() {
            return Err(KeepalivedError::EmptyEventSubject);
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

    fn sample_keepalived_item(id: &str, name: &str) -> KeepalivedItem {
        KeepalivedItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Keepalived::new("Keepalived");
        assert_eq!(m.name, "Keepalived");
        assert_eq!(m.keepalived_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_keepalived_item() {
        let mut m = Keepalived::new("Keepalived");
        m.add_keepalived_item(sample_keepalived_item("a1", "Alpha")).unwrap();
        assert_eq!(m.keepalived_item_count(), 1);
        assert!(m.get_keepalived_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Keepalived::new("Keepalived");
        assert!(m.add_keepalived_item(sample_keepalived_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Keepalived::new("Keepalived");
        m.add_keepalived_item(sample_keepalived_item("x", "One")).unwrap();
        assert!(m.add_keepalived_item(sample_keepalived_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Keepalived::new("Keepalived");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
