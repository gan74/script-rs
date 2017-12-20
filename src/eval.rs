use std::collections::HashMap;

use tree::*;
use value::*;

type Name = String;

#[derive(Debug)]
pub struct Env {
    values: HashMap<String, Value>
}

impl Env {
    pub fn new() -> Env {
        Env {
            values: HashMap::new()
        }
    }

    fn def(&mut self, name: &String, val: Value) -> Value {
        if self.values.contains_key(name) {
            panic!("\"{}\" has already been declared", name);
        }
        self.values.insert(name.clone(), val.clone());
        val
    }

    fn set(&mut self, name: &String, val: Value) -> Value {
        match self.values.get_mut(name) {
            Some(v) => *v = val.clone(),
            None => panic!("\"{}\" was not declared", name)
        }
        val
    }

    fn get(&mut self, name: &String) -> Value {
        match self.values.get(name) {
            Some(v) => v.clone(),
            None => panic!("\"{}\" was not declared", name)
        }
    }
}


pub fn eval(tree: &Tree<Name>, env: &mut Env) -> Value {
    match tree.tree {
        TreeType::Def(ref name, ref rhs) => {
            let val = eval(rhs, env);
            env.def(name, val)
        },

        TreeType::Assign(ref name, ref rhs) => {
            let val = eval(rhs, env);
            env.set(name, val)
        },

        TreeType::Ident(ref name) => env.get(name),
        TreeType::IntLit(val) => Value::Num(val as f64),
        TreeType::StrLit(ref val) => Value::Str(val.clone()),

        TreeType::Func(ref bind, ref body) => Value::Func(bind.iter().map(|b| b.ident_name().unwrap().clone()).collect(), body.clone()),

        TreeType::Add(ref lhs, ref rhs) => eval(lhs, env) + eval(rhs, env),
        TreeType::Sub(ref lhs, ref rhs) => eval(lhs, env) - eval(rhs, env),
        TreeType::Mul(ref lhs, ref rhs) => eval(lhs, env) * eval(rhs, env),
        TreeType::Div(ref lhs, ref rhs) => eval(lhs, env) / eval(rhs, env),

        TreeType::Eq(ref lhs, ref rhs) => Value::Num(if eval(lhs, env) == eval(rhs, env) { 1.0 } else { 0.0 }),
        TreeType::Neq(ref lhs, ref rhs) => Value::Num(if eval(lhs, env) != eval(rhs, env) { 1.0 } else { 0.0 }),

        TreeType::Call(ref func, ref args) => {
            let (params, body) = eval(func, env).to_func();
            if args.len() != params.len() {
                panic!("invalid number of arguments: expected {}, got {}", params.len(), args.len());
            }
            let mut inner = Env::new();
            // declare all args in the called env
            for (a, p) in args.into_iter().zip(params) {
                inner.def(&p, eval(a, env));
            }
            // call
            eval(body.as_ref(), &mut inner)
        },

        TreeType::Block(ref stats, ref expr) => {
            for s in stats {
                eval(s, env);
            }
            eval(expr, env)
        },

        TreeType::Tuple(ref elems) => Value::Tuple(elems.iter().map(|e| eval(e, env)).collect()),

        TreeType::If(ref cond, ref thenp, ref elsep) => 
            if eval(cond, env).to_bool() {
                eval(thenp, env)
            } else {
                eval(elsep, env)
            },

        TreeType::While(ref cond, ref body) => {
            while eval(cond, env).to_bool() {
                eval(body, env);
            }
            Value::Unit
        },
       
        _ => panic!("\"{}\" not supported", tree)
    }
}