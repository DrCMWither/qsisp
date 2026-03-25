#[derive(Debug, Clone, PartialEq)]
pub enum EvalError {
    UndefinedSymbol(String),
    InvalidForm(String),
    TypeError(String),
    ArityMismatch { expected: usize, got: usize },
}