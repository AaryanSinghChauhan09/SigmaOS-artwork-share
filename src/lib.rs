// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-mint-artwork-share: mint artwork share component model.
//!
//! Real implementation: shares, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Share {
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
pub enum MintArtworkShareError {
    EmptyShareName,
    DuplicateShare(String),
    EmptyEventSubject,
}

impl fmt::Display for MintArtworkShareError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyShareName => write!(f, "share name must not be empty"),
            Self::DuplicateShare(id) => write!(f, "duplicate share {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for MintArtworkShareError {}

pub struct MintArtworkShare {
    pub name: String,
    shares: BTreeMap<String, Share>,
    events: BTreeMap<String, Event>,
}

impl MintArtworkShare {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            shares: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_share(&mut self, item: Share) -> Result<(), MintArtworkShareError> {
        if item.name.trim().is_empty() {
            return Err(MintArtworkShareError::EmptyShareName);
        }
        let id = item.id.clone();
        if self.shares.contains_key(&id) {
            return Err(MintArtworkShareError::DuplicateShare(id.clone()));
        }
        self.shares.insert(id, item);
        Ok(())
    }

    pub fn share_count(&self) -> usize {
        self.shares.len()
    }

    pub fn get_share(&self, id: &str) -> Option<&Share> {
        self.shares.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), MintArtworkShareError> {
        if event.subject.trim().is_empty() {
            return Err(MintArtworkShareError::EmptyEventSubject);
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

    fn sample_share(id: &str, name: &str) -> Share {
        Share {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = MintArtworkShare::new("MintArtworkShare");
        assert_eq!(m.name, "MintArtworkShare");
        assert_eq!(m.share_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_share() {
        let mut m = MintArtworkShare::new("MintArtworkShare");
        m.add_share(sample_share("a1", "Alpha")).unwrap();
        assert_eq!(m.share_count(), 1);
        assert!(m.get_share("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = MintArtworkShare::new("MintArtworkShare");
        assert!(m.add_share(sample_share("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = MintArtworkShare::new("MintArtworkShare");
        m.add_share(sample_share("x", "One")).unwrap();
        assert!(m.add_share(sample_share("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = MintArtworkShare::new("MintArtworkShare");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
