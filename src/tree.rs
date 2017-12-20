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
        macro_rules! fe { ($x:expr) => ($x.for_each_ref(f)); }
        f(self);
        match self.tree {
            TreeType::Def(_, ref rhs) => fe!(rhs),
            TreeType::Assign(_, ref rhs) => fe!(rhs),

            TreeType::Add(ref lhs, ref rhs) => { fe!(lhs); fe!(rhs) },
            TreeType::Sub(ref lhs, ref rhs) => { fe!(lhs); fe!(rhs) },
            TreeType::Mul(ref lhs, ref rhs) => { fe!(lhs); fe!(rhs) }, 
            TreeType::Div(ref lhs, ref rhs) => { fe!(lhs); fe!(rhs) }, 

            TreeType::Eq(ref lhs, ref rhs) => { fe!(lhs); fe!(rhs) }, 
            TreeType::Neq(ref lhs, ref rhs) => { fe!(lhs); fe!(rhs) }, 

            TreeType::Func(ref bind, ref body) => { for b in bind { fe!(b); } fe!(body) },
            TreeType::Call(ref func, ref args) => { fe!(func); for a in args { fe!(a); } }, 

            TreeType::Block(ref stats, ref expr) => { for s in stats { fe!(s); } fe!(expr) },
            TreeType::Tuple(ref elems) => for e in elems { fe!(e); },

            TreeType::If(ref cond, ref thenp, ref elsep) => { fe!(cond); fe!(thenp); fe!(elsep) }, 
            TreeType::While(ref cond, ref body) => { fe!(cond); fe!(body) }, 

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
        macro_rules! tr { ($x:expr) => ($x.map_in_place(|t| t.transform_ref(f))); }
        let pos = self.pos.clone();
        match f(self.tree) {
            TreeType::Def(name, rhs) => TreeType::Def(name, tr!(rhs)),
            TreeType::Assign(name, rhs) => TreeType::Assign(name, tr!(rhs)),

            TreeType::Add(lhs, rhs) => TreeType::Add(tr!(lhs), tr!(rhs)),
            TreeType::Sub(lhs, rhs) => TreeType::Sub(tr!(lhs), tr!(rhs)),
            TreeType::Mul(lhs, rhs) => TreeType::Mul(tr!(lhs), tr!(rhs)), 
            TreeType::Div(lhs, rhs) => TreeType::Div(tr!(lhs), tr!(rhs)), 

            TreeType::Eq(lhs, rhs) => TreeType::Eq(tr!(lhs), tr!(rhs)), 
            TreeType::Neq(lhs, rhs) => TreeType::Neq(tr!(lhs), tr!(rhs)), 

            TreeType::Func(bind, body) => TreeType::Func(tr!(bind), tr!(body)),
            TreeType::Call(func, args) => TreeType::Call(tr!(func), tr!(args)),

            TreeType::Block(stats, expr) => TreeType::Block(tr!(stats), tr!(expr)),
            TreeType::Tuple(elems) => TreeType::Tuple(tr!(elems)),

            TreeType::If(cond, thenp, elsep) => TreeType::If(tr!(cond), tr!(thenp), tr!(elsep)), 
            TreeType::While(cond, body) => TreeType::While(tr!(cond), tr!(body)), 

            
            t @ TreeType::Empty | t @ TreeType::Ident(_) | t @ TreeType::IntLit(_) | t @ TreeType::StrLit(_) | t @ TreeType::Error(_) => t,
        }.with_pos(pos)
    }
}

/*impl<Name: Clone> Tree<Name> {
    pub fn rename<N, F: FnMut(TreeType<Name>) -> TreeType<N>>(self, mut f: F) -> Tree<N> {
        self.rename_ref(&mut f)
    }

    fn rename_ref<N, F: FnMut(TreeType<Name>) -> TreeType<N>>(self, mut f: &mut F) -> Tree<N> {
        let pos = self.pos.clone();
        match self.tree {
            TreeType::Add(lhs, rhs) => TreeType::Add(Box::new(lhs.rename_ref(f)), Box::new(rhs.rename_ref(f))),
            TreeType::Sub(lhs, rhs) => TreeType::Sub(Box::new(lhs.rename_ref(f)), Box::new(rhs.rename_ref(f))),
            TreeType::Mul(lhs, rhs) => TreeType::Mul(Box::new(lhs.rename_ref(f)), Box::new(rhs.rename_ref(f))), 
            TreeType::Div(lhs, rhs) => TreeType::Div(Box::new(lhs.rename_ref(f)), Box::new(rhs.rename_ref(f))), 

            TreeType::Eq(lhs, rhs) => TreeType::Eq(Box::new(lhs.rename_ref(f)), Box::new(rhs.rename_ref(f))), 
            TreeType::Neq(lhs, rhs) => TreeType::Neq(Box::new(lhs.rename_ref(f)), Box::new(rhs.rename_ref(f))), 

            TreeType::Func(bind, body) => TreeType::Func(bind.into_iter().map(|t| t.rename_ref(f)).collect(), Rc::new(Rc::try_unwrap(body).ok().unwrap().rename_ref(f))),
            TreeType::Call(func, args) => TreeType::Call(Box::new(func.rename_ref(f)), args.into_iter().map(|t| t.rename_ref(f)).collect()),

            TreeType::Block(stats, expr) => TreeType::Block(stats.into_iter().map(|t| t.rename_ref(f)).collect(), Box::new(expr.rename_ref(f))),
            TreeType::Tuple(elems) => TreeType::Tuple(elems.into_iter().map(|t| t.rename_ref(f)).collect()),

            TreeType::If(cond, thenp, elsep) => TreeType::If(Box::new(cond.rename_ref(f)), Box::new(thenp.rename_ref(f)), Box::new(elsep.rename_ref(f))), 
            TreeType::While(cond, body) => TreeType::While(Box::new(cond.rename_ref(f)), Box::new(body.rename_ref(f))),

            t => panic!("???"),
        }.with_pos(pos)
    }
}*/





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