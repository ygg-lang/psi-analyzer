use std::{
    fmt::Debug,
    ops::Range,
};

pub trait LanguageType {
    fn language_id() -> &'static str;
}

pub struct RustLanguage {}

impl LanguageType for RustLanguage {
    fn language_id() -> LanguageID {
        LanguageID("rust")
    }
}

pub struct LanguageID(&'static str);

impl From<String> for LanguageID {
    fn from(s: String) -> Self {
        Self(s.as_str())
    }
}
}