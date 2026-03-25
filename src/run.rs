use std::fs;
use std::path::PathBuf;

use crate::cli::CliOptions;
use crate::debugger::{debug_block, debug_header};
use crate::eval::{eval_program, Env, Value};
use crate::keywords::KeywordMap;
use crate::lexer::lex;
use crate::normalize::normalize_program;
use crate::parser::parse;
use crate::stdlib;

pub fn run(opts: &CliOptions) -> Result<(), String> {
    let source = fs::read_to_string(&opts.file_path)
        .map_err(|err| format!("Error @ reading file: {}\n{}", opts.file_path, err))?;

    debug_header(opts.verbose, opts.locale, &opts.file_path);

    let tokens = lex(&source, opts.locale)
        .map_err(|err| format!("Lexical error @ {:?}", err))?;
    debug_block(opts.verbose, "tokens", &tokens);

    let ast = parse(&tokens)
        .map_err(|err| format!("Syntax error @ {:?}", err))?;
    debug_block(opts.verbose, "ast", &ast);

    let kw = KeywordMap::new(opts.locale);
    let normalized_ast = normalize_program(&ast, &kw);
    debug_block(opts.verbose, "normalized ast", &normalized_ast);

    let env = Env::new(opts.locale);
    stdlib::install_stdlib(&env);

    let absolute_path = fs::canonicalize(&opts.file_path)
        .unwrap_or_else(|_| PathBuf::from(&opts.file_path));
    env.set(
        "__FILE__".to_string(),
        Value::String(absolute_path.to_string_lossy().into_owned()),
    );

    let values = eval_program(&normalized_ast, &env)
        .map_err(|err| format!("Eval error @ {:?}", err))?;
    debug_block(opts.verbose, "vals", &values);

    Ok(())
}