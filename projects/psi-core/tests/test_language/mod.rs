use psi_core::{LanguageID, LanguageRegistry, LanguageType};

#[test]
fn rust_language() {
    let langs = LanguageRegistry::default();

    langs.register_language(RustLanguage).ok();
    langs.register_language(JavascriptLanguage).ok();
    langs.register_language(JavascriptXLanguage).ok();
    println!("{:?}", langs);
}

pub struct RustLanguage;

pub struct JavascriptLanguage;

pub struct JavascriptXLanguage;

impl LanguageType for RustLanguage {
    fn display_id(&self) -> &'static str {
        "language.rust"
    }
}

impl LanguageType for JavascriptLanguage {
    fn display_id(&self) -> &'static str {
        "language.javascript"
    }
}

impl LanguageType for JavascriptXLanguage {
    fn display_id(&self) -> &'static str {
        "language.javascript.x"
    }
    fn parent(&self) -> Option<LanguageID> {
        Some(JavascriptLanguage::id())
    }
}
