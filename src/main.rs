
mod tokens;
mod position;
mod tokenizer;
mod parser;
mod ast;
mod fatal;
mod value;
mod eval;

use tokenizer::*;
use parser::*;
use eval::*;
use value::*;

use std::io;
use std::rc::{Rc};
use std::time::{Instant};

fn print_val(v: Value) -> Value {
	println!("{}", v);
	Value::Unit
}

fn scan_val(v: Value) -> Value {
	loop {
		println!("{}", v);
		let mut input = String::new();
		match io::stdin().read_line(&mut input) {
			Ok(_) => { input.pop(); return Value::Str(input.trim().to_string()) },
			_ => {}
		}
	}
}

fn main() {
	//
	let input = r#"{
			let fun = (t, u) => t + u
			let lst = 7, 2, 5, 8, 6, 4, 9, 3, 2
			for i : lst {
				print(fun(i, 2))
			}
		}"#;

	

	//println!("{:?}", Tokenizer::tokenize(input).collect::<Vec<_>>());

	let ast = parse(&mut Tokenizer::tokenize(input));
	println!("{}", ast);

	let mut env = Env::new();
	env.decl(&String::from("print"), Value::Func(Rc::new(print_val)));
	env.decl(&String::from("scan"), Value::Func(Rc::new(scan_val)));

	let now = Instant::now();

	eval_stmt(&ast, &mut env);

	let duration = now.elapsed();
	println!("done in {}ms", (duration.as_secs() * 1000) as u32 + duration.subsec_nanos() / 1000000);
}