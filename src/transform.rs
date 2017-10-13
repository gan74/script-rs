
use ast::*;
use context::*;
use utils::*;

pub trait Transform {
	fn transform(&mut self, tree: Tree, _: &mut Context) -> Tree {
		tree
	}
}

impl Transform for FnMut(Tree, &mut Context) -> Tree {
	fn transform(&mut self, tree: Tree, ctx: &mut Context) -> Tree {
		self(tree, ctx)
	}
}


pub trait TryTransform {
	fn transform(&mut self, tree: Tree, _: &mut Context) -> Result<Tree, Error> {
		Ok(tree)
	}
}

impl<T: Transform> TryTransform for T {
	fn transform(&mut self, tree: Tree, ctx: &mut Context) -> Result<Tree, Error> {
		Ok(self.transform(tree, ctx))
	}
}

impl TryTransform for FnMut(Tree, &mut Context) -> Result<Tree, Error> {
	fn transform(&mut self, tree: Tree, mut  ctx: &mut Context) -> Result<Tree, Error> {
		self(tree, ctx)
	}
}
