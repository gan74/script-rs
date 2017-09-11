use position::{Position};
use std::fmt::{Display};

use std::process::*;

pub fn fatal(msg: &str, pos: &Position) -> ! {
	println!("at {}: {}", pos, msg);
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