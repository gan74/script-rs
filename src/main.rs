

mod tokens;
mod position;
mod tokenizer;
mod parser;
mod ast;
mod utils;
mod value;
mod eval;

use tokenizer::*;
use parser::*;
use eval::*;
use value::*;
use utils::*;

use std::io;
use std::time::{Instant};



fn print_val(v: Value) -> Result<Value, ErrorKind> {
	let string = v.to_str(); // is list
	println!("{}", &string[1..string.len() - 1]);
	Ok(Value::Unit)
}

fn scan_val(v: Value) -> Result<Value, ErrorKind> {
	loop {
		println!("{}", v);
		let mut input = String::new();
		match io::stdin().read_line(&mut input) {
			Ok(_) => { input.pop(); return Ok(Value::Str(input.trim().to_string())) },
			_ => {}
		}
	}
}

fn modulo(a: Value, b: Value) -> Result<Value, ErrorKind> {
	let a = a.to_num()?;
	let b = b.to_num()?;
	Ok(Value::Num(a % b))
}

fn map(a: Value, b: Value) -> Result<Value, ErrorKind> {
	let list = a.to_list();
	let func = b;
	let mapped = list.into_iter().map(|v| func.call(v));
	Ok(Value::List(mapped.map(|x| x.unwrap()).collect()))
}

fn main() {
	let input = r#"{
			let primes = [2]
			let i = 3
			while (i - 10000) {
				let is_prime = 1 
				for p : primes {
					if mod(i, p) {
					} else {
						is_prime = 0
					}
				}
				if is_prime {
					primes = primes + i
				}
				i = i + 1
			}
			primes
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
	env.decl("print", Value::from_func_1(print_val)).unwrap();
	env.decl("map", Value::from_func_2(map)).unwrap();
	env.decl("mod", Value::from_func_2(modulo)).unwrap();
	env.decl("scan", Value::from_func_1(scan_val)).unwrap();

	let now = Instant::now();

	let res = eval(&ast, &mut env);

	let duration = now.elapsed();
	println!("done in {}ms", (duration.as_secs() * 1000) as u32 + duration.subsec_nanos() / 1000000);
	println!("{:?}", res);
}