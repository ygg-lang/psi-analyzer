#![feature(once_cell)]
#![feature(box_syntax)]
#![allow(clippy::needless_return)]
#![doc = include_str ! ("../Readme.md")]

pub use crate::{
    errors::{PsiError, PsiErrorKind},
    language_aspect::{
        manager::{LanguageRegistry, LANGUAGE_REGISTRY_INSTANCE},
        LanguageID,
    },
};

pub mod errors;
pub mod file_aspect;
pub mod language_aspect;
