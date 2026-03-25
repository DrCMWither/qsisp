mod locale;
mod lexer;

use std::{env, fs, process};

use lexer::lex;
use locale::{detect_locale_from_env, Locale};

fn print_usage(program: &str) {
    eprintln!("Usage:");
    eprintln!("  {program} <source-file>");
    eprintln!("  {program} --locale <zh-CN|en-US|ja-JP|fr-FR|de-DE> <source-file>");
}

fn parse_locale_arg(s: &str) -> Result<Locale, String> {
    match s {
        "zh-CN" | "zh_CN" => Ok(Locale::ZhCN),
        "en-US" | "en_US" => Ok(Locale::EnUS),
        "ja-JP" | "ja_JP" => Ok(Locale::JaJP),
        "fr-FR" | "fr_FR" => Ok(Locale::FrFR),
        "de-DE" | "de_DE" => Ok(Locale::DeDE),
        _ => Err(format!("Unsupported locale: {s}")),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args.first().map(String::as_str).unwrap_or("qsisp");

    let (locale, file_path) = match args.as_slice() {
        [_prog, file] => (detect_locale_from_env(), file.as_str()),
        [_prog, flag, loc, file] if flag == "--locale" => {
            let locale = match parse_locale_arg(loc) {
                Ok(l) => l,
                Err(msg) => {
                    eprintln!("{msg}");
                    print_usage(program);
                    process::exit(2);
                }
            };
            (locale, file.as_str())
        }
        _ => {
            print_usage(program);
            process::exit(2);
        }
    };

    let source = match fs::read_to_string(file_path) {
        Ok(s) => s,
        Err(err) => {
            eprintln!("Error @ reading file: {file_path}");
            eprintln!("{err}");
            process::exit(1);
        }
    };

    println!("== qsisp ==");
    println!("locale: {:?}", locale);
    println!("file:   {}", file_path);

    let tokens = match lex(&source, locale) {
        Ok(t) => t,
        Err(err) => {
            eprintln!("Syntax error @ {:?}", err);
            process::exit(1);
        }
    };

    println!("== tokens ==");
    for (idx, token) in tokens.iter().enumerate() {
        println!("[{idx:04}] {:?}", token);
    }
}