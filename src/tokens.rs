
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

	LeftBrace(P),
	RightBrace(P),

	Assign(P),
	Plus(P),
	Minus(P),

	Comma(P),
	Colon(P),

	If(P),
	Else(P),
	While(P),
	For(P),

	Arrow(P)

}