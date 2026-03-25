use std::collections::HashMap;
use crate::symbols;

use crate::locale::Locale;

#[derive(Debug, Clone)]
pub struct KeywordMap {
    aliases: HashMap<String, String>,
}

impl KeywordMap {
    pub fn new(locale: Locale) -> Self {
        let mut aliases = HashMap::new();

        for kw in [
            "define", "print", "if", "let", "lambda",
            "begin", "set!", "+", "-", "<=",
        ] {
            aliases.insert(kw.to_string(), kw.to_string());
        }

        match locale {
            Locale::ZhCN => {
                aliases.insert("定".into(), symbols::DEFINE.into());
                aliases.insert("印".into(), symbols::PRINT.into());
                aliases.insert("如".into(), symbols::IF.into());
                aliases.insert("令".into(), symbols::LET.into());
                aliases.insert("函".into(), symbols::LAMBDA.into());
                aliases.insert("始".into(), symbols::BEGIN.into());
                aliases.insert("设！".into(), symbols::SET.into());
                aliases.insert("表".into(), symbols::LIST.into());
                aliases.insert("宏".into(), symbols::DEFMACRO.into());
                aliases.insert("构".into(), symbols::CONS.into());
                aliases.insert("引".into(), symbols::QUOTE.into());
                aliases.insert("导".into(), symbols::IMPORT.into());
            }
            Locale::FrFR => {
                aliases.insert("définir".into(), symbols::DEFINE.into());
                aliases.insert("imprimer".into(), symbols::PRINT.into());
                aliases.insert("si".into(), symbols::IF.into());
                aliases.insert("laisser".into(), symbols::LET.into());
                aliases.insert("lambda".into(), symbols::LAMBDA.into());
                aliases.insert("début".into(), symbols::BEGIN.into());
                aliases.insert("assigner!".into(), symbols::SET.into());
                aliases.insert("liste".into(), symbols::LIST.into());
                aliases.insert("macro".into(), symbols::DEFMACRO.into());
                aliases.insert("construire".into(), symbols::CONS.into());
                aliases.insert("citer".into(), symbols::QUOTE.into());
                aliases.insert("importer".into(), symbols::IMPORT.into());
            }

            Locale::JaJP => {
                aliases.insert("定義".into(), symbols::DEFINE.into());
                aliases.insert("表示".into(), symbols::PRINT.into());
                aliases.insert("もし".into(), symbols::IF.into());
                aliases.insert("束縛".into(), symbols::LET.into());
                aliases.insert("関数".into(), symbols::LAMBDA.into());
                aliases.insert("開始".into(), symbols::BEGIN.into());
                aliases.insert("設定!".into(), symbols::SET.into());
                aliases.insert("リスト".into(), symbols::LIST.into());
                aliases.insert("マクロ".into(), symbols::DEFMACRO.into());
                aliases.insert("連結".into(), symbols::CONS.into());
                aliases.insert("引用".into(), symbols::QUOTE.into());
                aliases.insert("導入".into(), symbols::IMPORT.into());
            }

            Locale::DeDE => {
                aliases.insert("definieren".into(), symbols::DEFINE.into());
                aliases.insert("drucken".into(), symbols::PRINT.into());
                aliases.insert("wenn".into(), symbols::IF.into());
                aliases.insert("lassen".into(), symbols::LET.into());
                aliases.insert("lambda".into(), symbols::LAMBDA.into());
                aliases.insert("beginn".into(), symbols::BEGIN.into());
                aliases.insert("setzen!".into(), symbols::SET.into());
                aliases.insert("liste".into(), symbols::LIST.into());
                aliases.insert("makro".into(), symbols::DEFMACRO.into());
                aliases.insert("verbinden".into(), symbols::CONS.into());
                aliases.insert("zitieren".into(), symbols::QUOTE.into());
                aliases.insert("importieren".into(), symbols::IMPORT.into());
            }

            Locale::ArSA => {
                aliases.insert("عرّف".into(), symbols::DEFINE.into());
                aliases.insert("اطبع".into(), symbols::PRINT.into());
                aliases.insert("إذا".into(), symbols::IF.into());
                aliases.insert("دع".into(), symbols::LET.into());
                aliases.insert("لامدا".into(), symbols::LAMBDA.into());
                aliases.insert("ابدأ".into(), symbols::BEGIN.into());
                aliases.insert("!عيّن".into(), symbols::SET.into());
                aliases.insert("قائمة".into(), symbols::LIST.into());
                aliases.insert("ماكرو".into(), symbols::DEFMACRO.into());
                aliases.insert("ربط".into(), symbols::CONS.into());
                aliases.insert("اقتبس".into(), symbols::QUOTE.into());
                aliases.insert("استورد".into(), symbols::IMPORT.into());
            }
            Locale::EnUS => {}
        }

        Self { aliases }
    }

    pub fn canonicalize(&self, s: &str) -> String {
        self.aliases
            .get(s)
            .cloned()
            .unwrap_or_else(|| s.to_string())
    }
}