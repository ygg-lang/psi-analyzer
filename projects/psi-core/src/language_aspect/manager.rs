use dashmap::{mapref::multiple::RefMulti, DashMap};

use super::*;

pub static LANGUAGE_REGISTRY_INSTANCE: LazyLock<LanguageRegistry> = LazyLock::new(|| {
    let mut languages = DashMap::new();
    languages.insert(LanguageID::any().0, LanguageInstance::any());
    LanguageRegistry { languages }
});

pub struct LanguageRegistry {
    languages: DashMap<u64, LanguageInstance>,
}

impl Debug for LanguageRegistry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut languages = vec![];
        for instance in self.languages.iter() {
            languages.push(instance.display_name.clone());
        }
        f.debug_struct("LanguageRegistry").field("languages", &languages).finish()
    }
}

impl LanguageRegistry {
    pub fn register_language<T: LanguageType>(language: T) -> PsiResult<LanguageID> {
        let lang = LanguageInstance::instantiate(language);
        if LANGUAGE_REGISTRY_INSTANCE.languages.contains_key(&lang.id.0) {
            Err(PsiError::runtime_error(format!("Language {} already registered", lang.display_name)))?;
        }
        let key = lang.id.0;
        LANGUAGE_REGISTRY_INSTANCE.languages.insert(key, lang);
        Ok(lang.id)
    }
    pub fn override_parent(language: LanguageID, parent: LanguageID) -> PsiResult<()> {
        match LANGUAGE_REGISTRY_INSTANCE.languages.get_mut(&language.0) {
            Some(mut l) => l.parent = parent,
            None => Err(PsiError::runtime_error("Language not registered"))?,
        }
        Ok(())
    }
    pub fn unregister_language(language: LanguageID) -> PsiResult<()> {
        LANGUAGE_REGISTRY_INSTANCE.languages.remove(&language.0);
        Ok(())
    }
}

impl LanguageRegistry {
    pub fn get_all_languages() -> impl Iterator<Item = LanguageID> + 'static {
        LANGUAGE_REGISTRY_INSTANCE.languages.iter().map(|l| l.id)
    }
    pub fn find_language(language: LanguageID) -> Option<LanguageInstance> {
        let item = LANGUAGE_REGISTRY_INSTANCE.languages.get(&language.0)?;
        Some(item.value().clone())
    }
    pub fn get_parent(language: LanguageID) -> Option<LanguageID> {
        if language.is_any() {
            return None;
        }
        let item = LANGUAGE_REGISTRY_INSTANCE.languages.get(&language.0)?;
        Some(item.parent)
    }
    pub fn get_dialects(language: LanguageID) -> impl Iterator<Item = LanguageID> + 'static {
        LANGUAGE_REGISTRY_INSTANCE.languages.iter().filter_map(move |l| select_dialect(l, language))
    }
}

fn select_dialect(item: RefMulti<u64, LanguageInstance>, parent: LanguageID) -> Option<LanguageID> {
    if parent == item.id {
        return None;
    }
    if item.parent == parent { Some(item.id) } else { None }
}
