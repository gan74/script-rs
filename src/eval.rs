
use std::collections::{HashMap};
use std::rc::{Rc};

use tree::{Tree};
use value::{Value};

pub struct Env {
	symbols: Vec<HashMap<String, Value>>
}

impl Env {
	pub fn new() -> Env {
		Env {
			symbols: vec![HashMap::new()]
		}
	}

	pub fn decl(&mut self, name: &str, val: Value) {
		for h in self.symbols.iter().rev() {
			if let Some(_) = h.get(name) {
				panic!("{:?} has already been declared", name);
		    }
		}
		self.symbols.last_mut().unwrap().insert(name.to_owned(), val);
	}

	pub fn get(&mut self, name: &str) -> &mut Value {
		for mut h in &mut self.symbols.iter_mut().rev() {
			if let Some(v) = h.get_mut(name) {
			    return v;
			}
		}
		panic!("{:?} has not been declared", name);
	}

	fn push(&mut self) {
		self.symbols.push(HashMap::new());
	}

	fn pop(&mut self) {
		self.symbols.pop();
	}
}






pub fn eval(stmt: &Tree, env: &mut Env) -> Value {
	match stmt {
		&Tree::Unit => Value::Unit,

		&Tree::Decl(ref name, ref rhs) => { let rhs = eval(rhs, env); env.decl(name, rhs); Value::Unit },
		&Tree::Block(ref stmts) => eval_block(stmts, env),
		&Tree::If(ref cond, ref th, ref el) => if eval(cond, env).to_bool() { eval(th, env) } else { eval(el, env) },
		&Tree::While(ref cond, ref body) => { while eval(cond, env).to_bool() { eval(body, env); } Value::Unit },
		&Tree::For(ref name, ref lst, ref body) => eval_for(name, lst, body, env),

		&Tree::Assign(ref name, ref rhs) => { let rhs = eval(rhs, env); *env.get(name) = rhs.clone(); rhs },
		&Tree::Add(ref lhs, ref rhs) => { eval(lhs, env) + eval(rhs, env) },
		&Tree::Sub(ref lhs, ref rhs) => { eval(lhs, env) - eval(rhs, env) },

		&Tree::Call(ref f, ref a) => eval(f, env).call(eval(a, env)),
		&Tree::Func(ref a, ref b) => Value::Func(eval_func(a.clone(), b.clone())),

		&Tree::Ident(ref name) => env.get(name).clone(),
		&Tree::StrLit(ref val) => Value::Str(val.clone()),
		&Tree::NumLit(val) => Value::Num(val),

		&Tree::ListLit(ref lst) => Value::List(lst.iter().map(|e| eval(e, env)).collect())
	}
}

fn eval_block(block: &[Tree], env: &mut Env) -> Value {
	env.push(); 
	let mut ret = Value::Unit;
	for s in block { 
	 	ret = eval(s, env); 
	} 
	env.pop();
	ret
}

fn eval_for(name: &str, lst: &Tree, body: &Tree, env: &mut Env) -> Value {
	env.push(); 
	env.decl(name, Value::Unit);
	for e in eval(lst, env).to_list() { 
		*env.get(name) = e; 
		eval(body, env);
	}
	env.pop();
	Value::Unit
}

fn eval_func(args: Vec<String>, body: Rc<Tree>) -> Rc<Fn(Value) -> Value> {	
	match args.len() {

		/*0 => Rc::new(move |params| { 
			match params {
				Value::Unit => (),
				_ => panic!("Invalid number of arguments, expected none")
			}
			let mut env = Env::new();
			eval(&*body, &mut env)
		}),

		1 => Rc::new(move |params| { 
			let mut env = Env::new();
			env.decl(&args[0], params);
			eval(&*body, &mut env)
		}),*/

		_ => Rc::new(move |params| {
			let params = params.to_list();
			if params.len() != args.len() { 
				panic!("Invalid number of arguments, expected {}, got {}", args.len(), params.len()); 
			}
			let mut env = Env::new();
			for (i, p) in params.into_iter().enumerate() {
				env.decl(&args[i], p);
			}
			eval(&*body, &mut env)
		})
	}
}










