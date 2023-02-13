use std::{
    cmp::Ordering,
    collections::BTreeMap,
    fmt::{Debug, Formatter},
    hash::{Hash, Hasher},
};

use std::sync::{LazyLock, Mutex};

use crate::{errors::PsiResult, PsiError};

pub mod id;
pub mod manager;
pub mod typing;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LanguageID(u64);

#[derive(Clone, Debug)]
pub struct LanguageType {
    id: LanguageID,
    name: String,
    parent: LanguageID,
    mime_type: &'static str,
}
