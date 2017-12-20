use std::boxed::Box;
use std::rc::Rc;
use std::ops::*;
use std::fmt;

use position::*;
use map_in_place::*;

type UnboxedSubTree<Name> = Tree<Name>;
type SubTree<Name> = Box<UnboxedSubTree<Name>>;

#[derive(Debug, PartialEq, Clone)]
pub enum TreeType<Name> {
    Empty,

    Def(Name, SubTree<Name>),
    Assign(Name, SubTree<Name>),

    Ident(Name),

    IntLit(i64),
    StrLit(String),


    Add(SubTree<Name>, SubTree<Name>),
    Sub(SubTree<Name>, SubTree<Name>),
    Mul(SubTree<Name>, SubTree<Name>),
    Div(SubTree<Name>, SubTree<Name>),

    Eq(SubTree<Name>, SubTree<Name>),
    Neq(SubTree<Name>, SubTree<Name>),

    Func(Vec<UnboxedSubTree<Name>>, Rc<UnboxedSubTree<Name>>),
    Call(SubTree<Name>, Vec<UnboxedSubTree<Name>>),

    Block(Vec<UnboxedSubTree<Name>>, SubTree<Name>),
    Tuple(Vec<UnboxedSubTree<Name>>),

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



#[derive(Debug, PartialEq, Clone)]
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

    pub fn name(&self) -> Option<&Name> {
        match self.tree {
            TreeType::Def(ref name, _) => Some(name),
            TreeType::Assign(ref name, _) => Some(name),
            TreeType::Ident(ref name) => Some(name),

            _ => None
        }
    }

    pub fn ident_name(&self) -> Option<&Name> {
        match self.tree {
            TreeType::Ident(ref name) => Some(name),
            _ => None
        }
    }

    pub fn for_each<'a, F: FnMut(&'a Tree<Name>) -> ()>(&'a self, mut f: F) {
        self.for_each_ref(&mut f);
    }

    // helper for for_each
    fn for_each_ref<'a, F: FnMut(&'a Tree<Name>) -> ()>(&'a self, f: &mut F) {
        f(self);
        match self.tree {
            TreeType::Def(_, ref rhs) => rhs.for_each_ref(f),
            TreeType::Assign(_, ref rhs) => rhs.for_each_ref(f),

            TreeType::Add(ref lhs, ref rhs) => { lhs.for_each_ref(f); rhs.for_each_ref(f) },
            TreeType::Sub(ref lhs, ref rhs) => { lhs.for_each_ref(f); rhs.for_each_ref(f) },
            TreeType::Mul(ref lhs, ref rhs) => { lhs.for_each_ref(f); rhs.for_each_ref(f) }, 
            TreeType::Div(ref lhs, ref rhs) => { lhs.for_each_ref(f); rhs.for_each_ref(f) }, 

            TreeType::Eq(ref lhs, ref rhs) => { lhs.for_each_ref(f); rhs.for_each_ref(f) }, 
            TreeType::Neq(ref lhs, ref rhs) => { lhs.for_each_ref(f); rhs.for_each_ref(f) }, 

            TreeType::Func(ref bind, ref body) => { for b in bind { b.for_each_ref(f); } body.for_each_ref(f) },
            TreeType::Call(ref func, ref args) => { func.for_each_ref(f); for a in args { a.for_each_ref(f); } }, 

            TreeType::Block(ref stats, ref expr) => { for s in stats { s.for_each_ref(f); } expr.for_each_ref(f) },
            TreeType::Tuple(ref elems) => for e in elems { e.for_each_ref(f); },

            TreeType::If(ref cond, ref thenp, ref elsep) => { cond.for_each_ref(f); thenp.for_each_ref(f); elsep.for_each_ref(f) }, 
            TreeType::While(ref cond, ref body) => { cond.for_each_ref(f); body.for_each_ref(f) }, 

            TreeType::Empty | TreeType::Ident(_) | TreeType::IntLit(_) | TreeType::StrLit(_) | TreeType::Error(_) => (),
        }
    }
}


impl<Name: Clone> Tree<Name> {
    pub fn transform<F: FnMut(TreeType<Name>) -> TreeType<Name>>(self, mut f: F) -> Tree<Name> {
        self.transform_ref(&mut f)
    }

