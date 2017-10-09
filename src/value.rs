
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, Sub, Mul, Div};
use std::rc::{Rc};

use std::fmt;
use std::result;

use utils::*;

type Result<T> = result::Result<T, ErrorKind>;


#[derive(Clone)]
pub struct FuncValue {
	pub args: usize,
	pub func: Rc<Fn(Vec<Value>) -> Result<Value>>
}

#[derive(Clone)]
pub enum Value {
	Unit, 

	Func(FuncValue),

	Str(String),
	Num(f64),

	List(Vec<Value>)
}

impl Display for Value {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self {
			&Value::Unit => write!(f, "Unit"),
			&Value::Func(_) => write!(f, "Func"),
			&Value::Str(ref s) => write!(f, "{}", s),
			&Value::Num(v) => write!(f, "{}", v),
			&Value::List(ref lst) => {
				let mut string = lst.iter().fold(String::new(), |s, i| s + &format!("{}", i) + ", ");
				string.pop(); string.pop();
				write!(f, "[{}]", string)
			}
		}
	}
}

impl Debug for Value {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "{}", self)
	}
}

impl Value {
	pub fn from_func_1(func: fn(Value) -> Result<Value>) -> Value {
		Value::Func(FuncValue {
			args: 1,
			func: Rc::new(move |v| {
				let mut args = v.into_iter();
				let a = args.next().unwrap();
				func(a)
			})
		})
	}

	pub fn from_func_2(func: fn(Value, Value) -> Result<Value>) -> Value {
		Value::Func(FuncValue {
			args: 2,
			func: Rc::new(move |v| {
				let mut args = v.into_iter();
				let a = args.next().unwrap();
				let b = args.next().unwrap();
				func(a, b)
			})
		})
	}

	pub fn to_str(self) -> String {
		format!("{}", self)
	}

	pub fn to_list(self) -> Result<Vec<Value>> {
		match self {
			Value::List(lst) => Ok(lst.clone()),

			x => panic!("{:?} is not a list", x)
		}
	}

	pub fn to_bool(self) -> Result<bool> {		
		if let Some(n) = self.try_num() {
			Ok(n != 0.0)
		} else {
			Err(ErrorKind::Generic(format!("{:?} is not a bool", self)))
		}
	}

	pub fn to_num(self) -> Result<f64> {
		if let Some(n) = self.try_num() {
			Ok(n) 
		} else {
			Err(ErrorKind::Generic(format!("{:?} is not a number", self)))
		}
	}

	pub fn call(&self, args: Vec<Value>) -> Result<Value> {
		match self {
			&Value::Func(ref f) => 
				if args.len() != f.args {
					return Err(ErrorKind::WrongArgCount(f.args, args.len()))
				} else {
					(*f.func)(args)
				},

			x => Err(ErrorKind::Generic(format!("{:?} is not a function", x)))
		}
	}

	fn try_num(&self) -> Option<f64> {
		match self {
			&Value::Str(ref s) => s.parse().ok(),
			&Value::Num(n) => Some(n),
			_ => None
		}
	}
}


impl Add<Value> for Value {
	type Output = Result<Value>;

	fn add(self, rhs: Value) -> Self::Output {
		match self {
			Value::Str(s) => Ok(Value::Str(s + &rhs.to_str())),
			Value::Num(n) => Ok(Value::Num(n + rhs.to_num()?)),
			Value::List(mut v) => {
				if let Value::List(mut r) = rhs {
					v.append(&mut r);
				} else {
					v.push(rhs);
				}
				Ok(Value::List(v))
			}

			_ => panic!("{:?} and {:?} can not be added", self, rhs)
		}
	}
}

impl Sub<Value> for Value {
	type Output = Result<Value>;

	fn sub(self, rhs: Value) -> Self::Output {
		match self {
			Value::Num(n) => Ok(Value::Num(n - rhs.to_num()?)),

			_ => panic!("{:?} and {:?} can not be subtracted", self, rhs)
		}
	}
}

impl Mul<Value> for Value {
	type Output = Result<Value>;

	fn mul(self, rhs: Value) -> Self::Output {
		match self {
			//Value::Str(s) => Value::Str(s * rhs.to_num()),
			Value::Num(n) => Ok(Value::Num(n * rhs.to_num()?)),
			//Value::List(mut v) => { Value::List(v * rhs.to_num()) },

			_ => panic!("{:?} and {:?} can not be multiplied", self, rhs)
		}
	}
}

impl Div<Value> for Value {
	type Output = Result<Value>;

	fn div(self, rhs: Value) -> Self::Output {
		match self {
			Value::Num(n) => Ok(Value::Num(n / rhs.to_num()?)),

			_ => panic!("{:?} and {:?} can not be divided", self, rhs)
		}
	}
}


