use dashmap::{mapref::multiple::RefMulti, DashMap};
use std::{
    any::type_name,
    cmp::Ordering,
    collections::hash_map::DefaultHasher,
    fmt::{Debug, Formatter},
    hash::{Hash, Hasher},
};

use mime::Mime;

use crate::{errors::PsiResult, PsiError};

pub mod id;
pub mod instance;
pub mod manager;
pub mod resolver;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LanguageID(u64);

pub trait LanguageType
where
    Self: 'static,
{
    // hash of the debug_name
    fn id() -> LanguageID {
        let mut hasher = DefaultHasher::new();
        Self::debug_name().hash(&mut hasher);
        LanguageID(hasher.finish())
    }
    fn debug_name() -> &'static str {
        type_name::<Self>()
    }
    fn display_id(&self) -> &'static str {
        ""
    }
    fn parent(&self) -> Option<LanguageID> {
        None
    }
    fn case_insensitive(&self) -> bool {
        false
    }
    fn file_names(&self) -> Vec<String> {
        vec![]
    }
    fn file_extensions(&self) -> Vec<String> {
        vec![]
    }
    fn file_mime(&self) -> Vec<Mime> {
        vec![]
    }
}

#[derive(Clone, Debug)]
pub struct LanguageInstance {
    pub id: LanguageID,
    pub debug_name: &'static str,
    pub display_name: String,
    pub parent: Option<LanguageID>,
    pub case_insensitive: bool,
    pub file_names: Vec<String>,
    pub file_extensions: Vec<String>,
    pub file_mimes: Vec<Mime>,
}
