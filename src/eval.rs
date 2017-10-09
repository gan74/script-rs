
use std::collections::{HashMap};
use std::rc::{Rc};

use ast::{Tree};
use value::{FuncValue, Value};
use position::{Position};

use utils::{Error, ErrorKind, box_};


pub struct Env {
	symbols: Vec<HashMap<String, Value>>
}

impl Env {
	pub fn new() -> Env {
		Env {
			symbols: vec![HashMap::new()]
		}
	}

	pub fn decl(&mut self, name: &str, val: Value) -> Result<(), ErrorKind> {
		for h in self.symbols.iter().rev() {
			if let Some(_) = h.get(name) {
				return Err(ErrorKind::AlreadyDeclared(name.to_owned()));
		    }
		}
		self.symbols.last_mut().unwrap().insert(name.to_owned(), val);
		Ok(())
	}

	pub fn get(&mut self, name: &str) -> Result<&mut Value, ErrorKind> {
		for mut h in &mut self.symbols.iter_mut().rev() {
			if let Some(v) = h.get_mut(name) {
			    return Ok(v);
			}
		}
		Err(ErrorKind::Undeclared(name.to_owned()))
	}

	fn push(&mut self) {
		self.symbols.push(HashMap::new());
	}

	fn pop(&mut self) {
		self.symbols.pop();
	}
}



fn with_pos<T>(r: Result<T, ErrorKind>, pos: &Position) -> Result<T, Error> {
	r.map_err(|e| e.with_position(pos))
}

pub fn eval(stmt: &Tree, env: &mut Env) -> Result<Value, Error> {
	match stmt {
		&Tree::Unit    (_) => Ok(Value::Unit),

		&Tree::Decl    (ref p, ref name, ref rhs) => { let rhs = eval(rhs, env)?; with_pos(env.decl(name, rhs), p)?; Ok(Value::Unit) },
		&Tree::Block   (_, ref stmts) => eval_block(stmts, env),

		&Tree::If      (ref p, ref cond, ref th, ref el) => if with_pos(eval(cond, env)?.to_bool(), p)? { eval(th, env) } else { eval(el, env) },
		&Tree::While   (ref p, ref cond, ref body) => { while with_pos(eval(cond, env)?.to_bool(), p)? { eval(body, env)?; } Ok(Value::Unit) },

		&Tree::For     (_, ref name, ref lst, ref body) => eval_for(name, lst, body, env),

		&Tree::Assign  (ref p, ref name, ref rhs) => { let rhs = eval(rhs, env)?; *with_pos(env.get(name), p)? = rhs.clone(); Ok(rhs) },

		&Tree::Add     (ref p, ref lhs, ref rhs) => with_pos(eval(lhs, env)? + eval(rhs, env)?, p),
		&Tree::Sub     (ref p, ref lhs, ref rhs) => with_pos(eval(lhs, env)? - eval(rhs, env)?, p),
		&Tree::Mul     (ref p, ref lhs, ref rhs) => with_pos(eval(lhs, env)? * eval(rhs, env)?, p),
		&Tree::Div     (ref p, ref lhs, ref rhs) => with_pos(eval(lhs, env)? / eval(rhs, env)?, p),

		&Tree::Call    (ref p, ref f, ref a) => { let args = with_pos(eval(a, env)?.to_list(), p)?; with_pos(eval(f, env)?.call(args), p) },
		&Tree::Func    (_, ref a, ref b) => Ok(Value::Func(eval_func(a.clone(), b.clone()))),

		&Tree::Ident   (ref p, ref name) => with_pos(env.get(name).map(|i| i.clone()), p),
		&Tree::StrLit  (_, ref val) => Ok(Value::Str(val.clone())),
		&Tree::NumLit  (_, val) => Ok(Value::Num(val)),

		&Tree::ListLit (_, ref lst) => Ok(Value::List(lst.iter().map(|e| eval(e, env)).collect::<Result<Vec<Value>, Error>>()?))
	}
}

fn eval_block(block: &[Tree], env: &mut Env) -> Result<Value, Error> {
	env.push(); 
	let mut ret = Value::Unit;
	for s in block { 
	 	ret = eval(s, env)?; 
	} 
	env.pop();
	Ok(ret)
}

fn eval_for(name: &str, lst: &Tree, body: &Tree, env: &mut Env) -> Result<Value, Error> {
	let pos = lst.position();
	env.push(); 
	with_pos(env.decl(name, Value::Unit), &pos)?;
	for e in with_pos(eval(lst, env)?.to_list(), &pos)? { 
		*(with_pos(env.get(name), &pos)?) = e; 
		eval(body, env)?;
	}
	env.pop();
	Ok(Value::Unit)
}

fn eval_func(args: Vec<String>, body: Rc<Tree>) -> FuncValue {
	FuncValue {
		args: args.len(),

		func: Rc::new(move |params| {
			assert_eq!(params.len(), args.len());

			let mut env = Env::new();
			for (i, p) in params.into_iter().enumerate() {
				env.decl(&args[i], p)?;
			}
			eval(&*body, &mut env).map_err(|e| ErrorKind::Boxed(box_(e)))
		})
	}
}










