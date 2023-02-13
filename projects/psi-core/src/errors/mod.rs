use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
    sync::{MutexGuard, PoisonError, TryLockError},
};

mod runtime;

pub type PsiResult<T = ()> = Result<T, PsiError>;

pub struct PsiError {
    kind: PsiErrorKind,
}

#[derive(Debug)]
pub enum PsiErrorKind {
    RuntimeError(Box<RuntimeError>),
}

#[derive(Debug)]
pub struct RuntimeError {
    message: String,
}

impl Debug for PsiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            PsiErrorKind::RuntimeError(e) => Debug::fmt(e, f),
        }
    }
}

impl Display for PsiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for PsiError {}

impl PsiError {
    pub fn runtime_error<T>(message: T) -> Self
    where
        T: Into<String>,
    {
        Self { kind: PsiErrorKind::RuntimeError(box RuntimeError { message: message.into() }) }
    }
}
