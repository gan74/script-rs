
use std::fmt::{Display, Result, Formatter};
use std::rc::{Rc};

pub type Symbol = String;
pub type Literal = String;

#[derive(Debug)]
pub enum Statement {
	Decl(Symbol, Expression),
	Expr(Expression),
	Block(Vec<Statement>),

	If(Expression, Box<Statement>, Box<Statement>),
	While(Expression, Box<Statement>),
	For(Symbol, Expression, Box<Statement>),

	Unit
}

#[derive(Debug)]
pub enum Expression {
	Assign(Symbol, Box<Expression>),
	Add(Box<Expression>, Box<Expression>),
	Sub(Box<Expression>, Box<Expression>),
	
	Call(Box<Expression>, Box<Expression>),

	Func(Vec<Symbol>, Rc<Statement>),

	Ident(Symbol),

	NumLit(f64),
	StrLit(Literal),

	ListLit(Vec<Expression>),

}



impl Display for Statement {
	fn fmt(&self, f: &mut Formatter) -> Result {
		match self {
			&Statement::Decl(ref s, ref e) => write!(f, "let {} = {}", s, e),
			&Statement::Expr(ref e) => write!(f, "{}", e),
			&Statement::If(ref c, ref t, ref e) => write!(f, "if({}) {} else {}", c, t, e),
			&Statement::While(ref c, ref b) => write!(f, "while({}) {}", c, b),
			&Statement::For(ref n, ref l, ref b) => write!(f, "for({} : {}) {}", n, l, b),
			&Statement::Unit => Ok(()),
			&Statement::Block(ref s) => {
				let mut r = write!(f, "{{\n");
				for s in s {
					r = r.and_then(|_| write!(f, "{}\n", s));
				}
				r.and_then(|_| write!(f, "}}"))
			}
		}
	}
}


impl Display for Expression {
	fn fmt(&self, f: &mut Formatter) -> Result {
		match self {
			&Expression::Assign(ref l, ref r) => write!(f, "{} = {}", l, r),
			&Expression::Add(ref l, ref r) => write!(f, "({} + {})", l, r),
			&Expression::Sub(ref l, ref r) => write!(f, "({} - {})", l, r),
			&Expression::Call(ref o, ref a) => write!(f, "{}({})", o, a),
			&Expression::Func(ref a, ref b) => write!(f, "({}) => {}", concat(a), b),
			&Expression::Ident(ref name) => write!(f, "{}", name),
			&Expression::StrLit(ref val) => write!(f, "\"{}\"", val),
			&Expression::NumLit(ref val) => write!(f, "{}", val),
			&Expression::ListLit(ref lst) => write!(f, "[{}]", concat(lst))
		}
	}
}

fn concat<T: Display>(lst: &[T]) -> String {
	let mut string = lst.iter().fold(String::new(), |s, i| s + &format!("{}", i) + ", ");
	string.pop(); string.pop();
	string
}