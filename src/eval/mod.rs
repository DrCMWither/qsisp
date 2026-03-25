pub mod apply;
pub mod core;
pub mod env;
pub mod error;
pub mod special_forms;
pub mod value;

pub use core::{eval, eval_program};
pub use env::Env;
pub use error::EvalError;
pub use value::{BuiltinFunc, Function, Value};