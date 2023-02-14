use super::*;

impl Eq for LanguageInstance {}

impl PartialEq for LanguageInstance {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl PartialOrd for LanguageInstance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl Ord for LanguageInstance {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl Hash for LanguageInstance {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl LanguageInstance {
    pub fn instantiate<T>(language: T) -> Self
    where
        T: LanguageType + 'static,
    {
        Self {
            id: T::id(),
            debug_name: T::debug_name(),
            display_name: language.display_id().to_string(),
            parent: language.parent(),
            case_insensitive: language.case_insensitive(),
            file_names: language.file_names(),
            file_extensions: language.file_extensions(),
            file_mimes: language.file_mime(),
        }
    }
}
