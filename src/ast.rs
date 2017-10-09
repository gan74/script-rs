
use std::fmt::{Display, Result, Formatter};
use std::rc::{Rc};

pub type Symbol = String;
pub type Literal = String;

use utils::*;

use position::{Position};

type P = Position;


#[derive(Debug)]
pub enum Tree {
	Assign(P, Symbol, Box<Tree>),
	Add(P, Box<Tree>, Box<Tree>),
	Sub(P, Box<Tree>, Box<Tree>),
	Mul(P, Box<Tree>, Box<Tree>),
	Div(P, Box<Tree>, Box<Tree>),
	
	Call(P, Box<Tree>, Box<Tree>),
	Func(P, Vec<Symbol>, Rc<Tree>),

	Ident(P, Symbol),
	NumLit(P, f64),
	StrLit(P, Literal),
	ListLit(P, Vec<Tree>),

	Decl(P, Symbol, Box<Tree>),
	Block(P, Vec<Tree>),
	If(P, Box<Tree>, Box<Tree>, Box<Tree>),
	While(P, Box<Tree>, Box<Tree>),
	For(P, Symbol, Box<Tree>, Box<Tree>),

	Unit(P)
}


impl Display for Tree {
	fn fmt(&self, f: &mut Formatter) -> Result {
		match self {
			&Tree::Assign(_, ref l, ref r) => write!(f, "{} = {}", l, r),
			&Tree::Add(_, ref l, ref r) => write!(f, "({} + {})", l, r),
			&Tree::Sub(_, ref l, ref r) => write!(f, "({} - {})", l, r),
			&Tree::Mul(_, ref l, ref r) => write!(f, "({} * {})", l, r),
			&Tree::Div(_, ref l, ref r) => write!(f, "({} / {})", l, r),
			&Tree::Call(_, ref o, ref a) => write!(f, "{}({})", o, a),
			&Tree::Func(_, ref a, ref b) => write!(f, "({}) => {}", concat(a), b),
			&Tree::Ident(_, ref name) => write!(f, "{}", name),
			&Tree::StrLit(_, ref val) => write!(f, "\"{}\"", val),
			&Tree::NumLit(_, ref val) => write!(f, "{}", val),
			&Tree::ListLit(_, ref lst) => write!(f, "[{}]", concat(lst)),

			&Tree::Decl(_, ref s, ref e) => write!(f, "let {} = {}", s, e),
			&Tree::If(_, ref c, ref t, ref e) => write!(f, "if({}) {} else {}", c, t, e),
			&Tree::While(_, ref c, ref b) => write!(f, "while({}) {}", c, b),
			&Tree::For(_, ref n, ref l, ref b) => write!(f, "for({} : {}) {}", n, l, b),
			&Tree::Unit(_) => Ok(()),
			&Tree::Block(_, ref s) => {
				let mut r = write!(f, "{{\n");
				for s in s {
					r = r.and_then(|_| write!(f, "{}\n", s));
				}
				r.and_then(|_| write!(f, "}}"))
			}
		}
	}
}

impl Tree {
	pub fn position(&self) -> Position {
		match self {
			&Tree::Assign(ref p, _, _) => p,
			&Tree::Add(ref p, _, _) => p,
			&Tree::Sub(ref p, _, _) => p,
			&Tree::Mul(ref p, _, _) => p,
			&Tree::Div(ref p, _, _) => p,
			&Tree::Call(ref p, _, _) => p,
			&Tree::Func(ref p, _, _) => p,
			&Tree::Ident(ref p, _) => p,
			&Tree::StrLit(ref p, _) => p,
			&Tree::NumLit(ref p, _) => p,
			&Tree::ListLit(ref p, _) => p,

			&Tree::Decl(ref p, _, _) => p,
			&Tree::If(ref p, _, _, _) => p,
			&Tree::While(ref p, _, _) => p,
			&Tree::For(ref p, _, _, _) => p,
			&Tree::Unit(ref p) => p,
			&Tree::Block(ref p, _) => p
		}.clone()
	}
}
