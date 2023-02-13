use std::fmt::Debug;
use std::borrow::BorrowMut;
use std::cell::LazyCell;
use std::ops::DerefMut;
use std::sync::{LazyLock, Mutex, TryLockResult};
use crate::LanguageID;

pub mod manager;
pub mod id;

pub trait LanguageType {
    fn language_id() -> LanguageID;
    fn mime_type() -> &'static str;
    fn base_language() -> Option<Self>
        where
            Self: Sized,
    {
        None
    }
    fn dialects() -> Vec<Self>
        where
            Self: Sized,
    {
        Vec::new()
    }
}
