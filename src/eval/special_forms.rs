use std::fs;
use std::path::Path;

use crate::lexer::lex;
use crate::locale_pack::LocalePack;
use crate::normalize::normalize_program;
use crate::parser::Expr;
use crate::parser::parse;

use super::core::{eval, expr_to_value, is_truthy};
use super::env::Env;
use super::error::EvalError;
use super::value::{Function, Value};

pub fn eval_define(items: &[Expr], env: &Env, pack: &LocalePack) -> Result<Value, EvalError> {
    if items.len() != 3 {
        return Err(EvalError::InvalidForm(
            "define expects exactly 2 arguments".into(),
        ));
    }

    match &items[1] {
        Expr::Symbol(name) => {
            let value = eval(&items[2], env, pack)?;
            env.set(name.clone(), value.clone());
            Ok(value)
        }

        Expr::List(sig) => {
            if sig.is_empty() {
                return Err(EvalError::InvalidForm(
                    "define: function signature cannot be empty".into(),
                ));
            }

            let fname = match &sig[0] {
                Expr::Symbol(s) => s.clone(),
                _ => {
                    return Err(EvalError::InvalidForm(
                        "define: function name must be a symbol".into(),
                    ));
                }
            };

            let mut params = Vec::new();
            for param in &sig[1..] {
                match param {
                    Expr::Symbol(s) => params.push(s.clone()),
                    _ => {
                        return Err(EvalError::InvalidForm(
                            "define: function parameters must be symbols".into(),
                        ));
                    }
                }
            }

            let func = Value::Function(Function {
                name: Some(fname.clone()),
                params,
                body: items[2].clone(),
                env: env.clone(),
            });

            env.set(fname, func.clone());
            Ok(func)
        }

        _ => Err(EvalError::InvalidForm(
            "define: first argument must be a symbol or a parameter list".into(),
        )),
    }
}

pub fn eval_if(items: &[Expr], env: &Env, pack: &LocalePack) -> Result<Value, EvalError> {
    if items.len() != 4 {
        return Err(EvalError::InvalidForm(
            "if expects exactly 3 arguments".into(),
        ));
    }

    let cond = eval(&items[1], env, pack)?;
    if is_truthy(&cond) {
        eval(&items[2], env, pack)
    } else {
        eval(&items[3], env, pack)
    }
}

pub fn eval_let(items: &[Expr], env: &Env, pack: &LocalePack) -> Result<Value, EvalError> {
    if items.len() != 3 {
        return Err(EvalError::InvalidForm(
            "let expects exactly 2 arguments".into(),
        ));
    }

    let bindings = match &items[1] {
        Expr::List(xs) => xs,
        _ => {
            return Err(EvalError::InvalidForm(
                "let: first argument must be a binding list".into(),
            ));
        }
    };

    let local_env = Env::child(env.clone());

    for binding in bindings {
        let pair = match binding {
            Expr::List(pair) if pair.len() == 2 => pair,
            _ => {
                return Err(EvalError::InvalidForm(
                    "let: each binding must be a list of length 2".into(),
                ));
            }
        };

        let name = match &pair[0] {
            Expr::Symbol(s) => s.clone(),
            _ => {
                return Err(EvalError::InvalidForm(
                    "let: binding name must be a symbol".into(),
                ));
            }
        };

        let value = eval(&pair[1], env, pack)?;
        local_env.set(name, value);
    }

    eval(&items[2], &local_env, pack)
}

pub fn eval_lambda(items: &[Expr], env: &Env, _pack: &LocalePack) -> Result<Value, EvalError> {
    if items.len() != 3 {
        return Err(EvalError::InvalidForm(
            "lambda expects exactly 2 arguments".into(),
        ));
    }

    let params_expr = match &items[1] {
        Expr::List(xs) => xs,
        _ => {
            return Err(EvalError::InvalidForm(
                "lambda: parameter list must be a list".into(),
            ));
        }
    };

    let mut params = Vec::new();
    for p in params_expr {
        match p {
            Expr::Symbol(s) => params.push(s.clone()),
            _ => {
                return Err(EvalError::InvalidForm(
                    "lambda: parameters must be symbols".into(),
                ));
            }
        }
    }

    Ok(Value::Function(Function {
        name: None,
        params,
        body: items[2].clone(),
        env: env.clone(),
    }))
}

