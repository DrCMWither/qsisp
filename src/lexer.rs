use crate::locale::Locale;
use crate::locale_pack::LocalePack;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    LParen,
    RParen,
    String(String),
    Symbol(String),
    Number(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LexError {
    UnterminatedString { at: usize },
    UnterminatedComment { at: usize },
}


pub fn lex(src: &str, locale: Locale) -> Result<Vec<Token>, LexError> {
    let fromPack = LocalePack::for_locale(locale);
    let sp = fromPack.delimiters;
    let mut out = Vec::new();
    let mut i = 0;

    while i < src.len() {
        let rest = &src[i..];

        if rest.starts_with(sp.comment.open) {
            i += sp.comment.open.len();
            loop {
                if i >= src.len() {
                    return Err(LexError::UnterminatedComment { at: i });
                }
                if src[i..].starts_with(sp.comment.close) {
                    i += sp.comment.close.len();
                    break;
                }
                let ch = src[i..].chars().next().unwrap();
                i += ch.len_utf8();
            }
            continue;
        }

        if rest.starts_with(sp.list.open) {
            let start = i;
            i += sp.list.open.len();
            out.push(Token {
                kind: TokenKind::LParen,
                start,
                end: i,
            });
            continue;
        }

        if rest.starts_with(sp.list.close) {
            let start = i;
            i += sp.list.close.len();
            out.push(Token {
                kind: TokenKind::RParen,
                start,
                end: i,
            });
            continue;
        }

        if rest.starts_with(sp.string.open) {
            let start = i;
            i += sp.string.open.len();
            let content_start = i;

            loop {
                if i >= src.len() {
                    return Err(LexError::UnterminatedString { at: start });
                }
                if src[i..].starts_with(sp.string.close) {
                    let content = src[content_start..i].to_string();
                    i += sp.string.close.len();
                    out.push(Token {
                        kind: TokenKind::String(content),
                        start,
                        end: i,
                    });
                    break;
                }
                let ch = src[i..].chars().next().unwrap();
                i += ch.len_utf8();
            }
            continue;
        }

        let ch = rest.chars().next().unwrap();
        if ch.is_whitespace() {
            i += ch.len_utf8();
            continue;
        }

        let start = i;
        while i < src.len() {
            let r = &src[i..];
            if r.starts_with(sp.comment.open)
                || r.starts_with(sp.list.open)
                || r.starts_with(sp.list.close)
                || r.starts_with(sp.string.open)
                || r.starts_with(sp.string.close)
            {
                break;
            }

            let ch = r.chars().next().unwrap();
            if ch.is_whitespace() {
                break;
            }

            i += ch.len_utf8();
        }

        let text = src[start..i].to_string();
        let kind = if is_number_like(&text) {
            TokenKind::Number(text)
        } else {
            TokenKind::Symbol(text)
        };

        out.push(Token {
            kind,
            start,
            end: i,
        });
    }

    Ok(out)
}

fn is_number_like(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }

    if let Some(rest) = s.strip_prefix('-') {
        !rest.is_empty() && rest.chars().all(|c| c.is_ascii_digit())
    } else {
        s.chars().all(|c| c.is_ascii_digit())
    }
}