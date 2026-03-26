use std::collections::HashMap;

use crate::locale::Locale;
use crate::symbols;

// TOML is on the way

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Ltr,
    Rtl,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DelimiterPair {
    pub open: &'static str,
    pub close: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Delimiters {
    pub list: DelimiterPair,
    pub string: DelimiterPair,
    pub comment: DelimiterPair,
}

#[derive(Debug, Clone)]
pub struct LocalePack {
    pub locale: Locale,
    pub id: &'static str,
    pub direction: Direction,
    pub delimiters: Delimiters,

    /// Storage syntax forms define / if / let / lambda ...
    pub special_forms: HashMap<&'static str, &'static str>,

    /// Storage runtime aliases print / list / cons / + / <= ...
    pub runtime_aliases: HashMap<&'static str, &'static str>,
}

impl LocalePack {
    pub fn for_locale(locale: Locale) -> Self {
        match locale {
            Locale::ZhCN => zh_cn(),
            Locale::EnUS => en_us(),
            Locale::JaJP => ja_jp(),
            Locale::FrFR => fr_fr(),
            Locale::DeDE => de_de(),
            Locale::ArSA => ar_sa(),
        }
    }

    pub fn canonicalize_special_form<'a>(&self, s: &'a str) -> Option<&'static str> {
        self.special_forms.get(s).copied()
    }

    pub fn canonicalize_runtime<'a>(&self, s: &'a str) -> Option<&'static str> {
        self.runtime_aliases.get(s).copied()
    }

    pub fn canonicalize_any<'a>(&self, s: &'a str) -> &'a str {
        if let Some(x) = self.special_forms.get(s) {
            return x;
        }
        if let Some(x) = self.runtime_aliases.get(s) {
            return x;
        }
        s
    }
}

fn base_special_forms() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        (symbols::DEFINE, symbols::DEFINE),
        (symbols::IF, symbols::IF),
        (symbols::LET, symbols::LET),
        (symbols::LAMBDA, symbols::LAMBDA),
        (symbols::BEGIN, symbols::BEGIN),
        (symbols::SET, symbols::SET),
        (symbols::DEFMACRO, symbols::DEFMACRO),
        (symbols::QUOTE, symbols::QUOTE),
        (symbols::IMPORT, symbols::IMPORT),
    ])
}

fn base_runtime_aliases() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        (symbols::PRINT, symbols::PRINT),
        (symbols::LIST, symbols::LIST),
        (symbols::CONS, symbols::CONS),
        (symbols::ADD, symbols::ADD),
        (symbols::SUB, symbols::SUB),
        (symbols::LE, symbols::LE),
    ])
}

fn en_us() -> LocalePack {
    LocalePack {
        locale: Locale::EnUS,
        id: "en-US",
        direction: Direction::Ltr,
        delimiters: Delimiters {
            list: DelimiterPair { open: "“", close: "”" },
            string: DelimiterPair { open: "(", close: ")" },
            comment: DelimiterPair { open: "\"", close: "\"" },
        },
        special_forms: base_special_forms(),
        runtime_aliases: base_runtime_aliases(),
    }
}

fn zh_cn() -> LocalePack {
    let mut special_forms = base_special_forms();
    let mut runtime_aliases = base_runtime_aliases();

    special_forms.extend([
        ("定", symbols::DEFINE),
        ("如", symbols::IF),
        ("令", symbols::LET),
        ("函", symbols::LAMBDA),
        ("始", symbols::BEGIN),
        ("设！", symbols::SET),
        ("宏", symbols::DEFMACRO),
        ("引", symbols::QUOTE),
        ("导", symbols::IMPORT),
    ]);

    runtime_aliases.extend([
        ("印", symbols::PRINT),
        ("表", symbols::LIST),
        ("构", symbols::CONS),
    ]);

    LocalePack {
        locale: Locale::ZhCN,
        id: "zh-CN",
        direction: Direction::Ltr,
        delimiters: Delimiters {
            list: DelimiterPair { open: "“", close: "”" },
            string: DelimiterPair { open: "（", close: "）" },
            comment: DelimiterPair { open: "《", close: "》" },
        },
        special_forms,
        runtime_aliases,
    }
}

