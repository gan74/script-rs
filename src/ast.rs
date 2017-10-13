
use std::fmt::{Display, Formatter};
use std::result::{Result};
use std::rc::{Rc};

use std::fmt;

pub type Literal = String;

use position::*;
use utils::*;
use transform::*;
use symbol::*;
use context::*;

type P = Position;

#[derive(Debug)]
pub enum Tree {
	AssignByName(P, SymbolName, Box<Tree>),
	IdentByName(P, SymbolName),
	DeclByName(P, SymbolName, Box<Tree>),
	ForByName(P, SymbolName, Box<Tree>, Box<Tree>),
	FuncByName(P, Vec<SymbolName>, Box<Tree>),

	Ident(P, SymbolRef),
	Assign(P, SymbolRef, Box<Tree>),
	Decl(P, SymbolRef, Box<Tree>),
	For(P, SymbolRef, Box<Tree>, Box<Tree>),
	Func(P, Vec<SymbolRef>, Rc<Tree>),

	Add(P, Box<Tree>, Box<Tree>),
	Sub(P, Box<Tree>, Box<Tree>),
	Mul(P, Box<Tree>, Box<Tree>),
	Div(P, Box<Tree>, Box<Tree>),
	
	Call(P, Box<Tree>, Box<Tree>),

	NumLit(P, f64),
	StrLit(P, Literal),
	ListLit(P, Vec<Tree>),

	Block(P, Vec<Tree>),
	If(P, Box<Tree>, Box<Tree>, Box<Tree>),
	While(P, Box<Tree>, Box<Tree>),

	Unit(P),
}






impl Tree {
	pub fn for_each<F: FnMut(&Tree) -> ()>(&self, mut func: F) {
		self.for_each_ref(&mut func);
	}

	pub fn transform<'a, F: TryTransform>(self, mut f: F, ctx: &mut Context) -> Result<Tree, Error> {
		self.try_transform_ref(&mut f, ctx)
	}



	fn is_rec(&self) -> bool {
		match self {
			&Tree::FuncByName(..) | &Tree::Func(..) => true,
			&Tree::ForByName(..) | &Tree::For(..) => true,
			&Tree::Block(..) | &Tree::If(..) | &Tree::While(..) => true,
			_ => false
		}
	}

	fn for_each_ref<F: FnMut(&Tree) -> ()>(&self, func: &mut F) {
		match self {
			&Tree::AssignByName(_, _, ref r) => r.for_each_ref(func),
			&Tree::DeclByName(_, _, ref e) => e.for_each_ref(func),
			&Tree::ForByName(_, _, ref l, ref b) => { l.for_each_ref(func); b.for_each_ref(func) },
			&Tree::FuncByName(_, _, ref b) => b.for_each_ref(func),

			&Tree::Add(_, ref l, ref r) => { l.for_each_ref(func); r.for_each_ref(func) },
			&Tree::Sub(_, ref l, ref r) => { l.for_each_ref(func); r.for_each_ref(func) },
			&Tree::Mul(_, ref l, ref r) => { l.for_each_ref(func); r.for_each_ref(func) },
			&Tree::Div(_, ref l, ref r) => { l.for_each_ref(func); r.for_each_ref(func) },
			&Tree::Call(_, ref o, ref a) => { o.for_each_ref(func); a.for_each_ref(func) },
			&Tree::ListLit(_, ref lst) => for t in lst.iter() { t.for_each_ref(func) },
			&Tree::If(_, ref c, ref t, ref e) => { c.for_each_ref(func); t.for_each_ref(func); e.for_each_ref(func) },
			&Tree::While(_, ref c, ref b) => { c.for_each_ref(func); b.for_each_ref(func) },
			&Tree::Block(_, ref s) => for t in s.iter() { t.for_each_ref(func) },

			&Tree::Assign(_, _, ref r) => r.for_each_ref(func),
			&Tree::Decl(_, _, ref r) => r.for_each_ref(func),
			&Tree::For(_, _, ref c, ref b) => { c.for_each_ref(func); b.for_each_ref(func) },
			&Tree::Func(_, _, ref b) => b.for_each_ref(func),

			_ => ()
		}
		func(self);
	}

	fn try_transform_ref<F: TryTransform>(self, f: &mut F, ctx: &mut Context) -> Result<Tree, Error> {
		let tr = |t: Box<Tree>, f: &mut F, ctx: &mut Context| -> Result<Box<Tree>, Error> { Ok(box_(t.try_transform_ref(f, ctx)?)) };

		let rec = self.is_rec();
		if rec {
			ctx.push();
		}
 
		let r = Ok(match f.transform(self, ctx)? {
			Tree::AssignByName(p, s, r) => Tree::AssignByName(p, s, tr(r, f, ctx)?),
			Tree::DeclByName(p, s, r) => Tree::DeclByName(p, s, tr(r, f, ctx)?),
			Tree::ForByName(p, s, c, b) => Tree::ForByName(p, s, tr(c, f, ctx)?, tr(b, f, ctx)?),
			Tree::FuncByName(p, s, b) => Tree::FuncByName(p, s, tr(b, f, ctx)?),

			Tree::Add(p, l, r) => Tree::Add(p, tr(l, f, ctx)?, tr(r, f, ctx)?),
			Tree::Sub(p, l, r) => Tree::Sub(p, tr(l, f, ctx)?, tr(r, f, ctx)?),
			Tree::Mul(p, l, r) => Tree::Mul(p, tr(l, f, ctx)?, tr(r, f, ctx)?),
			Tree::Div(p, l, r) => Tree::Div(p, tr(l, f, ctx)?, tr(r, f, ctx)?),
			Tree::Call(p, o, a) => Tree::Call(p, tr(o, f, ctx)?, tr(a, f, ctx)?),
			Tree::ListLit(p, l) => Tree::ListLit(p, l.into_iter().map(|t| t.try_transform_ref(f, ctx)).collect::<Result<Vec<_>, Error>>()?),
			Tree::Block(p, l) => Tree::Block(p, l.into_iter().map(|t| t.try_transform_ref(f, ctx)).collect::<Result<Vec<_>, Error>>()?),
			Tree::If(p, c, t, e) => Tree::If(p, tr(c, f, ctx)?, tr(t, f, ctx)?, tr(e, f, ctx)?),
			Tree::While(p, c, b) => Tree::While(p, tr(c, f, ctx)?, tr(b, f, ctx)?),

			Tree::Assign(p, i, r) => Tree::Assign(p, i, tr(r, f, ctx)?),
			Tree::Decl(p, i, r) => Tree::Decl(p, i, tr(r, f, ctx)?),
			Tree::For(p, i, c, b) => Tree::For(p, i, tr(c, f, ctx)?, tr(b, f, ctx)?),
			Tree::Func(p, s, b) => {
				let b = Rc::try_unwrap(b).map_err(|_| ErrorKind::Generic("function in use".to_owned()).with_position(&p))?;
				Tree::Func(p, s, Rc::new(b.try_transform_ref(f, ctx)?))
			},

			t => t
		});

		if rec {
			ctx.pop();
		}
		r
	}
}






