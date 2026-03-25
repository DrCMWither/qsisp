use crate::eval::{BuiltinFunc, Env, EvalError, Value};


pub fn install_stdlib(env: &Env) {
    bind_builtin(env, "print", builtin_print);
    bind_builtin(env, "+", builtin_add);
    bind_builtin(env, "-", builtin_sub);
    bind_builtin(env, "<=", builtin_le);
    bind_builtin(env, "list", builtin_list);
    bind_builtin(env, "cons", builtin_cons);
}

fn bind_builtin(env: &Env, name: &'static str, func: BuiltinFunc) {
    env.set(
        name.to_string(),
        Value::Builtin { name, func },
    );
}

fn builtin_print(args: &[Value]) -> Result<Value, EvalError> {
    if args.len() != 1 {
        return Err(EvalError::InvalidForm(
            "print expects exactly 1 argument".into(),
        ));
    }

    println!("{}", args[0]);
    Ok(args[0].clone())
}

fn builtin_add(args: &[Value]) -> Result<Value, EvalError> {
    if args.len() < 2 {
        return Err(EvalError::InvalidForm(
            "+ expects at least 2 arguments".into(),
        ));
    }

    let mut sum = 0;
    for v in args {
        match v {
            Value::Number(n) => sum += n,
            other => {
                return Err(EvalError::TypeError(format!(
                    "expected number, got {:?}",
                    other
                )));
            }
        }
    }
    Ok(Value::Number(sum))
}

fn builtin_sub(args: &[Value]) -> Result<Value, EvalError> {
    if args.is_empty() {
        return Err(EvalError::InvalidForm(
            "- expects at least 1 argument".into(),
        ));
    }

    let first = match &args[0] {
        Value::Number(n) => *n,
        other => {
            return Err(EvalError::TypeError(format!(
                "expected number, got {:?}",
                other
            )));
        }
    };

    if args.len() == 1 {
        return Ok(Value::Number(-first));
    }

    let mut acc = first;
    for v in &args[1..] {
        match v {
            Value::Number(n) => acc -= n,
            other => {
                return Err(EvalError::TypeError(format!(
                    "expected number, got {:?}",
                    other
                )));
            }
        }
    }

    Ok(Value::Number(acc))
}

fn builtin_le(args: &[Value]) -> Result<Value, EvalError> {
    if args.len() != 2 {
        return Err(EvalError::InvalidForm(
            "<= expects exactly 2 arguments".into(),
        ));
    }

    let lhs = match &args[0] {
        Value::Number(n) => *n,
        other => {
            return Err(EvalError::TypeError(format!(
                "expected number, got {:?}",
                other
            )));
        }
    };

    let rhs = match &args[1] {
        Value::Number(n) => *n,
        other => {
            return Err(EvalError::TypeError(format!(
                "expected number, got {:?}",
                other
            )));
        }
    };

    Ok(Value::Bool(lhs <= rhs))
}

fn builtin_list(args: &[Value]) -> Result<Value, EvalError> {
    Ok(Value::List(args.to_vec()))
}

fn builtin_cons(args: &[Value]) -> Result<Value, EvalError> {
    if args.len() != 2 {
        return Err(EvalError::InvalidForm(
            "cons expects exactly 2 arguments".into(),
        ));
    }

    let head = args[0].clone();
    match &args[1] {
        Value::List(tail) => {
            let mut new_list = vec![head];
            new_list.extend(tail.clone());
            Ok(Value::List(new_list))
        }
        Value::Nil => {
            Ok(Value::List(vec![head]))
        }
        other => {
            Err(EvalError::TypeError(format!(
                "cons: second argument must be a list or nil, got {:?}",
                other
            )))
        }
    }
}