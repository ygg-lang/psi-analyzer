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
            display_id: language.,
            parent: T::parent(&language),
            file_name: T::file_names(&language),
            file_extension: T::file_extensions(&language),
            mime: T::file_mime(&language),
        }
    }
}
