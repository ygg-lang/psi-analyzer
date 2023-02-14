use super::*;

#[derive(Clone, Default)]
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
    pub fn register_language<T>(&self, language: T) -> PsiResult<LanguageID>
    where
        T: LanguageType + 'static,
    {
        let lang = LanguageInstance::instantiate(language);
        if self.languages.contains_key(&lang.id.0) {
            Err(PsiError::runtime_error(format!("Language {} already registered", lang.debug_name)))?;
        }
        let key = lang.id.0;
        self.languages.insert(key, lang);
        Ok(LanguageID(key))
    }
    pub fn override_parent(&self, language: LanguageID, parent: Option<LanguageID>) -> PsiResult<()> {
        match self.languages.get_mut(&language.0) {
            Some(mut l) => l.parent = parent,
            None => Err(PsiError::runtime_error("Language does not registered"))?,
        }
        Ok(())
    }
    pub fn unregister_language(&self, language: LanguageID) -> PsiResult<()> {
        self.languages.remove(&language.0);
        Ok(())
    }
}

impl LanguageRegistry {
    pub fn get_language(&self, language: LanguageID) -> Option<LanguageInstance> {
        let item = self.languages.get(&language.0)?;
        Some(item.value().clone())
    }
    pub fn get_all_languages<'a>(&'a self) -> impl Iterator<Item = LanguageInstance> + 'a {
        self.languages.iter().map(|l| l.value().clone())
    }
    pub fn get_base(&self, language: LanguageID) -> Option<LanguageID> {
        self.languages.get(&language.0)?.parent
    }
    pub fn get_dialects<'a>(&'a self, language: LanguageID) -> impl Iterator<Item = LanguageID> + 'a {
        self.languages.iter().filter_map(move |l| select_dialect(l, language))
    }
}

fn select_dialect(item: RefMulti<u64, LanguageInstance>, parent: LanguageID) -> Option<LanguageID> {
    if parent == item.id {
        return None;
    }
    if Some(parent) == item.parent { Some(item.id) } else { None }
}
