use super::*;

pub static LANGUAGE_REGISTRY_INSTANCE: LazyLock<Mutex<LanguageRegistry>> = LazyLock::new(|| {
    Mutex::new(LanguageRegistry {
        languages: vec![],
    })
});

pub struct LanguageRegistry {
    languages: Vec<LanguageID>,
}

impl LanguageRegistry {

}