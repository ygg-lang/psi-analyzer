use std::ops::Deref;

use psi_core::{LanguageID, LanguageRegistry, LanguageType, LANGUAGE_REGISTRY_INSTANCE};

#[test]
fn rust_language() {
    LanguageRegistry::register_language(RustLanguage).ok();
    LanguageRegistry::register_language(JavascriptLanguage).ok();
    LanguageRegistry::register_language(JavascriptXLanguage).ok();
    println!("{:?}", LANGUAGE_REGISTRY_INSTANCE.deref());
}

pub struct RustLanguage;

pub struct JavascriptLanguage;

pub struct JavascriptXLanguage;

impl LanguageType for RustLanguage {
    fn display_id() -> &'static str {
        "language.rust"
    }
}

impl LanguageType for JavascriptLanguage {
    fn display_id() -> &'static str {
        "language.javascript"
    }
}

impl LanguageType for JavascriptXLanguage {
    fn display_id() -> &'static str {
        "language.javascript.x"
    }
    fn parent(&self) -> Option<LanguageID> {
        Some(JavascriptLanguage::id())
    }
}
