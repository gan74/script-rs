use std::boxed::{Box};
use std::fmt;

use position::*;

type UnboxedSubTree<Name> = Tree<Name>;
type SubTree<Name> = Box<UnboxedSubTree<Name>>;


#[derive(Debug)]
pub enum TreeType<Name> {

    Empty,

    Def(Name, SubTree<Name>),
    Assign(Name, SubTree<Name>),

    Ident(Name),

    IntLit(i64),

    Add(SubTree<Name>, SubTree<Name>),
    Sub(SubTree<Name>, SubTree<Name>),
    Mul(SubTree<Name>, SubTree<Name>),
    Div(SubTree<Name>, SubTree<Name>),

    Block(Vec<UnboxedSubTree<Name>>, SubTree<Name>),

    If(SubTree<Name>, SubTree<Name>, SubTree<Name>),
    While(SubTree<Name>, SubTree<Name>),

    Error(&'static str)
}

impl<Name> TreeType<Name> {
    pub fn with_pos(self, pos: Position) -> Tree<Name> {
        Tree {
            tree: self,
            pos: pos
        }
    }
}




#[derive(Debug)]
pub struct Tree<Name> {
    pub tree: TreeType<Name>,
    pub pos: Position
}

impl<Name> Tree<Name> {
    pub fn is_error(&self) -> bool {
        match self.tree {
            TreeType::Error(_) => true,
            _ => false
        }
    }

    pub fn is_empty(&self) -> bool {
        match self.tree {
            TreeType::Empty => true,
            _ => false
        }
    }

    pub fn block_from_vec(mut stats: Vec<UnboxedSubTree<Name>>, pos: Position) -> Tree<Name> {
        let expr = stats.pop().unwrap_or(TreeType::Empty.with_pos(pos.clone()));
        TreeType::Block(stats, Box::new(expr)).with_pos(pos)
    }

    pub fn block_from_expr(expr: UnboxedSubTree<Name>, pos: Position) -> Tree<Name> {
        TreeType::Block(Vec::new(), Box::new(expr)).with_pos(pos)
    }



    pub fn for_each_subtree<'a, F: FnMut(&'a Tree<Name>) -> ()>(&'a self, mut f: F) {f(self);
        self.for_each_subtree_ref(&mut f);
    }

    fn for_each_subtree_ref<'a, F: FnMut(&'a Tree<Name>) -> ()>(&'a self, f: &mut F) {
        f(self);
        match self.tree {
            TreeType::Def(_, ref rhs) => rhs.for_each_subtree_ref(f),
            TreeType::Assign(_, ref rhs) => rhs.for_each_subtree_ref(f),
            TreeType::Add(ref lhs, ref rhs) => { lhs.for_each_subtree_ref(f); rhs.for_each_subtree_ref(f) },
            TreeType::Sub(ref lhs, ref rhs) => { lhs.for_each_subtree_ref(f); rhs.for_each_subtree_ref(f) },
            TreeType::Mul(ref lhs, ref rhs) => { lhs.for_each_subtree_ref(f); rhs.for_each_subtree_ref(f) }, 
            TreeType::Div(ref lhs, ref rhs) => { lhs.for_each_subtree_ref(f); rhs.for_each_subtree_ref(f) }, 
            TreeType::Block(ref stats, ref expr) => { for s in stats { s.for_each_subtree_ref(f); } expr.for_each_subtree_ref(f) },
            TreeType::If(ref cond, ref thenp, ref elsep) => { cond.for_each_subtree_ref(f); thenp.for_each_subtree_ref(f); elsep.for_each_subtree_ref(f) }, 
            TreeType::While(ref cond, ref body) => { cond.for_each_subtree_ref(f); body.for_each_subtree_ref(f) }, 

            TreeType::Empty | TreeType::Ident(_) | TreeType::IntLit(_) | TreeType::Error(_) => (),
        }
    }
}



impl<Name> fmt::Display for Tree<Name> where Name: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.tree {
            TreeType::Empty => write!(f, "()"),
            TreeType::Def(ref name, ref rhs) => write!(f, "let {} = {}", name, rhs),
            TreeType::Assign(ref name, ref rhs) => write!(f, "{} = {}", name, rhs),
            TreeType::Ident(ref name) => write!(f, "{}", name),
            TreeType::IntLit(val) => write!(f, "{}", val),
            TreeType::Add(ref lhs, ref rhs) => write!(f, "({} + {})", lhs, rhs),
            TreeType::Sub(ref lhs, ref rhs) => write!(f, "({} - {})", lhs, rhs),
            TreeType::Mul(ref lhs, ref rhs) => write!(f, "{} * {}", lhs, rhs),
            TreeType::Div(ref lhs, ref rhs) => write!(f, "{} / {}", lhs, rhs),
            TreeType::Block(ref stats, ref expr) => {
                let mut r = write!(f, "{{\n");
                for s in stats {
                    r = r.and_then(|_| write!(f, "{}\n", s));
                }
                r.and_then(|_| write!(f, "{}\n}}", expr))
            },
            TreeType::If(ref cond, ref thenp, ref elsep) => 
                if elsep.is_empty() { 
                    write!(f, "if {} {}", cond, thenp)
                } else {
                    write!(f, "if {} {} else {}", cond, thenp, elsep)
                },
            TreeType::While(ref cond, ref body) => write!(f, "while {} {}", cond, body),
            TreeType::Error(err) => write!(f, "<error {}: {}>", self.pos, err),

            _ => write!(f, "???")
        }
    }
}