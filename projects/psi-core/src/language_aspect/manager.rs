use std::collections::BTreeMap;
use std::collections::hash_map::DefaultHasher;

use crate::errors::PsiResult;
use crate::PsiError;

use super::*;

pub static LANGUAGE_REGISTRY_INSTANCE: LazyLock<Mutex<LanguageRegistry>> = LazyLock::new(|| {
    Mutex::new(LanguageRegistry {
        languages: Default::default(),
    })
});

pub struct LanguageRegistry {
    languages: BTreeMap<u64, LanguageType>,
}

impl LanguageRegistry {
    pub fn register_language<S>(language: S, parent: LanguageID) -> PsiResult<LanguageID> where S: Into<String> {
        let name = language.into();
        let mut hasher = DefaultHasher::new();
        name.hash(&mut hasher);
        let id = hasher.finish();
        let mut this = LANGUAGE_REGISTRY_INSTANCE.try_lock()?;
        if this.languages.contains_key(&id) {
            Err(PsiError::runtime_error(format!("Language {} already registered", name)))?;
        }
        this.languages.insert(id, LanguageType {
            id: LanguageID(id),
            name,
            parent,
            mime_type: "",
        });
        Ok(LanguageID(id))
    }
}

impl LanguageRegistry {
    pub fn get_all_languages(&self) -> impl Iterator<Item=LanguageID> + '_ {
        self.languages.iter().map(|l| LanguageID(*l.0))
    }
    pub fn find_language(&self, language: LanguageID) -> Option<&LanguageType> {
        self.languages.get(&language.0)
    }
    pub fn get_parent(&self, language: LanguageID) -> Option<LanguageID> {
        if language.is_any() {
            return None;
        }
        Some(self.find_language(language)?.parent)
    }
}
