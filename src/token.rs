
use position::*;

#[derive(Clone, PartialEq, Debug)]
pub enum TokenType {

    Ident(String),

    StrLit(String),
    NumLit(String),

    LeftPar,
    RightPar,

    LeftBracket,
    RightBracket,

    LeftBrace,
    RightBrace,

    Assign,

    Plus,
    Minus,
    Star,
    Slash,

    Comma,
    Colon,

    If,
    Else,
    While,

    Let,

    Error
}

impl TokenType {
    pub fn with_pos(self, pos: Position) -> Token {
        Token {
            token: self,
            pos: pos
        }
    }
}

#[derive(Clone, Debug)]
pub struct Token {
    pub token: TokenType,
    pub pos: Position
}