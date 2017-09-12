

mod tokens;
mod position;
mod tokenizer;
mod parser;
mod tree;
mod utils;
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
	let string = v.to_str(); // is list
	println!("{}", &string[1..string.len() - 1]);
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

fn map(v: Value) -> Value {
	let args = v.to_list();
	assert_eq!(args.len(), 2);

	let mut args = args.into_iter();
	let list = args.next().unwrap().to_list();
	let func = args.next().unwrap();
	println!("map {:?} {:?}", list, func);
	let mapped = list.into_iter().map(|v| func.call(v));
	Value::List(mapped.collect())
}

fn main() {
	let input = r#"{
			let zero = () => 0
			let one = t => 0 + t
			let two = (t, u) => t * 1 + u 

			let x = 0
			let y = 9
			y = x = 1 * y + 2
			print(x)
			print(zero())
			print(one(1))
			print(two(1, 2))
		}"#;

	/*let input = r#"{
			let count = 10000000
			while count {
				count = count - 1
			}
		}"#;*/




	let ast = parse(&mut Tokenizer::tokenize(input));
	println!("{}", ast);

	let mut env = Env::new();
	env.decl("print", Value::Func(Rc::new(print_val)));
	env.decl("map", Value::Func(Rc::new(map)));
	env.decl("scan", Value::Func(Rc::new(scan_val)));

	let now = Instant::now();

	eval(&ast, &mut env);

	let duration = now.elapsed();
	println!("done in {}ms", (duration.as_secs() * 1000) as u32 + duration.subsec_nanos() / 1000000);
}