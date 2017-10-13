
use std::rc::{Rc};

use context::*;
use symbol::*;
use transform::*;
use position::*;
use ast::*;
use utils::*;

pub struct NameAnalysis {
}



impl<'a> NameAnalysis {
	pub fn new() -> NameAnalysis {
		NameAnalysis {
		}
	}
}


impl TryTransform for NameAnalysis {
	fn transform(&mut self, tree: Tree, ctx: &mut Context) -> Result<Tree, Error> {
		let with_pos = |r: Result<SymbolRef, ErrorKind>, p: &Position| r.map_err(|e| e.with_position(&p));
		Ok(match tree {
			Tree::IdentByName(p, n) => { let re = with_pos(ctx.by_name(n), &p)?; Tree::Ident(p, re) },
			Tree::AssignByName(p, n, r) => { let re = with_pos(ctx.by_name(n), &p)?; Tree::Assign(p, re, r) },
			Tree::DeclByName(p, n, r) => { let re = with_pos(ctx.decl(n), &p)?; Tree::Decl(p, re, r) },
			Tree::ForByName(p, n, c, b) => { let re = with_pos(ctx.decl(n), &p)?; Tree::For(p, re, c, b) },
			Tree::FuncByName(p, s, b) => {
				let a = s.into_iter().map(|s| ctx.decl(s)).collect::<Result<Vec<_>, ErrorKind>>().map_err(|e| e.with_position(&p))?;
				Tree::Func(p, a, Rc::new(*b))
			},

			t => t
		})
	}
}

