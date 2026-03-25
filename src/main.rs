mod cli;
mod locale;
mod lexer;
mod parser;
mod normalize;
mod eval;
mod stdlib;
mod symbols;
mod debugger;
mod run;
mod locale_pack;

use std::env;
use std::process;

use cli::{parse_args, print_usage};
use run::run;

fn main() {
    let program = env::args()
        .next()
        .unwrap_or_else(|| "qsisp".to_string());

    let opts = match parse_args() {
        Ok(opts) => opts,
        Err(msg) => {
            if msg != "invalid arguments" {
                eprintln!("{msg}");
            }
            print_usage(&program);
            process::exit(2);
        }
    };

    if let Err(msg) = run(&opts) {
        eprintln!("{msg}");
        process::exit(1);
    }
}