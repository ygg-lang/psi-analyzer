use std::{
    cmp::Ordering,
    collections::hash_map::DefaultHasher,
    fmt::{Debug, Formatter},
    hash::{Hash, Hasher},
    intrinsics::type_id,
    sync::LazyLock,
};

use mime::Mime;

use crate::{errors::PsiResult, PsiError, LANGUAGE_REGISTRY_INSTANCE};

pub mod id;
pub mod manager;
pub mod typing;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LanguageID(u64);

pub trait LanguageType {
    fn display_name(&self) -> &str {
        self.name()
    }

    fn parent(&self) -> LanguageID {
        LanguageID::any()
    }
    fn file_pattern(&self) -> Vec<String> {
        vec![]
    }
    fn extensions(&self) -> Vec<String> {
        vec![]
    }
}

#[derive(Clone, Debug)]
pub struct LanguageInstance {
    id: LanguageID,
    display_name: String,
    parent: LanguageID,
    file_name: Vec<String>,
    file_extension: Vec<String>,
    mime: Vec<Mime>,
}

impl LanguageInstance {
    pub fn instantiate<T: LanguageType>(language: T) -> Self {
        let id = LanguageID(type_id::<T>());
        let display_name = language.display_name().to_string();
        let parent = language.parent();
        let file_name = language.file_pattern();
        let file_extension = language.extensions();
        let mime = vec![];
        Self { id, display_name, parent, file_name, file_extension, mime }
    }
}

pub trait PluginType {
    fn file_extension() -> Vec<String> {}
    fn file_name() -> Vec<String> {}
}
