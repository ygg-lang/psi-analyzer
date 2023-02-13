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
    pub fn get_parent(&self) -> Option<LanguageID> {
        if self.id.is_any() {
            return None;
        }
        return Some(self.parent.language_type().ok()?.id);
    }
}
