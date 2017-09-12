use std::fmt::{Display};

use std::process::*;

use position::{Position};
use tokens::{Token};


/*pub enum Either<A, B> {
	Left(A),
	Right(B)
}*/


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