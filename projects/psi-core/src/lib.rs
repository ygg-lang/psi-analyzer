#![feature(once_cell)]
#![feature(box_syntax)]
#![feature(core_intrinsics)]
#![allow(clippy::needless_return)]
#![doc = include_str ! ("../Readme.md")]

pub use crate::{
    errors::{PsiError, PsiErrorKind},
    // file_aspect::{
    //     manager::{FileRegistry, File_REGISTRY_INSTANCE},
    //     FileID, FileInstance, FileType,
    // },
    language_aspect::{manager::LanguageRegistry, LanguageID, LanguageInstance, LanguageType},
};

pub mod errors;
pub mod records;
// pub mod file_aspect;
pub mod language_aspect;
