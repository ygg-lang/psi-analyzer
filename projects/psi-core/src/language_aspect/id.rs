use std::collections::hash_map::DefaultHasher;

use crate::{LANGUAGE_REGISTRY_INSTANCE, PsiError};
use crate::errors::PsiResult;

use super::*;

impl LanguageID {
    pub fn any() -> Self {
        Self(0)
    }
    pub fn is_any(&self) -> bool {
        self.0 == 0
    }
    pub fn new(s: &str) -> LanguageID {
        let mut hasher = DefaultHasher::new();
        s.hash(&mut hasher);
        Self(hasher.finish())
    }


    pub fn language_type(&self) -> PsiResult<LanguageType> {
        match LANGUAGE_REGISTRY_INSTANCE.try_lock()?.find_language(LanguageID(self.0)) {
            Some(s) => {
                Ok(s.clone())
            }
            None => {
                Err(PsiError::runtime_error(format!("Language {} not found", self.0)))
            }
        }
    }
}

