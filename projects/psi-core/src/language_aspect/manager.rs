use std::collections::BTreeMap;

use super::*;

pub static LANGUAGE_REGISTRY_INSTANCE: LazyLock<Mutex<LanguageRegistry>> = LazyLock::new(|| {
    Mutex::new(LanguageRegistry {
        languages: Default::default(),
    })
});

pub struct LanguageRegistry {
    languages: BTreeMap<String, LanguageType>,
}

impl LanguageRegistry {
    pub fn find_language<S: AsRef<str>>(&self, language_id: S) -> Option<&LanguageType> {
        self.languages.get(language_id.as_ref())
    }

    pub fn register_language(&mut self, language: LanguageType) {
        self.languages.insert(language.language_id.as_ref().to_string(), language);
    }
}