
mod transform;
mod tokens;
mod position;
mod tokenizer;
mod name_analysis;
mod parser;
mod symbol;
mod ast;
mod context;
mod utils;
mod value;
mod eval;

use tokenizer::*;
use name_analysis::*;
use parser::*;
use eval::*;
use value::*;
use utils::*;
use context::*;

use std::time::{Instant};


fn print_val(v: Value) -> Result<Value, ErrorKind> {
	println!("{}", &v.to_str());
	Ok(Value::Unit)
}

/*fn scan_val(v: Value) -> Result<Value, ErrorKind> {
	loop {
		println!("{}", v);
		let mut input = String::new();
		match io::stdin().read_line(&mut input) {
			Ok(_) => { input.pop(); return Ok(Value::Str(input.trim().to_string())) },
			_ => {}
		}
	}
}

fn map(a: Value, b: Value) -> Result<Value, ErrorKind> {
	let list = a.to_list();
	let func = b;
	let mapped = list.into_iter().map(|v| func.call(v));
	Ok(Value::List(mapped.map(|x| x.unwrap()).collect()))
}*/

fn modulus(a: Value, b: Value) -> Result<Value, ErrorKind> {
	let a = a.to_num()?;
	let b = b.to_num()?;
	Ok(Value::Num(a % b))
}




fn main() {
	let input = r#"{
			let primes = [2]

			let is_prime = (num) => {
				let prime = 1 
				for p : primes {
					if mod(num, p) {
					} else {
						prime = 0
					}
				}
				prime
			}

			let i = 3
			while (i - 2500) {
				if is_prime(i) {
					primes = primes + i
				}
				i = i + 1
			}
			primes
		}"#;


	let mut ast = parse(&mut Tokenizer::tokenize(input));
	println!("{}", ast);

	let mut ctx = Context::new();
	let mut env = Env::new();

	*env.get(&ctx.decl("mod".to_owned()).unwrap()) = Value::from_func_2(modulus);
	*env.get(&ctx.decl("print".to_owned()).unwrap()) = Value::from_func_1(print_val);

	ast = ast.transform(NameAnalysis::new(), &mut ctx).unwrap();

	let now = Instant::now();

	let res = eval(&ast, &mut env);

	let duration = now.elapsed();
	println!("done in {}ms", (duration.as_secs() * 1000) as u32 + duration.subsec_nanos() / 1000000);
	println!("{:?}", res);
}