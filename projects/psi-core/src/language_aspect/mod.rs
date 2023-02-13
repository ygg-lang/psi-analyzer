use std::borrow::BorrowMut;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::ops::DerefMut;
use std::sync::{LazyLock, Mutex};

pub mod manager;
pub mod id;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct LanguageID(usize);

#[derive(Clone, Debug)]
pub struct LanguageType {
    id: LanguageID,
    name: String,
    base: LanguageID,
    mime_type: &'static str,
}

impl Eq for LanguageType {}

impl PartialEq for LanguageType {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
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
            base: LanguageID::any(),
            mime_type: "",
        }
    }
    pub fn get_parent(&self) -> Option<LanguageID> {
        if self.id.is_any() {
            return None;
        }
        return Some(self.base.language_type().ok()?.id);
    }
}