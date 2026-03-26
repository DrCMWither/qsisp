use crate::parser::Expr;

use super::env::Env;
use super::error::EvalError;

#[derive(Clone, PartialEq)]
pub struct Function {
    pub name: Option<String>,
    pub params: Vec<String>,
    pub body: Expr,
    pub env: Env,
}

pub type BuiltinFunc = fn(&[Value]) -> Result<Value, EvalError>;

#[derive(Clone, PartialEq)]
pub enum Value {
    Number(i64),
    String(String),
    Bool(bool),
    Function(Function),
    Symbol(String),
    List(Vec<Value>),
    Macro(Function),
    Builtin {
        name: &'static str,
        func: BuiltinFunc,
    },
    Nil,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n)           => write!(f, "{n}"),
            Value::String(s)           => write!(f, "{s}"),
            Value::Bool(b)             => write!(f, "{b}"),
            Value::Symbol(s)           => write!(f, "{s}"),
            Value::List(items)         => {
                let items_str: Vec<String> = items.iter().map(|v| v.to_string()).collect();
                write!(f, "({})", items_str.join(" "))
            }
            Value::Function(func)      => {
                if let Some(name) = &func.name {
                    write!(f, "<function:{name}>")
                } else {
                    write!(f, "<function>")
                }
            }
            Value::Macro(func)         => {
                if let Some(name) = &func.name {
                    write!(f, "<macro:{name}>")
                } else {
                    write!(f, "<macro>")
                }
            }
            Value::Builtin { name, .. } => write!(f, "<builtin:{name}>"),
            Value::Nil                  => write!(f, "nil"),
        }
    }
}

impl std::fmt::Debug for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Function")
            .field("name", &self.name)
            .field("params", &self.params)
            .field("body", &self.body)
            .field("env", &"<captured-env>")
            .finish()
    }
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n)            => f.debug_tuple("Number").field(n).finish(),
            Value::String(s)            => f.debug_tuple("String").field(s).finish(),
            Value::Bool(b)              => f.debug_tuple("Bool").field(b).finish(),
            Value::Symbol(s)            => f.debug_tuple("Symbol").field(s).finish(),
            Value::List(items)          => f.debug_tuple("List").field(items).finish(),
            Value::Function(func)       => f.debug_tuple("Function").field(func).finish(),
            Value::Macro(func)          => f.debug_tuple("Macro").field(func).finish(),
            Value::Builtin { name, .. } => f.debug_tuple("Builtin").field(name).finish(),
            Value::Nil                  => write!(f, "Nil"),
        }
    }
}