use std::fmt::{write, Display, Formatter};

#[derive(PartialEq, Debug)]
pub struct Token<'source> {
    pub kind: TokenKind<'source>,
    pub literal: String,
}

#[derive(Debug, PartialEq, Clone, Copy, Debug)]
pub enum TokenKind<'source> {
    EOF,
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
    LeftParent,
    RightParent,
    LeftAng,
    RightAng,
    LeftBrace,
    RightBrace,
    Number(&'source str),
    Keyword(&'source str),
    Identifier(&'source str),
    Function,
    Var,
    Const,
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::EOF => write!(f, "EOF"),
            TokenKind::Error => write!(f, "Ilegal"),
            TokenKind::Plus => write!(f, "+"),
            TokenKind::Minus => write!(f, "-"),
            TokenKind::Multiply => write!(f, "*"),
            TokenKind::Divide => write!(f, "/"),
            TokenKind::Assign => write!(f, "="),
            TokenKind::Equals => write!(f, "=="),
            TokenKind::Colon => write!(f, ":"),
            TokenKind::SemiColon => write!(f, ";"),
            TokenKind::Comma => write!(f, ","),
            TokenKind::LeftParent => write!(f, "("),
            TokenKind::RightParent => write!(f, ")"),
            TokenKind::LeftAng => write!(f, "["),
            TokenKind::RightAng => write!(f, "]"),
            TokenKind::LeftBrace => write!(f, "{{"),
            TokenKind::RightBrace => write!(f, "}}"),
            TokenKind::Function => write!(f, "Function"),
            TokenKind::Var => write!(f, "var"),
            TokenKind::Const => write!(f, "const"),
            _ => write!(f, "other"),
        }
    }
}
