#![feature(once_cell)]
#![allow(clippy::needless_return)]
#![doc = include_str ! ("../Readme.md")]

pub use crate::language_aspect::{manager::LANGUAGE_REGISTRY_INSTANCE, manager::LanguageRegistry, id::LanguageID};

pub mod language_aspect;
// pub mod file_aspect;