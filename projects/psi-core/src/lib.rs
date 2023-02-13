#![feature(once_cell)]
#![feature(box_syntax)]
#![allow(clippy::needless_return)]
#![doc = include_str ! ("../Readme.md")]

pub use crate::{errors::{PsiError, PsiErrorKind}};
pub use crate::language_aspect::{LanguageID, manager::LANGUAGE_REGISTRY_INSTANCE, manager::LanguageRegistry};

pub mod language_aspect;
pub mod errors;
// pub mod file_aspect;