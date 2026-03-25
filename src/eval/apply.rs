use crate::parser::Expr;

use super::core::eval;
use super::env::Env;
use super::error::EvalError;
use super::value::Value;

pub fn apply(func_value: Value, arg_exprs: &[Expr], call_env: &Env) -> Result<Value, EvalError> {
    match func_value {
        Value::Function(func) => {
            if arg_exprs.len() != func.params.len() {
                return Err(EvalError::ArityMismatch {
                    expected: func.params.len(),
                    got: arg_exprs.len(),
                });
            }

            let local_env = Env::child(func.env.clone());

            if let Some(name) = &func.name {
                local_env.set(name.clone(), Value::Function(func.clone()));
            }

            for (param, arg_expr) in func.params.iter().zip(arg_exprs.iter()) {
                let arg_value = eval(arg_expr, call_env)?;
                local_env.set(param.clone(), arg_value);
            }

            eval(&func.body, &local_env)
        }

        Value::Builtin { func, .. } => {
            let mut args = Vec::new();
            for arg_expr in arg_exprs {
                args.push(eval(arg_expr, call_env)?);
            }
            func(&args)
        }

        other => Err(EvalError::TypeError(format!(
            "attempted to call a non-function value: {:?}",
            other
        ))),
    }
}