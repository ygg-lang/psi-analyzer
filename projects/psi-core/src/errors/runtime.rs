use std::sync::{MutexGuard, PoisonError, TryLockError};

use super::

*;

impl<T> From<PoisonError<MutexGuard<'_, T>>> for PsiError {
    fn from(error: PoisonError<MutexGuard<'_, T>>) -> Self {
        Self::runtime_error(format!("Language registry is poisoned: {}", error))
    }
}

impl<T> From<TryLockError<MutexGuard<'_, T>>> for PsiError {
    fn from(error: TryLockError<MutexGuard<'_, T>>) -> Self {
        Self::runtime_error(format!("Language registry is locked: {}", error))
    }
}