use psi_core::{LanguageID, LanguageRegistry, LANGUAGE_REGISTRY_INSTANCE};

#[test]
fn ready() {
    println!("it, works!")
}

#[test]
fn rust_language() {
    LanguageRegistry::register_language("Rust", LanguageID::any()).ok();
    println!("{:?}", LANGUAGE_REGISTRY_INSTANCE.lock().unwrap());
}