impl Display for Tree {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self {
			&Tree::IdentByName(_, ref name) => write!(f, "{}", name),
			&Tree::AssignByName(_, ref l, ref r) => write!(f, "{} = {}", l, r),
			&Tree::DeclByName(_, ref s, ref e) => write!(f, "let {} = {}", s, e),
			&Tree::ForByName(_, ref n, ref l, ref b) => write!(f, "for({} : {}) {}", n, l, b),
			&Tree::FuncByName(_, ref a, ref b) => write!(f, "({}) => {}", concat(a), b),

			&Tree::Add(_, ref l, ref r) => write!(f, "({} + {})", l, r),
			&Tree::Sub(_, ref l, ref r) => write!(f, "({} - {})", l, r),
			&Tree::Mul(_, ref l, ref r) => write!(f, "({} * {})", l, r),
			&Tree::Div(_, ref l, ref r) => write!(f, "({} / {})", l, r),
			&Tree::Call(_, ref o, ref a) => write!(f, "{}({})", o, a),
			&Tree::StrLit(_, ref val) => write!(f, "\"{}\"", val),
			&Tree::NumLit(_, ref val) => write!(f, "{}", val),
			&Tree::ListLit(_, ref lst) => write!(f, "[{}]", concat(lst)),
			&Tree::If(_, ref c, ref t, ref e) => write!(f, "if({}) {} else {}", c, t, e),
			&Tree::While(_, ref c, ref b) => write!(f, "while({}) {}", c, b),
			&Tree::Unit(_) => Ok(()),
			&Tree::Block(_, ref s) => {
				let mut r = write!(f, "{{\n");
				for s in s {
					r = r.and_then(|_| write!(f, "{}\n", s));
				}
				r.and_then(|_| write!(f, "}}"))
			},

			&Tree::Ident(_, ref re) => write!(f, "{}", re),
			&Tree::Assign(_, ref re, ref rhs) => write!(f, "{} = {}", re, rhs),
			&Tree::Decl(_, ref re, ref rhs) => write!(f, "let {} = {}", re, rhs),
			&Tree::For(_, ref re, ref c, ref b) => write!(f, "for({} : {}) {}", re, c, b),
			&Tree::Func(_, ref a, ref b) => write!(f, "({}) => {}", concat(a), b),
		}
	}
}

impl Tree {
	pub fn position(&self) -> Position {
		match self {
			&Tree::IdentByName(ref p, _) => p,
			&Tree::AssignByName(ref p, _, _) => p,
			&Tree::DeclByName(ref p, _, _) => p,
			&Tree::ForByName(ref p, _, _, _) => p,
			&Tree::FuncByName(ref p, _, _) => p,

			&Tree::Add(ref p, _, _) => p,
			&Tree::Sub(ref p, _, _) => p,
			&Tree::Mul(ref p, _, _) => p,
			&Tree::Div(ref p, _, _) => p,
			&Tree::Call(ref p, _, _) => p,
			&Tree::StrLit(ref p, _) => p,
			&Tree::NumLit(ref p, _) => p,
			&Tree::ListLit(ref p, _) => p,
			&Tree::If(ref p, _, _, _) => p,
			&Tree::While(ref p, _, _) => p,
			&Tree::Unit(ref p) => p,
			&Tree::Block(ref p, _) => p,

			&Tree::Ident(ref p, _) => p,
			&Tree::Assign(ref p, _, _) => p,
			&Tree::Decl(ref p, _, _) => p,
			&Tree::For(ref p, _, _, _) => p,
			&Tree::Func(ref p, _, _) => p,
		}.clone()
	}
}
