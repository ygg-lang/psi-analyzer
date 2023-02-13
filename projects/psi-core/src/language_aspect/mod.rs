use std::borrow::BorrowMut;
use std::cmp::Ordering;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::ops::DerefMut;
use std::sync::{LazyLock, Mutex};

pub mod manager;
pub mod id;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LanguageID(u64);

#[derive(Clone, Debug)]
pub struct LanguageType {
    id: LanguageID,
    name: String,
    parent: LanguageID,
    mime_type: &'static str,
}

impl Eq for LanguageType {}

impl PartialEq for LanguageType {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl PartialOrd for LanguageType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl Ord for LanguageType {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl Hash for LanguageType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl LanguageType {
    pub fn any() -> Self {
        Self {
            id: LanguageID::any(),
            name: "*".to_string(),
            parent: LanguageID::any(),
            mime_type: "",
        }
    }
    pub fn get_parent(&self) -> Option<LanguageID> {
        if self.id.is_any() {
            return None;
        }
        return Some(self.parent.language_type().ok()?.id);
    }
}