use super::*;
use crate::LanguageRegistry;

impl LanguageID {
    pub fn new(s: &str) -> LanguageID {
        let normalized = normalize(s);
        let mut hasher = DefaultHasher::new();
        normalized.hash(&mut hasher);
        Self(hasher.finish())
    }

    pub fn language_type(&self, registry: &LanguageRegistry) -> PsiResult<LanguageInstance> {
        match registry.find_language(LanguageID(self.0)) {
            Some(s) => Ok(s.clone()),
            None => Err(PsiError::runtime_error(format!("Language {} not found", self.0))),
        }
    }
}

fn normalize(s: &str) -> String {
    s.to_ascii_lowercase()
}
