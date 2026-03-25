use std::collections::HashMap;

use crate::locale::Locale;
use crate::locale_pack::LocalePack;

#[derive(Debug, Clone)]
pub struct KeywordMap {
    aliases: HashMap<String, String>,
}

impl KeywordMap {
    pub fn new(locale: Locale) -> Self {
        let pack = LocalePack::for_locale(locale);
        let mut aliases = HashMap::new();

        for (k, v) in pack.special_forms.iter() {
            aliases.insert((*k).to_string(), (*v).to_string());
        }

        for (k, v) in pack.runtime_aliases.iter() {
            aliases.insert((*k).to_string(), (*v).to_string());
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