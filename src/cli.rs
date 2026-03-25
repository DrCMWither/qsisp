use std::env;

use crate::locale::{detect_locale_from_env, Locale};

#[derive(Debug, Clone)]
pub struct CliOptions {
    pub locale: Locale,
    pub verbose: bool,
    pub file_path: String,
}

pub fn print_usage(program: &str) {
    eprintln!("Usage:");
    eprintln!("  {program} <source-file>");
    eprintln!("  {program} --verbose <source-file>");
    eprintln!("  {program} --locale <zh-CN|en-US|ja-JP|fr-FR|de-DE|ar-SA> <source-file>");
    eprintln!("  {program} --locale <zh-CN|en-US|ja-JP|fr-FR|de-DE|ar-SA> --verbose <source-file>");
    eprintln!("  {program} --verbose --locale <zh-CN|en-US|ja-JP|fr-FR|de-DE|ar-SA> <source-file>");
}

pub fn parse_locale_arg(s: &str) -> Result<Locale, String> {
    match s {
        "zh-CN" | "zh_CN" => Ok(Locale::ZhCN),
        "en-US" | "en_US" => Ok(Locale::EnUS),
        "ja-JP" | "ja_JP" => Ok(Locale::JaJP),
        "fr-FR" | "fr_FR" => Ok(Locale::FrFR),
        "de-DE" | "de_DE" => Ok(Locale::DeDE),
        "ar-SA" | "ar_SA" => Ok(Locale::ArSA),
        _ => Err(format!("Unsupported locale: {s}")),
    }
}

pub fn parse_args() -> Result<CliOptions, String> {
    let args: Vec<String> = env::args().collect();

    match args.as_slice() {
        [_prog, file] => Ok(CliOptions {
            locale: detect_locale_from_env(),
            verbose: false,
            file_path: file.clone(),
        }),

        [_prog, flag, file] if flag == "--verbose" => Ok(CliOptions {
            locale: detect_locale_from_env(),
            verbose: true,
            file_path: file.clone(),
        }),

        [_prog, flag, loc, file] if flag == "--locale" => Ok(CliOptions {
            locale: parse_locale_arg(loc)?,
            verbose: false,
            file_path: file.clone(),
        }),

        [_prog, flag1, loc, flag2, file]
            if flag1 == "--locale" && flag2 == "--verbose" =>
        {
            Ok(CliOptions {
                locale: parse_locale_arg(loc)?,
                verbose: true,
                file_path: file.clone(),
            })
        }

        [_prog, flag1, flag2, loc, file]
            if flag1 == "--verbose" && flag2 == "--locale" =>
        {
            Ok(CliOptions {
                locale: parse_locale_arg(loc)?,
                verbose: true,
                file_path: file.clone(),
            })
        }

        _ => Err("invalid arguments".to_string()),
    }
}