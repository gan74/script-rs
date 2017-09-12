
use tokens::{Token};
use position::{Position};

use utils::*;

pub struct Tokenizer<'a> {
	buffer: &'a str,
	pos: Position
}

impl<'a> Tokenizer<'a> {
	pub fn tokenize(buffer: &'a str) -> Tokenizer {
		Tokenizer {
			buffer: buffer,
			pos: Position::new(String::from("string... "))
		}
	}


	fn next_token(&mut self) -> Token<'a> {
		let mut chars = self.buffer.chars();
		let nx = chars.next().unwrap();
		let pe = chars.next().clone();
		match nx {
			'=' if pe == Some('>') => { self.advance(2); Token::Arrow(self.tp()) },
			',' => { self.advance(1); Token::Comma(self.tp()) },
			':' => { self.advance(1); Token::Colon(self.tp()) },
			'=' => { self.advance(1); Token::Assign(self.tp()) },
			'+' => { self.advance(1); Token::Plus(self.tp()) },
			'-' => { self.advance(1); Token::Minus(self.tp()) },
			'*' => { self.advance(1); Token::Times(self.tp()) },
			'/' => { self.advance(1); Token::Div(self.tp()) },
			'(' => { self.advance(1); Token::LeftPar(self.tp()) },
			')' => { self.advance(1); Token::RightPar(self.tp()) },
			'[' => { self.advance(1); Token::LeftBracket(self.tp()) },
			']' => { self.advance(1); Token::RightBracket(self.tp()) },
			'{' => { self.advance(1); Token::LeftBrace(self.tp()) },
			'}' => { self.advance(1); Token::RightBrace(self.tp()) },
			'0'...'9' => self.next_num(),
			'"' => self.next_str(),
			_ => self.next_ident(),
		}
	}

	fn next_num(&mut self) -> Token<'a> {
		for c in self.buffer.chars().enumerate().skip(1) {
			if !c.1.is_numeric() && c.1 != '.' {
				return Token::NumLit(self.tp(), self.advance(c.0))
			}
		}
		Token::NumLit(self.tp(), self.advance(self.buffer.len()))
	}

	fn next_str(&mut self) -> Token<'a> {
		for c in self.buffer.chars().enumerate().skip(1) {
			if c.1 == '"' {
				let s = self.advance(c.0 + 1);
				return Token::StrLit(self.tp(), &s[1..(s.len() - 1)]);
			}
		}
		fatal_pos("Unterminated string literal", self.tp());
	}

	fn next_ident(&mut self) -> Token<'a> {
		for c in self.buffer.chars().enumerate() {
			if !c.1.is_alphanumeric() && c.1 != '_' {
				return ident_type(self.advance(c.0), self.tp())
			}
		}
		ident_type(self.advance(self.buffer.len()), self.tp())
	}






	fn advance(&mut self, len: usize) -> &'a str  {
		self.pos.col += len;
		let i = &self.buffer[..len];
		self.buffer = &self.buffer[len..];
		i
	}

	fn skip_whitespace(&mut self) {
		for c in self.buffer.chars().enumerate() {
			if !c.1.is_whitespace() {
				self.buffer = &self.buffer[c.0..];
				break;
			} else if c.1 == '\n' {
				self.pos.next_line();
			} else {
				self.pos.col += 1;
			}
		}
	}

	fn tp(&self) -> Position {
		self.pos.clone()
	}
}


impl<'a> Iterator for Tokenizer<'a> {
	type Item = Token<'a>;

	fn next(&mut self) -> Option<Self::Item> {
		self.skip_whitespace();
		if self.buffer.is_empty() {
			None
		} else {
			Some(self.next_token())
 		}
	}
}





fn ident_type<'a>(word: &'a str, pos: Position) -> Token<'a> {
	if word.chars().next().is_none() {
		fatal_pos("Empty identifier", pos);
	}
	match word {
		"let" => Token::Let(pos.clone()),
		"if" => Token::If(pos.clone()),
		"else" => Token::Else(pos.clone()),
		"while" => Token::While(pos.clone()),
		"for" => Token::For(pos.clone()),
		ident => Token::Ident(pos.clone(), ident)
	}
}