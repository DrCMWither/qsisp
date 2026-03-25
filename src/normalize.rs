use crate::locale_pack::LocalePack;
use crate::parser::Expr;

pub fn normalize_program(exprs: &[Expr], pack: &LocalePack) -> Vec<Expr> {
    exprs.iter().map(|e| normalize_expr(e, pack)).collect()
}

fn normalize_expr(expr: &Expr, pack: &LocalePack) -> Expr {
    match expr {
        Expr::Number(n) => Expr::Number(*n),
        Expr::String(s) => Expr::String(s.clone()),
        Expr::Symbol(s) => Expr::Symbol(s.clone()),
        Expr::List(items) => normalize_list(items, pack),
    }
}

fn normalize_list(items: &[Expr], pack: &LocalePack) -> Expr {
    if items.is_empty() {
        return Expr::List(vec![]);
    }

    let head = match &items[0] {
        Expr::Symbol(s) => {
            let canonical = pack.canonicalize_special_form(s).unwrap_or(s);
            Expr::Symbol(canonical.to_string())
        }
        other => normalize_expr(other, pack),
    };

    let head_name = match &head {
        Expr::Symbol(s) => Some(s.clone()),
        _ => None,
    };

    let mut out = vec![head];

    match head_name.as_deref() {
        Some("define") => {
            if items.len() >= 2 {
                match &items[1] {
                    Expr::Symbol(name) => out.push(Expr::Symbol(name.clone())),
                    Expr::List(sig) => {
                        let mut nsig = Vec::new();
                        if let Some(first) = sig.first() {
                            match first {
                                Expr::Symbol(name) => nsig.push(Expr::Symbol(name.clone())),
                                other => nsig.push(normalize_expr(other, pack)),
                            }
                        }
                        for p in sig.iter().skip(1) {
                            match p {
                                Expr::Symbol(name) => nsig.push(Expr::Symbol(name.clone())),
                                other => nsig.push(normalize_expr(other, pack)),
                            }
                        }
                        out.push(Expr::List(nsig));
                    }
                    other => out.push(normalize_expr(other, pack)),
                }
            }
            for arg in items.iter().skip(2) {
                out.push(normalize_expr(arg, pack));
            }
        }

        Some("lambda") => {
            if items.len() >= 2 {
                match &items[1] {
                    Expr::List(params) => {
                        let plist = params
                            .iter()
                            .map(|p| match p {
                                Expr::Symbol(name) => Expr::Symbol(name.clone()),
                                other => normalize_expr(other, pack),
                            })
                            .collect();
                        out.push(Expr::List(plist));
                    }
                    other => out.push(normalize_expr(other, pack)),
                }
            }
            for arg in items.iter().skip(2) {
                out.push(normalize_expr(arg, pack));
            }
        }

        Some("let") => {
            if items.len() >= 2 {
                match &items[1] {
                    Expr::List(bindings) => {
                        let nbindings = bindings
                            .iter()
                            .map(|b| match b {
                                Expr::List(pair) if pair.len() == 2 => {
                                    let name = match &pair[0] {
                                        Expr::Symbol(name) => Expr::Symbol(name.clone()),
                                        other => normalize_expr(other, pack),
                                    };
                                    let value = normalize_expr(&pair[1], pack);
                                    Expr::List(vec![name, value])
                                }
                                other => normalize_expr(other, pack),
                            })
                            .collect();
                        out.push(Expr::List(nbindings));
                    }
                    other => out.push(normalize_expr(other, pack)),
                }
            }
            for arg in items.iter().skip(2) {
                out.push(normalize_expr(arg, pack));
            }
        }

        _ => {
            for arg in items.iter().skip(1) {
                out.push(normalize_expr(arg, pack));
            }
        }
    }

    Expr::List(out)
}