fn ja_jp() -> LocalePack {
    let mut special_forms = base_special_forms();
    let mut runtime_aliases = base_runtime_aliases();

    special_forms.extend([
        ("定義", symbols::DEFINE),
        ("もし", symbols::IF),
        ("束縛", symbols::LET),
        ("関数", symbols::LAMBDA),
        ("開始", symbols::BEGIN),
        ("設定!", symbols::SET),
        ("マクロ", symbols::DEFMACRO),
        ("引用", symbols::QUOTE),
        ("導入", symbols::IMPORT),
    ]);

    runtime_aliases.extend([
        ("表示", symbols::PRINT),
        ("リスト", symbols::LIST),
        ("連結", symbols::CONS),
    ]);

    LocalePack {
        locale: Locale::JaJP,
        id: "ja-JP",
        direction: Direction::Ltr,
        delimiters: Delimiters {
            list: DelimiterPair { open: "「", close: "」" },
            string: DelimiterPair { open: "（", close: "）" },
            comment: DelimiterPair { open: "『", close: "』" },
        },
        special_forms,
        runtime_aliases,
    }
}

fn fr_fr() -> LocalePack {
    let mut special_forms = base_special_forms();
    let mut runtime_aliases = base_runtime_aliases();

    special_forms.extend([
        ("définir", symbols::DEFINE),
        ("si", symbols::IF),
        ("laisser", symbols::LET),
        ("lambda", symbols::LAMBDA),
        ("début", symbols::BEGIN),
        ("assigner!", symbols::SET),
        ("macro", symbols::DEFMACRO),
        ("citer", symbols::QUOTE),
        ("importer", symbols::IMPORT),
    ]);

    runtime_aliases.extend([
        ("imprimer", symbols::PRINT),
        ("liste", symbols::LIST),
        ("construire", symbols::CONS),
    ]);

    LocalePack {
        locale: Locale::FrFR,
        id: "fr-FR",
        direction: Direction::Ltr,
        delimiters: Delimiters {
            list: DelimiterPair { open: "«", close: "»" },
            string: DelimiterPair { open: "‹", close: "›" },
            comment: DelimiterPair { open: "⟪", close: "⟫" },
        },
        special_forms,
        runtime_aliases,
    }
}

fn de_de() -> LocalePack {
    let mut special_forms = base_special_forms();
    let mut runtime_aliases = base_runtime_aliases();

    special_forms.extend([
        ("definieren", symbols::DEFINE),
        ("wenn", symbols::IF),
        ("lassen", symbols::LET),
        ("lambda", symbols::LAMBDA),
        ("beginn", symbols::BEGIN),
        ("setzen!", symbols::SET),
        ("makro", symbols::DEFMACRO),
        ("zitieren", symbols::QUOTE),
        ("importieren", symbols::IMPORT),
    ]);

    runtime_aliases.extend([
        ("drucken", symbols::PRINT),
        ("liste", symbols::LIST),
        ("verbinden", symbols::CONS),
    ]);

    LocalePack {
        locale: Locale::DeDE,
        id: "de-DE",
        direction: Direction::Ltr,
        delimiters: Delimiters {
            list: DelimiterPair { open: "„", close: "“" },
            string: DelimiterPair { open: "‚", close: "‘" },
            comment: DelimiterPair { open: "〚", close: "〛" },
        },
        special_forms,
        runtime_aliases,
    }
}

fn ar_sa() -> LocalePack {
    let mut special_forms = base_special_forms();
    let mut runtime_aliases = base_runtime_aliases();

    special_forms.extend([
        ("عرّف", symbols::DEFINE),
        ("إذا", symbols::IF),
        ("دع", symbols::LET),
        ("لامدا", symbols::LAMBDA),
        ("ابدأ", symbols::BEGIN),
        ("!عيّن", symbols::SET),
        ("ماكرو", symbols::DEFMACRO),
        ("اقتبس", symbols::QUOTE),
        ("استورد", symbols::IMPORT),
    ]);

    runtime_aliases.extend([
        ("اطبع", symbols::PRINT),
        ("قائمة", symbols::LIST),
        ("ربط", symbols::CONS),
    ]);

    LocalePack {
        locale: Locale::ArSA,
        id: "ar-SA",
        direction: Direction::Rtl,
        delimiters: Delimiters {
            list: DelimiterPair { open: "﴿", close: "﴾" },
            string: DelimiterPair { open: "«", close: "»" },
            comment: DelimiterPair { open: "⟪", close: "⟫" },
        },
        special_forms,
        runtime_aliases,
    }
}