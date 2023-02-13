use super::*;

pub static LANGUAGE_REGISTRY_INSTANCE: LazyLock<Mutex<LanguageRegistry>> = LazyLock::new(|| {
    let mut languages = BTreeMap::new();
    languages.insert(LanguageID::any().0, LanguageType::any());
    Mutex::new(LanguageRegistry { languages })
});

pub struct LanguageRegistry {
    languages: BTreeMap<u64, LanguageType>,
}

impl Debug for LanguageRegistry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let languages = self.languages.values().map(|s| s.name.as_str()).collect::<Vec<_>>();
        f.debug_struct("LanguageRegistry").field("languages", &languages).finish()
    }
}

impl LanguageRegistry {
    pub fn register_language<S>(language: S, parent: LanguageID) -> PsiResult<LanguageID>
    where
        S: Into<String>,
    {
        let name = language.into();
        let id = LanguageID::new(&name);
        let mut this = LANGUAGE_REGISTRY_INSTANCE.try_lock()?;
        if this.languages.contains_key(&id.0) {
            Err(PsiError::runtime_error(format!("Language {} already registered", name)))?;
        }
        this.languages.insert(id.0, LanguageType { id, name, parent, mime_type: "" });
        Ok(id)
    }
    pub fn override_parent(language: LanguageID, parent: LanguageID) -> PsiResult<()> {
        let mut this = LANGUAGE_REGISTRY_INSTANCE.try_lock()?;
        match this.languages.get_mut(&language.0) {
            Some(l) => l.parent = parent,
            None => Err(PsiError::runtime_error("Language not registered"))?,
        }
        Ok(())
    }
    pub fn unregister_language(language: LanguageID) -> PsiResult<()> {
        let mut this = LANGUAGE_REGISTRY_INSTANCE.try_lock()?;
        this.languages.remove(&language.0);
        Ok(())
    }
}

impl LanguageRegistry {
    pub fn get_all_languages(&self) -> impl Iterator<Item = LanguageID> + '_ {
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
    pub fn get_dialects(&self, language: LanguageID) -> impl Iterator<Item = LanguageID> + '_ {
        self.languages.iter().filter_map(move |l| if l.1.parent == language { Some(LanguageID(*l.0)) } else { None })
    }
}
