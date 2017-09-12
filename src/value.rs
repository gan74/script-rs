
use std::fmt::{Debug, Display, Result, Formatter};
use std::ops::{Add, Sub, Mul, Div};
use std::rc::{Rc};

#[derive(Clone)]
pub enum Value {
	Unit, 

	Func(Rc<Fn(Value) -> Value>),

	Str(String),
	Num(f64),

	List(Vec<Value>)
}

impl Display for Value {
	fn fmt(&self, f: &mut Formatter) -> Result {
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
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "{}", self)
	}
}

impl Value {
	pub fn to_str(self) -> String {
		format!("{}", self)
	}

	pub fn to_list(self) -> Vec<Value> {
		match self {
			Value::List(lst) => lst.clone(),

			x => panic!("{:?} is not a list", x)
		}
	}

	pub fn to_bool(self) -> bool {		
		if let Some(n) = self.try_num() {
			n != 0.0
		} else {
			panic!("{:?} is not a bool", self)
		}
	}

	pub fn to_num(self) -> f64 {
		if let Some(n) = self.try_num() {
			n 
		} else {
			panic!("{:?} is not a number", self)
		}
	}


	pub fn call(&self, args: Value) -> Value {
		match self {
			&Value::Func(ref f) => (*f)(args),

			x => panic!("{:?} is not a function", x)
		}
	}

	fn try_num(&self) -> Option<f64> {
		match self {
			&Value::Str(ref s) => s.parse().ok(),
			&Value::Num(n) => Some(n),
			_ =>  None
		}
	}
}


impl Add<Value> for Value {
	type Output = Value;

	fn add(self, rhs: Value) -> Value {
		match self {
			Value::Str(s) => Value::Str(s + &rhs.to_str()),
			Value::Num(n) => Value::Num(n + rhs.to_num()),
			Value::List(mut v) => { v.push(rhs); Value::List(v) },

			_ => panic!("{:?} and {:?} can not be added", self, rhs)
		}
	}
}

impl Sub<Value> for Value {
	type Output = Value;

	fn sub(self, rhs: Value) -> Value {
		match self {
			Value::Num(n) => Value::Num(n - rhs.to_num()),

			_ => panic!("{:?} and {:?} can not be subtracted", self, rhs)
		}
	}
}

impl Mul<Value> for Value {
	type Output = Value;

	fn mul(self, rhs: Value) -> Value {
		match self {
			//Value::Str(s) => Value::Str(s * rhs.to_num()),
			Value::Num(n) => Value::Num(n * rhs.to_num()),
			//Value::List(mut v) => { Value::List(v * rhs.to_num()) },

			_ => panic!("{:?} and {:?} can not be multiplied", self, rhs)
		}
	}
}

impl Div<Value> for Value {
	type Output = Value;

	fn div(self, rhs: Value) -> Value {
		match self {
			Value::Num(n) => Value::Num(n / rhs.to_num()),

			_ => panic!("{:?} and {:?} can not be divided", self, rhs)
		}
	}
}


