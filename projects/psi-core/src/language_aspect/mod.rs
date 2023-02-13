use std::borrow::BorrowMut;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::ops::DerefMut;
use std::sync::{LazyLock, Mutex};

use crate::LanguageID;

pub mod manager;
pub mod id;

#[derive(Copy, Clone, Debug)]
pub struct LanguageType {
    pub language_id: LanguageID,
    pub mime_type: &'static str,
}

impl Eq for LanguageType {}

impl PartialEq for LanguageType {
    fn eq(&self, other: &Self) -> bool {
        self.language_id == other.language_id
    }
}

impl Hash for LanguageType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.language_id.hash(state);
    }
}