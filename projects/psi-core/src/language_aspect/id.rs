use std::sync::TryLockResult;
use crate::LANGUAGE_REGISTRY_INSTANCE;
use super::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct LanguageID(&'static str);

impl AsRef<str> for LanguageID {
    fn as_ref(&self) -> &str {
        self.0
    }
}

impl LanguageID {
    pub fn any() -> Self {
        Self::new("")
    }
    pub fn new(id: &'static str) -> Self {
        Self(id)
    }
    pub fn get_type(&self) -> Option<LanguageType> {
        match LANGUAGE_REGISTRY_INSTANCE.lock() {
            Ok(o) => {
                o.find_language(self.0).copied()
            }
            Err(_) => {None}
        }
    }
}


pub struct RustLanguage {}
