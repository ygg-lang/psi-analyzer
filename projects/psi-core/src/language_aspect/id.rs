use super::*;

pub struct LanguageID(&'static str);

impl LanguageID {
    pub fn any() -> Self {
        Self::new("")
    }
    pub fn new(id: &'static str) -> Self {
        Self(id)
    }
}


pub struct RustLanguage {}

impl LanguageType for RustLanguage {
    fn language_id() -> LanguageID {
        LanguageID("rust")
    }

    fn mime_type() -> &'static str {
        todo!()
    }
}