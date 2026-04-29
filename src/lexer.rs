use crate::locale::Locale;
use crate::locale_pack::LocalePack;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind<'a> {
    LParen,
    RParen,
    String(&'a str),
    Symbol(&'a str),
    Number(&'a str),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token<'a> {
    pub kind: TokenKind<'a>,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LexError {
    UnterminatedString { at: usize },
    UnterminatedComment { at: usize },
}


pub fn lex<'a>(src: &'a str, locale: Locale) -> Result<Vec<Token<'a>>, LexError> {
    let from_pack = LocalePack::for_locale(locale);
    let sp = from_pack.delimiters;
    let mut out = Vec::new();
    let mut i = 0;

    while i < src.len() {
        let rest = &src[i..];

        let whitespace_len = rest.len() - rest.trim_start().len();
        if whitespace_len > 0 {
            i += whitespace_len;
            continue;
        }

        if let Some(after_open) = rest.strip_prefix(sp.comment.open) {
            let content_start = i + sp.comment.open.len();
            if let Some(close_idx) = src[content_start..].find(sp.comment.close) {
                i = content_start + close_idx + sp.comment.close.len();
                continue;
            } else {
                return Err(LexError::UnterminatedComment { at: i });
            }
        }

        if rest.starts_with(sp.list.open) {
            out.push(Token { kind: TokenKind::LParen, start: i, end: i + sp.list.open.len() });
            i += sp.list.open.len();
            continue;
        }
        if rest.starts_with(sp.list.close) {
            out.push(Token { kind: TokenKind::RParen, start: i, end: i + sp.list.close.len() });
            i += sp.list.close.len();
            continue;
        }

        if let Some(after_open) = rest.strip_prefix(sp.string.open) {
            let content_start = i + sp.string.open.len();
            if let Some(close_idx) = src[content_start..].find(sp.string.close) {
                let content = &src[content_start..content_start + close_idx]; // 零拷贝切片
                let end = content_start + close_idx + sp.string.close.len();
                out.push(Token {
                    kind: TokenKind::String(content),
                    start: i,
                    end,
                });
                i = end;
                continue;
            } else {
                return Err(LexError::UnterminatedString { at: i });
            }
        }

        let mut symbol_len = rest.len();
        for (pos, ch) in rest.char_indices() {
            let current = &rest[pos..];

            if ch.is_whitespace()
                || current.starts_with(sp.comment.open)
                || current.starts_with(sp.list.open)
                || current.starts_with(sp.list.close)
                || current.starts_with(sp.string.open)
                || current.starts_with(sp.string.close)
            {
                symbol_len = pos;
                break;
            }
        }

        let text = &rest[..symbol_len];
        let kind = if is_number_like(text) {
            TokenKind::Number(text)
        } else {
            TokenKind::Symbol(text)
        };

        out.push(Token {
            kind,
            start: i,
            end: i + symbol_len,
        });

        i += symbol_len;
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