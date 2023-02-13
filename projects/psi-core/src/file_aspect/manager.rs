use super::*;

pub static File_REGISTRY_INSTANCE: LazyLock<FileRegistry> = LazyLock::new(|| {
    let mut languages = DashMap::new();
    languages.insert(FileID::any().0, LanguageInstance::any());
    FileRegistry { languages }
});

pub struct FileRegistry {
    languages: DashMap<u64, LanguageInstance>,
}

impl Debug for FileRegistry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut languages = vec![];
        for instance in self.languages.iter() {
            languages.push(instance.name.clone());
        }
        f.debug_struct("LanguageRegistry").field("languages", &languages).finish()
    }
}

impl FileRegistry {
    pub fn register_language<S>(language: S, parent: LanguageID) -> PsiResult<LanguageID>
    where
        S: Into<String>,
    {
        let name = language.into();
        let id = LanguageID::new(&name);
        if File_REGISTRY_INSTANCE.languages.contains_key(&id.0) {
            Err(PsiError::runtime_error(format!("Language {} already registered", name)))?;
        }
        File_REGISTRY_INSTANCE.languages.insert(id.0, LanguageInstance { id, name, parent, mime_type: "" });
        Ok(id)
    }
    pub fn override_parent(language: LanguageID, parent: LanguageID) -> PsiResult<()> {
        match File_REGISTRY_INSTANCE.languages.get_mut(&language.0) {
            Some(mut l) => l.parent = parent,
            None => Err(PsiError::runtime_error("Language not registered"))?,
        }
        Ok(())
    }
    pub fn unregister_language(language: LanguageID) -> PsiResult<()> {
        File_REGISTRY_INSTANCE.languages.remove(&language.0);
        Ok(())
    }
}

impl FileRegistry {
    pub fn get_all_languages() -> impl Iterator<Item = LanguageID> + 'static {
        File_REGISTRY_INSTANCE.languages.iter().map(|l| l.id)
    }
    pub fn find_language(language: LanguageID) -> Option<LanguageInstance> {
        let item = File_REGISTRY_INSTANCE.languages.get(&language.0)?;
        Some(item.value().clone())
    }
    pub fn get_parent(language: LanguageID) -> Option<LanguageID> {
        if language.is_any() {
            return None;
        }
        let item = File_REGISTRY_INSTANCE.languages.get(&language.0)?;
        Some(item.parent)
    }
    pub fn get_dialects(language: LanguageID) -> impl Iterator<Item = LanguageID> + 'static {
        File_REGISTRY_INSTANCE.languages.iter().filter_map(move |l| select_dialect(l, language))
    }
}

fn select_dialect(item: RefMulti<u64, LanguageInstance>, parent: LanguageID) -> Option<LanguageID> {
    if parent == item.id {
        return None;
    }
    if item.parent == parent { Some(item.id) } else { None }
}
