
use std::rc::{Rc};

use ast::*;
use value::*;
use symbol::*;
use position::*;
use utils::*;


pub struct Env {
	symbols: Vec<Value>
}

impl Env {
	pub fn new() -> Env {
		Env {
			symbols: Vec::new()
		}
	}

	pub fn get(&mut self, re: &SymbolRef) -> &mut Value {
		let id = re.id();
		while self.symbols.len() <= id {
			self.symbols.push(Value::Unit)
		}
		self.symbols.get_mut(id).unwrap()
	}
}



fn with_pos<T>(r: Result<T, ErrorKind>, pos: &Position) -> Result<T, Error> {
	r.map_err(|e| e.with_position(pos))
}

pub fn eval(stmt: &Tree, env: &mut Env) -> Result<Value, Error> {
	match stmt {
		&Tree::Unit          (_) => Ok(Value::Unit),

		&Tree::Decl          (_, ref re, ref rhs) => { let rhs = eval(rhs, env)?; *env.get(re) = rhs.clone(); Ok(Value::Unit) }, //{ let rhs = eval(rhs, env)?; with_pos(env.decl(name, rhs), p)?; Ok(Value::Unit) },
		&Tree::Assign        (_, ref re, ref rhs) => { let rhs = eval(rhs, env)?; *env.get(re) = rhs.clone(); Ok(rhs) },
		&Tree::Ident         (_, ref re) => Ok(env.get(re).clone()),
		&Tree::For           (_, ref re, ref lst, ref body) => eval_for(re, lst, body, env),
		&Tree::Func          (_, ref a, ref b) => Ok(Value::Func(eval_func(a.clone(), b.clone()))),

		&Tree::Block         (_, ref stmts) => eval_block(stmts, env),
		&Tree::If            (ref p, ref cond, ref th, ref el) => if with_pos(eval(cond, env)?.to_bool(), p)? { eval(th, env) } else { eval(el, env) },
		&Tree::While         (ref p, ref cond, ref body) => { while with_pos(eval(cond, env)?.to_bool(), p)? { eval(body, env)?; } Ok(Value::Unit) },
		&Tree::Add           (ref p, ref lhs, ref rhs) => with_pos(eval(lhs, env)? + eval(rhs, env)?, p),
		&Tree::Sub           (ref p, ref lhs, ref rhs) => with_pos(eval(lhs, env)? - eval(rhs, env)?, p),
		&Tree::Mul           (ref p, ref lhs, ref rhs) => with_pos(eval(lhs, env)? * eval(rhs, env)?, p),
		&Tree::Div           (ref p, ref lhs, ref rhs) => with_pos(eval(lhs, env)? / eval(rhs, env)?, p),
		&Tree::Call          (ref p, ref f, ref a) => { let args = with_pos(eval(a, env)?.to_list(), p)?; with_pos(eval(f, env)?.call(env, args), p) },
		&Tree::StrLit        (_, ref val) => Ok(Value::Str(val.clone())),
		&Tree::NumLit        (_, val) => Ok(Value::Num(val)),
		&Tree::ListLit       (_, ref lst) => Ok(Value::List(lst.iter().map(|e| eval(e, env)).collect::<Result<Vec<Value>, Error>>()?)),

		t => panic!("unexpected ast node {:?}", t)
	}
}


fn eval_block(block: &[Tree], env: &mut Env) -> Result<Value, Error> {
	let mut ret = Value::Unit;
	for s in block { 
	 	ret = eval(s, env)?; 
	} 
	Ok(ret)
}

fn eval_for(re: &SymbolRef, lst: &Tree, body: &Tree, env: &mut Env) -> Result<Value, Error> {
	let pos = lst.position();
	for e in with_pos(eval(lst, env)?.to_list(), &pos)? { 
		*env.get(re) = e; 
		eval(body, env)?;
	}
	Ok(Value::Unit)
}

fn eval_func(args: Vec<SymbolRef>, body: Rc<Tree>) -> FuncValue {
	let arg_count = args.len();
	FuncValue {
		args: arg_count,

		func: Rc::new(move |env: &mut Env, params| {
			assert_eq!(params.len(), arg_count);
			for (i, p) in params.into_iter().enumerate() {
				*env.get(&args[i]) = p;
			}
			eval(&*body, env).map_err(|e| ErrorKind::Boxed(box_(e)))
		})
	}
}










