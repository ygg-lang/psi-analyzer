mod runtime;

pub type PsiResult<T=()> = Result<T, PsiError>;


pub struct PsiError {
    kind: PsiErrorKind,
}

pub enum PsiErrorKind {
    RuntimeError(Box<RuntimeError>),
}

pub struct RuntimeError {
    message: String,
}

impl PsiError {
    pub fn runtime_error(message: String) -> Self {
        Self {
            kind: PsiErrorKind::RuntimeError(box RuntimeError { message }),
        }
    }
}