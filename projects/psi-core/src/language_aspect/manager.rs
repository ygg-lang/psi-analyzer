use std::collections::BTreeMap;

use super::*;

pub static LANGUAGE_REGISTRY_INSTANCE: LazyLock<Mutex<LanguageRegistry>> = LazyLock::new(|| {
    Mutex::new(LanguageRegistry {
        languages: Default::default(),
    })
});

pub struct LanguageRegistry {
    languages: BTreeMap<usize, LanguageType>,
}

impl LanguageRegistry {
    pub fn get_all_languages(&self) -> impl Iterator<Item=LanguageID> + '_ {
        self.languages.iter().map(|(k, _)| LanguageID(*k))
    }
    pub fn find_language(&self, language: LanguageID) -> Option<&LanguageType> {
        self.languages.get(&language.0)
    }
    pub fn register_language(&mut self, language: LanguageType) {
        self.languages.insert(language.id.0, language);
    }
    pub fn get_parent(&self, language: LanguageID) -> Option<LanguageID> {
        if language.is_any() {
            return None;
        }
        Some(self.find_language(language)?.base)
    }
}