    // helper for transform
    fn transform_ref<F: FnMut(TreeType<Name>) -> TreeType<Name>>(self, mut f: &mut F) -> Tree<Name> {
        let pos = self.pos.clone();
        match f(self.tree) {
            TreeType::Def(name, rhs) => TreeType::Def(name, rhs.map_in_place(|t| t.transform_ref(f))),
            TreeType::Assign(name, rhs) => TreeType::Assign(name, rhs.map_in_place(|t| t.transform_ref(f))),

            TreeType::Add(lhs, rhs) => TreeType::Add(lhs.map_in_place(|t| t.transform_ref(f)), rhs.map_in_place(|t| t.transform_ref(f))),
            TreeType::Sub(lhs, rhs) => TreeType::Sub(lhs.map_in_place(|t| t.transform_ref(f)), rhs.map_in_place(|t| t.transform_ref(f))),
            TreeType::Mul(lhs, rhs) => TreeType::Mul(lhs.map_in_place(|t| t.transform_ref(f)), rhs.map_in_place(|t| t.transform_ref(f))), 
            TreeType::Div(lhs, rhs) => TreeType::Div(lhs.map_in_place(|t| t.transform_ref(f)), rhs.map_in_place(|t| t.transform_ref(f))), 

            TreeType::Eq(lhs, rhs) => TreeType::Eq(lhs.map_in_place(|t| t.transform_ref(f)), rhs.map_in_place(|t| t.transform_ref(f))), 
            TreeType::Neq(lhs, rhs) => TreeType::Neq(lhs.map_in_place(|t| t.transform_ref(f)), rhs.map_in_place(|t| t.transform_ref(f))), 

            TreeType::Func(bind, body) => TreeType::Func(bind.into_iter().map(|t| t.transform_ref(f)).collect(), body.map_in_place(|t| t.transform_ref(f))),
            TreeType::Call(func, args) => TreeType::Call(func.map_in_place(|t| t.transform_ref(f)), args.map_in_place(|t| t.transform_ref(f))),

            TreeType::Block(stats, expr) => TreeType::Block(stats.map_in_place(|t| t.transform_ref(f)), expr.map_in_place(|t| t.transform_ref(f))),
            TreeType::Tuple(elems) => TreeType::Tuple(elems.into_iter().map(|t| t.transform_ref(f)).collect()),

            TreeType::If(cond, thenp, elsep) => TreeType::If(cond.map_in_place(|t| t.transform_ref(f)), thenp.map_in_place(|t| t.transform_ref(f)), elsep.map_in_place(|t| t.transform_ref(f))), 
            TreeType::While(cond, body) => TreeType::While(cond.map_in_place(|t| t.transform_ref(f)), body.map_in_place(|t| t.transform_ref(f))), 

            t => t,
        }.with_pos(pos)
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
            TreeType::StrLit(ref val) => write!(f, "\"{}\"", val),

            TreeType::Add(ref lhs, ref rhs) => write!(f, "({} + {})", lhs, rhs),
            TreeType::Sub(ref lhs, ref rhs) => write!(f, "({} - {})", lhs, rhs),
            TreeType::Mul(ref lhs, ref rhs) => write!(f, "{} * {}", lhs, rhs),
            TreeType::Div(ref lhs, ref rhs) => write!(f, "{} / {}", lhs, rhs),

            TreeType::Eq(ref lhs, ref rhs) => write!(f, "{} == {}", lhs, rhs),
            TreeType::Neq(ref lhs, ref rhs) => write!(f, "{} != {}", lhs, rhs),

            TreeType::Func(ref bind, ref body) => {
                let mut r = write!(f, "(");
                for b in bind {
                    r = r.and_then(|_| write!(f, "{}, ", b));
                }
                r.and_then(|_| write!(f, ") => {}", body))
            },
            TreeType::Call(ref func, ref args) => {
                let mut r = write!(f, "{}(", func);
                for a in args {
                    r = r.and_then(|_| write!(f, "{}, ", a));
                }
                r.and_then(|_| write!(f, ")"))
            },

            TreeType::Block(ref stats, ref expr) => {
                let mut r = write!(f, "{{\n");
                for s in stats {
                    r = r.and_then(|_| write!(f, "{}\n", s));
                }
                r.and_then(|_| write!(f, "{}\n}}", expr))
            },

            TreeType::Tuple(ref elems) => {
                let mut r = write!(f, "(");
                for e in elems {
                    r = r.and_then(|_| write!(f, "{}, ", e));
                }
                r.and_then(|_| write!(f, ")"))
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