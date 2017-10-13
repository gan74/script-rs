use std::fmt::{Display, Result, Formatter};

use position::*;

type P = Position;

#[derive(Debug, Clone)]
pub enum Token<'a> {
	Let(P),

	Ident(P, &'a str),

	StrLit(P, &'a str),
	NumLit(P, &'a str),

	LeftPar(P),
	RightPar(P),

	LeftBracket(P),
	RightBracket(P),

	LeftBrace(P),
	RightBrace(P),

	Assign(P),
	Plus(P),
	Minus(P),
	Times(P),
	Div(P),

	Comma(P),
	Colon(P),

	If(P),
	Else(P),
	While(P),
	For(P),

	Arrow(P)
}


impl<'a> Display for Token<'a> {
	fn fmt(&self, f: &mut Formatter) -> Result {
		match self.clone() {
			Token::Let(_) 			=> write!(f, "let"),
			Token::Ident(_, n) 		=> write!(f, "{}", n),
			Token::StrLit(_, s) 	=> write!(f, "{:?}", s),
			Token::NumLit(_, n) 	=> write!(f, "{}", n),
			Token::LeftPar(_) 		=> write!(f, "("),
			Token::RightPar(_) 		=> write!(f, ")"),
			Token::LeftBracket(_) 	=> write!(f, "["),
			Token::RightBracket(_) 	=> write!(f, "]"),
			Token::LeftBrace(_) 	=> write!(f, "{{"),
			Token::RightBrace(_) 	=> write!(f, "}}"),
			Token::Assign(_) 		=> write!(f, "="),
			Token::Plus(_) 			=> write!(f, "+"),
			Token::Minus(_) 		=> write!(f, "-"),
			Token::Times(_) 		=> write!(f, "*"),
			Token::Div(_) 			=> write!(f, "/"),
			Token::Comma(_) 		=> write!(f, ","),
			Token::Colon(_) 		=> write!(f, ":"),
			Token::If(_) 			=> write!(f, "if"),
			Token::Else(_) 			=> write!(f, "else"),
			Token::While(_) 		=> write!(f, "while"),
			Token::For(_) 			=> write!(f, "for"),
			Token::Arrow(_) 		=> write!(f, "=>")
		}
	}
}

impl<'a> Token<'a> {
	pub fn position(&self) -> P {
		match self.clone() {
			Token::Let(p) 			=> p,
			Token::Ident(p, _) 		=> p,
			Token::StrLit(p, _) 	=> p,
			Token::NumLit(p, _) 	=> p,
			Token::LeftPar(p) 		=> p,
			Token::RightPar(p) 		=> p,
			Token::LeftBracket(p) 	=> p,
			Token::RightBracket(p)	=> p,
			Token::LeftBrace(p) 	=> p,
			Token::RightBrace(p) 	=> p,
			Token::Assign(p) 		=> p,
			Token::Plus(p) 			=> p,
			Token::Minus(p) 		=> p,
			Token::Times(p) 		=> p,
			Token::Div(p) 			=> p,
			Token::Comma(p) 		=> p,
			Token::Colon(p) 		=> p,
			Token::If(p) 			=> p,
			Token::Else(p) 			=> p,
			Token::While(p) 		=> p,
			Token::For(p) 			=> p,
			Token::Arrow(p) 		=> p
		}
	}
}