pub fn eval_begin(items: &[Expr], env: &Env, pack: &LocalePack) -> Result<Value, EvalError> {
    if items.len() == 1 {
        return Ok(Value::Nil);
    }

    let mut last = Value::Nil;
    for expr in &items[1..] {
        last = eval(expr, env, pack)?;
    }
    Ok(last)
}

pub fn eval_set(items: &[Expr], env: &Env, pack: &LocalePack) -> Result<Value, EvalError> {
    if items.len() != 3 {
        return Err(EvalError::InvalidForm(
            "set! expects exactly 2 arguments".into(),
        ));
    }

    let name = match &items[1] {
        Expr::Symbol(s) => s.clone(),
        _ => {
            return Err(EvalError::InvalidForm(
                "set!: first argument must be a symbol".into(),
            ));
        }
    };

    let value = eval(&items[2], env, pack)?;
    env.assign(&name, value.clone())?;
    Ok(value)
}

pub fn eval_quote(items: &[Expr], _env: &Env, _pack: &LocalePack) -> Result<Value, EvalError> {
    if items.len() != 2 {
        return Err(EvalError::InvalidForm(
            "quote expects exactly 1 argument".into(),
        ));
    }
    Ok(expr_to_value(&items[1]))
}

pub fn eval_defmacro(items: &[Expr], env: &Env, _pack: &LocalePack) -> Result<Value, EvalError> {
    if items.len() != 3 {
        return Err(EvalError::InvalidForm(
            "defmacro expects exactly 2 arguments: (defmacro (name params...) body)".into(),
        ));
    }

    match &items[1] {
        Expr::List(sig) => {
            if sig.is_empty() {
                return Err(EvalError::InvalidForm(
                    "defmacro: macro signature cannot be empty".into(),
                ));
            }

            let fname = match &sig[0] {
                Expr::Symbol(s) => s.clone(),
                _ => {
                    return Err(EvalError::InvalidForm(
                        "defmacro: macro name must be a symbol".into(),
                    ));
                }
            };

            let mut params = Vec::new();
            for param in &sig[1..] {
                match param {
                    Expr::Symbol(s) => params.push(s.clone()),
                    _ => {
                        return Err(EvalError::InvalidForm(
                            "defmacro: macro parameters must be symbols".into(),
                        ));
                    }
                }
            }

            let mac = Value::Macro(Function {
                name: Some(fname.clone()),
                params,
                body: items[2].clone(),
                env: env.clone(),
            });

            env.set(fname, mac.clone());
            Ok(mac)
        }
        _ => Err(EvalError::InvalidForm(
            "defmacro: first argument must be a signature list, like (name params...)".into(),
        )),
    }
}

pub fn eval_import(items: &[Expr], env: &Env, pack: &LocalePack) -> Result<Value, EvalError> {
    if items.len() != 2 {
        return Err(EvalError::InvalidForm(
            "import expects exactly 1 string argument".into(),
        ));
    }

    let rel_filename = match &items[1] {
        Expr::String(s) => s,
        _ => return Err(EvalError::TypeError("import requires a string path".into())),
    };

    let base_dir = env
        .get("__FILE__")
        .and_then(|v| {
            if let Value::String(path_str) = v {
                Path::new(&path_str).parent().map(|p| p.to_path_buf())
            } else {
                None
            }
        })
        .unwrap_or_else(|| std::env::current_dir().unwrap());

    let target_path = base_dir.join(rel_filename);
    let canonical_path = fs::canonicalize(&target_path).unwrap_or(target_path);

    let src = fs::read_to_string(&canonical_path).map_err(|e| {
        EvalError::InvalidForm(format!("Cannot read imported file {:?}: {}", canonical_path, e))
    })?;

    let tokens = lex(&src, env.locale).map_err(|e| {
        EvalError::InvalidForm(format!("Lexical error in {:?}: {:?}", canonical_path, e))
    })?;

    let ast = parse(&tokens).map_err(|e| {
        EvalError::InvalidForm(format!("Syntax error in {:?}: {:?}", canonical_path, e))
    })?;

    let normalized_ast = normalize_program(&ast, pack);

    let old_file = env.get("__FILE__");
    env.set(
        "__FILE__".to_string(),
        Value::String(canonical_path.to_string_lossy().into_owned()),
    );

    let mut last_val = Value::Nil;
    for expr in &normalized_ast {
        last_val = super::core::eval(expr, env, pack)?;
    }

    if let Some(old) = old_file {
        env.set("__FILE__".to_string(), old);
    }

    Ok(last_val)
}