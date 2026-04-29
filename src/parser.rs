use crate::lexer::{Token, TokenKind};

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Symbol(String),
    Number(i64),
    String(String),
    List(Vec<Expr>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    UnexpectedEof,
    UnexpectedRParen { at: usize },
    MissingRParen,
    InvalidNumber { text: String, at: usize },
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Number(n) => write!(f, "{}", n),
            Expr::String(s) => write!(f, "\"{}\"", s),
            Expr::Symbol(s) => write!(f, "{}", s),
            Expr::List(items) => {
                let items_str: Vec<String> = items.iter().map(|e| e.to_string()).collect();
                write!(f, "({})", items_str.join(" "))
            }
        }
    }
}

pub fn parse(tokens: &[Token]) -> Result<Vec<Expr>, ParseError> {
    let mut pos = 0;
    let mut exprs = Vec::new();

    while pos < tokens.len() {
        exprs.push(parse_expr(tokens, &mut pos)?);
    }

    Ok(exprs)
}

fn parse_expr(tokens: &[Token], pos: &mut usize) -> Result<Expr, ParseError> {
    if *pos >= tokens.len() {
        return Err(ParseError::UnexpectedEof);
    }

    let tok = &tokens[*pos];
    match &tok.kind {
        TokenKind::LParen => parse_list(tokens, pos),
        TokenKind::RParen => Err(ParseError::UnexpectedRParen { at: tok.start }),
        TokenKind::String(s) => {
            *pos += 1;
            Ok(Expr::String(s.to_string()))
        }
        TokenKind::Symbol(s) => {
            *pos += 1;
            Ok(Expr::Symbol(s.to_string()))
        }
        TokenKind::Number(n) => {
            let at = tok.start;
            let text = n.clone();
            *pos += 1;
            match text.parse::<i64>() {
                Ok(v) => Ok(Expr::Number(v)),
                Err(_) => Err(ParseError::InvalidNumber { text: text.to_string(), at }),
            }
        }
    }
}

fn parse_list(tokens: &[Token], pos: &mut usize) -> Result<Expr, ParseError> {
    *pos += 1; // Take LParen
    let mut items = Vec::new();

    while *pos < tokens.len() {
        match &tokens[*pos].kind {
            TokenKind::RParen => {
                *pos += 1; // Take RParen
                return Ok(Expr::List(items));
            }
            _ => items.push(parse_expr(tokens, pos)?),
        }
    }

    Err(ParseError::MissingRParen)
}