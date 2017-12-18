use position::*;
use token::*;

use std::str::{Chars};

#[derive(Clone)]
pub struct Tokenizer<'a> {
    chars: Chars<'a>,
    pos: Position
}

impl<'a> Tokenizer<'a> {
    pub fn tokenize(chars: Chars<'a>) -> Tokenizer {
        Tokenizer {
            chars: chars,
            pos: Position::new("string... ")
        }
    }

    fn next_str(&mut self) -> TokenType {
        let mut str_lit = String::new();
        loop {
            if let Some(c) = self.next_char() {
                if c == '"' {
                    return TokenType::StrLit(str_lit);
                }
                str_lit.push(c);
            } else {
                return TokenType::Error;
            }
        }
        
    }

    fn next_num(&mut self, c: char) -> TokenType {
        let len = self.chars.clone().take_while(|c| c.is_numeric()).count();
        let mut num = String::with_capacity(len + 1);
        num.push(c);
        for _ in 0..len {
            num.push(self.next_char().unwrap());
        }
        TokenType::NumLit(num)
    }

    fn next_ident_string(&mut self) -> &str {
        let len = self.chars.clone().take_while(|c| c.is_alphanumeric()).count();
        let r = &self.chars.as_str()[..len];
         for _ in 0..len {
           self.next_char();
        }
        r
    }


    fn next_token(&mut self) -> Option<Token> {
        loop {
            let token_pos = self.pos.clone();
            if let Some(next_char) = self.next_char() {
                if next_char.is_whitespace() {
                    continue;
                }
                return Some(match next_char {
                    '=' =>
                        match self.chars.clone().next() {
                            Some('=') => { self.next_char(); TokenType::Eq },
                            _ => TokenType::Assign
                        },
                        
                    '!' =>
                        match self.chars.clone().next() {
                            Some('=') => { self.next_char(); TokenType::Neq },
                            _ => TokenType::Not
                        },

                    '+' => TokenType::Plus,
                    '-' => TokenType::Minus,
                    '*' => TokenType::Star,
                    '/' => TokenType::Slash,
                    ',' => TokenType::Comma,
                    ':' => TokenType::Colon,
                    '(' => TokenType::LeftPar,
                    ')' => TokenType::RightPar,
                    '{' => TokenType::LeftBrace,
                    '}' => TokenType::RightBrace,
                    '[' => TokenType::LeftBracket,
                    ']' => TokenType::RightBracket,

                    '"' => self.next_str(),
                    c if c.is_numeric() => self.next_num(c),
                    c if c.is_alphabetic() => 
                        match (c, self.next_ident_string()) {
                            ('i', "f") => TokenType::If,
                            ('e', "lse") => TokenType::Else,
                            ('l', "et") => TokenType::Let,
                            ('w', "hile") => TokenType::While,
                            (c, s) => {
                                let mut name = String::with_capacity(s.len() + 1);
                                name.push(c);
                                name.push_str(s);
                                TokenType::Ident(name)
                            }
                        },
                    _ => TokenType::Error

                }.with_pos(token_pos));
            } else {
                return None;
            }
        }
    }

    

    fn next_char(&mut self) -> Option<char> {
        let c = self.chars.next();
        match c { 
            Some('\n') => self.pos.next_line(),
            _ => self.pos.next_col()
        }
        c
    }

}


impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

