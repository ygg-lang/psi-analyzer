use std::{
    cmp::Ordering,
    collections::hash_map::DefaultHasher,
    fmt::{Debug, Formatter},
    hash::{Hash, Hasher},
    sync::LazyLock,
};

use crate::{errors::PsiResult, PsiError, LANGUAGE_REGISTRY_INSTANCE};

pub mod id;
pub mod manager;
pub mod typing;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LanguageID(u64);

pub trait LanguageType {
    fn name(&self) -> &str;
    fn parent(&self) -> LanguageID {
        LanguageID::any()
    }
}

#[derive(Clone, Debug)]
pub struct LanguageInstance {
    id: LanguageID,
    name: String,
    parent: LanguageID,
    mime_type: &'static str,
}
