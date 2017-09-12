
use std::fmt::{Display, Result, Formatter};
use std::rc::{Rc};

pub type Symbol = String;
pub type Literal = String;

use utils::*;


#[derive(Debug)]
pub enum Tree {
	Assign(Symbol, Box<Tree>),
	Add(Box<Tree>, Box<Tree>),
	Sub(Box<Tree>, Box<Tree>),
	Mul(Box<Tree>, Box<Tree>),
	Div(Box<Tree>, Box<Tree>),
	
	Call(Box<Tree>, Box<Tree>),

	Func(Vec<Symbol>, Rc<Tree>),

	Ident(Symbol),

	NumLit(f64),
	StrLit(Literal),

	ListLit(Vec<Tree>),

	// controls
	Decl(Symbol, Box<Tree>),
	Block(Vec<Tree>),

	If(Box<Tree>, Box<Tree>, Box<Tree>),
	While(Box<Tree>, Box<Tree>),
	For(Symbol, Box<Tree>, Box<Tree>),

	Unit
}


impl Display for Tree {
	fn fmt(&self, f: &mut Formatter) -> Result {
		match self {
			&Tree::Assign(ref l, ref r) => write!(f, "{} = {}", l, r),
			&Tree::Add(ref l, ref r) => write!(f, "({} + {})", l, r),
			&Tree::Sub(ref l, ref r) => write!(f, "({} - {})", l, r),
			&Tree::Mul(ref l, ref r) => write!(f, "({} * {})", l, r),
			&Tree::Div(ref l, ref r) => write!(f, "({} / {})", l, r),
			&Tree::Call(ref o, ref a) => write!(f, "{}({})", o, a),
			&Tree::Func(ref a, ref b) => write!(f, "({}) => {}", concat(a), b),
			&Tree::Ident(ref name) => write!(f, "{}", name),
			&Tree::StrLit(ref val) => write!(f, "\"{}\"", val),
			&Tree::NumLit(ref val) => write!(f, "{}", val),
			&Tree::ListLit(ref lst) => write!(f, "[{}]", concat(lst)),

			&Tree::Decl(ref s, ref e) => write!(f, "let {} = {}", s, e),
			&Tree::If(ref c, ref t, ref e) => write!(f, "if({}) {} else {}", c, t, e),
			&Tree::While(ref c, ref b) => write!(f, "while({}) {}", c, b),
			&Tree::For(ref n, ref l, ref b) => write!(f, "for({} : {}) {}", n, l, b),
			&Tree::Unit => Ok(()),
			&Tree::Block(ref s) => {
				let mut r = write!(f, "{{\n");
				for s in s {
					r = r.and_then(|_| write!(f, "{}\n", s));
				}
				r.and_then(|_| write!(f, "}}"))
			}
		}
	}
}

