use std::collections::hash_map::DefaultHasher;

use crate::{errors::PsiResult, PsiError, LANGUAGE_REGISTRY_INSTANCE};

use super::*;

impl LanguageID {
    pub fn any() -> Self {
        Self(0)
    }
    pub fn is_any(&self) -> bool {
        self.0 == 0
    }
    pub fn new(s: &str) -> LanguageID {
        let normalized = normalize(s);
        let mut hasher = DefaultHasher::new();
        normalized.hash(&mut hasher);
        Self(hasher.finish())
    }

    pub fn language_type(&self) -> PsiResult<LanguageType> {
        match LANGUAGE_REGISTRY_INSTANCE.try_lock()?.find_language(LanguageID(self.0)) {
            Some(s) => Ok(s.clone()),
            None => Err(PsiError::runtime_error(format!("Language {} not found", self.0))),
        }
    }
}

fn normalize(s: &str) -> String {
    s.to_ascii_lowercase()
}
