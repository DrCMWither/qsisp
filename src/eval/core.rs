use crate::parser::Expr;
use crate::symbols;

use super::apply::apply;
use super::env::Env;
use super::error::EvalError;
use super::special_forms::{
    eval_begin, eval_define, eval_if, eval_lambda, eval_let, eval_set, eval_quote, eval_defmacro, eval_import,
};
use super::value::Value;

pub fn eval_program(exprs: &[Expr], env: &Env) -> Result<Vec<Value>, EvalError> {
    let mut out = Vec::new();
    for expr in exprs {
        out.push(eval(expr, env)?);
    }
    Ok(out)
}

pub fn eval(expr: &Expr, env: &Env) -> Result<Value, EvalError> {
    match expr {
        Expr::Number(n) => Ok(Value::Number(*n)),
        Expr::String(s) => Ok(Value::String(s.clone())),
        Expr::Symbol(name) => env
            .get(name)
            .ok_or_else(|| EvalError::UndefinedSymbol(name.clone())),
        Expr::List(items) => eval_list(items, env),
    }
}

fn eval_list(items: &[Expr], env: &Env) -> Result<Value, EvalError> {
    if items.is_empty() {
        return Err(EvalError::InvalidForm(
            "empty list cannot be evaluated".into(),
        ));
    }

    match &items[0] {
        Expr::Symbol(name) if name == symbols::DEFINE => return eval_define(items, env),
        Expr::Symbol(name) if name == symbols::IF => return eval_if(items, env),
        Expr::Symbol(name) if name == symbols::LET => return eval_let(items, env),
        Expr::Symbol(name) if name == symbols::LAMBDA => return eval_lambda(items, env),
        Expr::Symbol(name) if name == symbols::BEGIN => return eval_begin(items, env),
        Expr::Symbol(name) if name == symbols::SET => return eval_set(items, env),
        Expr::Symbol(name) if name == symbols::QUOTE => return eval_quote(items, env),
        Expr::Symbol(name) if name == symbols::DEFMACRO => return eval_defmacro(items, env),
        Expr::Symbol(name) if name == symbols::IMPORT => return eval_import(items, env),
        _ => {}
    }

    let callee = eval(&items[0], env)?;
    match callee {
        Value::Macro(mac) => {
            let local_env = Env::child(mac.env.clone());
            if items.len() - 1 != mac.params.len() {
                return Err(EvalError::ArityMismatch { expected: mac.params.len(), got: items.len() - 1 });
            }
            for (param, arg_expr) in mac.params.iter().zip(items[1..].iter()) {
                local_env.set(param.clone(), expr_to_value(arg_expr));
            }
            let expanded_value = eval(&mac.body, &local_env)?;
            let expanded_expr = value_to_expr(&expanded_value)?;
            eval(&expanded_expr, env)
        }
        _ => apply(callee, &items[1..], env)
    }
}

pub fn expr_to_value(expr: &Expr) -> Value {
    match expr {
        Expr::Number(n) => Value::Number(*n),
        Expr::String(s) => Value::String(s.clone()),
        Expr::Symbol(s) => Value::Symbol(s.clone()),
        Expr::List(items) => Value::List(items.iter().map(expr_to_value).collect()),
    }
}

pub fn value_to_expr(val: &Value) -> Result<Expr, EvalError> {
    match val {
        Value::Number(n) => Ok(Expr::Number(*n)),
        Value::String(s) => Ok(Expr::String(s.clone())),
        Value::Symbol(s) => Ok(Expr::Symbol(s.clone())),
        Value::List(items) => {
            let mut exprs = Vec::new();
            for item in items {
                exprs.push(value_to_expr(item)?);
            }
            Ok(Expr::List(exprs))
        }
        _ => Err(EvalError::TypeError("Unable to transform closure into AST node".into())),
    }
}

pub(crate) fn is_truthy(value: &Value) -> bool {
    match value {
        Value::Bool(b) => *b,
        Value::Nil => false,
        Value::Number(n) => *n != 0,
        Value::String(s) => !s.is_empty(),
        Value::Function(_)
        | Value::Builtin { .. }
        | Value::Symbol(_)
        | Value::List(_)
        | Value::Macro(_) => true,
    }
}