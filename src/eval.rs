
use std::collections::{HashMap};
use std::rc::{Rc};

use ast::{Statement, Expression};
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
		panic!("{:?} was not declared", name);
	}

	fn push(&mut self) {
		self.symbols.push(HashMap::new());
	}

	fn pop(&mut self) {
		self.symbols.pop();
	}
}








pub fn eval_stmt(stmt: &Statement, env: &mut Env) -> Value {
	match stmt {
		&Statement::Decl(ref name, ref rhs) => { let rhs = eval_expr(rhs, env); env.decl(name, rhs); Value::Unit },
		&Statement::Block(ref stmts) => eval_block(stmts, env),
		&Statement::Expr(ref expr) => eval_expr(expr, env),
		&Statement::If(ref cond, ref th, ref el) => if eval_expr(cond, env).to_bool() { eval_stmt(th, env) } else { eval_stmt(el, env) },
		&Statement::While(ref cond, ref body) => { while eval_expr(cond, env).to_bool() { eval_stmt(body, env); } Value::Unit },
		&Statement::For(ref name, ref lst, ref body) => eval_for(name, lst, body, env),
		&Statement::Unit => Value::Unit
	}
}

fn eval_block(block: &[Statement], env: &mut Env) -> Value {
	env.push(); 
	let mut ret = Value::Unit;
	for s in block { 
	 	ret = eval_stmt(s, env); 
	} 
	env.pop();
	ret
}

fn eval_for(name: &str, lst: &Expression, body: &Statement, env: &mut Env) -> Value {
	env.push(); 
	env.decl(name, Value::Unit);
	for e in eval_expr(lst, env).to_list() { 
		*env.get(name) = e; 
		eval_stmt(body, env);
	}
	env.pop();
	Value::Unit
}





pub fn eval_expr(expr: &Expression, env: &mut Env) -> Value {
	match expr {
		&Expression::Assign(ref name, ref rhs) => { let rhs = eval_expr(rhs, env); *env.get(name) = rhs.clone(); rhs },
		&Expression::Add(ref lhs, ref rhs) => { eval_expr(lhs, env) + eval_expr(rhs, env) },
		&Expression::Sub(ref lhs, ref rhs) => { eval_expr(lhs, env) - eval_expr(rhs, env) },

		&Expression::Call(ref f, ref a) => eval_expr(f, env).call(eval_expr(a, env)),
		&Expression::Func(ref a, ref b) => Value::Func(eval_func(a.clone(), b.clone())),

		&Expression::Ident(ref name) => env.get(name).clone(),
		&Expression::StrLit(ref val) => Value::Str(val.clone()),
		&Expression::NumLit(val) => Value::Num(val),

		&Expression::ListLit(ref lst) => Value::List(lst.iter().map(|e| eval_expr(e, env)).collect())

	}
}

fn eval_func(args: Vec<String>, body: Rc<Statement>) -> Rc<Fn(Value) -> Value> {	
	match args.len() {

		0 => Rc::new(move |params| { 
			match params {
				Value::Unit => (),
				_ => panic!("Invalid number of arguments, expected none")
			}
			let mut env = Env::new();
			eval_stmt(&*body, &mut env)
		}),

		1 => Rc::new(move |params| { 
			let mut env = Env::new();
			env.decl(&args[0], params);
			eval_stmt(&*body, &mut env)
		}),

		_ => Rc::new(move |params| {
			let params = params.to_list();
			if params.len() != args.len() { 
				panic!("Invalid number of arguments, expected {}, got {}", args.len(), params.len()); 
			}
			let mut env = Env::new();
			for (i, p) in params.into_iter().enumerate() {
				env.decl(&args[i], p);
			}
			eval_stmt(&*body, &mut env)
		})
	}
}










