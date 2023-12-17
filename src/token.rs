use std::fmt::{write, Display, Formatter};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Token {
    pub kind: TokenKind,
    pub literal: String,
}

#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub enum TokenKind {
    EOF,
    #[default]
    Error,
    Plus,
    Minus,
    Multiply,
    Divide,
    Assign,
    Equals,
    Colon,
    SemiColon,
    Comma,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Number,
    Keyword,
    Identifier,
    Function,
    Var,
    Const,
    Bang,
    Asterisk,
    Slash,
    GreaterThan,
    LessThan,
    True,
    False,
    If,
    Else,
    Return,

    Eq,
    NotEq,
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::EOF => write!(f, "EOF"),
            TokenKind::Error => write!(f, "illegal"),
            TokenKind::Plus => write!(f, "+"),
            TokenKind::Minus => write!(f, "-"),
            TokenKind::Multiply => write!(f, "*"),
            TokenKind::Divide => write!(f, "/"),
            TokenKind::Assign => write!(f, "="),
            TokenKind::Equals => write!(f, "=="),
            TokenKind::Colon => write!(f, ":"),
            TokenKind::SemiColon => write!(f, ";"),
            TokenKind::Comma => write!(f, ","),
            TokenKind::LeftParen => write!(f, "("),
            TokenKind::RightParen => write!(f, ")"),
            TokenKind::LeftBracket => write!(f, "["),
            TokenKind::RightBracket => write!(f, "]"),
            TokenKind::LeftBrace => write!(f, "{{"),
            TokenKind::RightBrace => write!(f, "}}"),
            TokenKind::Function => write!(f, "function"),
            TokenKind::Var => write!(f, "var"),
            TokenKind::Const => write!(f, "const"),
            TokenKind::Bang => write!(f, "!"),
            TokenKind::Slash => write!(f, "#"),
            TokenKind::GreaterThan => write!(f, ">"),
            TokenKind::LessThan => write!(f, "<"),
            TokenKind::True => write!(f, "true"),
            TokenKind::False => write!(f, "false"),
            TokenKind::If => write!(f, "if"),
            TokenKind::Else => write!(f, "else"),
            TokenKind::Return => write!(f, "return"),
            TokenKind::Eq => write!(f, "=="),
            TokenKind::NotEq => write!(f, "!="),
            _ => write!(f, "other"),
        }
    }
}

pub fn lookup_ident(identifier: &str) -> TokenKind {
    match identifier {
        "var" => TokenKind::Var,
        "const" => TokenKind::Const,
        "true" => TokenKind::True,
        "false" => TokenKind::False,
        "if" => TokenKind::If,
        "else" => TokenKind::Else,
        "return" => TokenKind::Return,
        _ => TokenKind::Identifier,
    }
}
