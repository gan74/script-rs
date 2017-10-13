
use std::fmt::{Debug, Display, Formatter};

use std::process::*;
use std::fmt;

use position::*;
use tokens::*;


pub enum ErrorKind {
	Boxed(Box<Error>),
	Generic(String),
	NotDeclared(String),
	AlreadyDeclared(String),
	WrongArgCount(usize, usize)
}

pub struct Error {
	tpe: ErrorKind,
	pos: Position
}

impl ErrorKind {
	pub fn with_position(self, pos: &Position) -> Error {
		match self {
			ErrorKind::Boxed(err) => *err,
			tpe => 
				Error {
				tpe: tpe,
				pos: pos.clone()
			}
		}
	}
}

impl Debug for ErrorKind {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self {
			&ErrorKind::Boxed(ref e) => write!(f, "{:?}", *e), 
			&ErrorKind::Generic(ref s) => write!(f, "{}", s), 
			&ErrorKind::NotDeclared(ref s) => write!(f, "{} was not declared", s), 
			&ErrorKind::AlreadyDeclared(ref s) => write!(f, "{} has already been declared", s), 
			&ErrorKind::WrongArgCount(e, g) => write!(f, "expected {} arguments, got {}", e, g)
		}
	}
}

impl Debug for Error {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "{:?}, at {}", self.tpe, self.pos)
	}
}








pub fn fatal(msg: &str) -> ! {
	println!("{}", msg);
	exit(1)
}

pub fn fatal_tk(msg: &str, tk: Option<Token>) -> ! {
	if let Some(tk) = tk {
		println!("{}, got '{}', at {}", msg, tk, tk.position());
	} else {
		println!("{}, got EOF", msg);
	}
	exit(1)
}

pub fn fatal_pos(msg: &str, pos: Position) -> ! {
	println!("{}, at {}", msg, pos);
	exit(1)
}




pub fn concat<T: Display>(lst: &[T]) -> String {
	let mut string = lst.iter().fold(String::new(), |s, i| s + &format!("{}", i) + ", ");
	string.pop(); string.pop();
	string
}

pub fn box_<T>(t: T) -> Box<T> {
	Box::new(t)
}


