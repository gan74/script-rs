
use std::collections::{HashMap};

use symbol::*;
use utils::*;



pub struct Context {
	symbols: Vec<SymbolRef>,
	frames: Vec<Frame>

}

#[derive(Debug)]
struct Frame {
	by_id: HashMap<SymbolId, SymbolRef>,
	by_name: HashMap<SymbolName, SymbolRef>
}


impl Frame {
	fn new() -> Frame {
		Frame {
			by_id: HashMap::new(),
			by_name: HashMap::new()
		}
	}

	fn get(&self, name: &SymbolName) -> Option<&SymbolRef> {
		self.by_name.get(name)
	}

	fn push(&mut self, sym: &SymbolRef) {
		self.by_id.insert(sym.id(), sym.clone());
		self.by_name.insert(sym.name().clone(), sym.clone());
	}
}



impl Context {
	pub fn new() -> Context {
		Context {
			symbols: Vec::new(),
			frames: vec![Frame::new()]
		}
	}

	pub fn decl(&mut self, name: SymbolName) -> Result<SymbolRef, ErrorKind> {
		if self.by_name(name.clone()).is_ok() {
			Err(ErrorKind::AlreadyDeclared(name))
		} else {
			let next_id = self.symbols.len();
			self.symbols.push(Symbol::new(name, next_id).into());
			let sym = self.symbols.last().unwrap();
			self.frames.last_mut().unwrap().push(&sym);
			Ok(sym.clone())
		}
	}

	pub fn by_name(&mut self, name: SymbolName) -> Result<SymbolRef, ErrorKind> {
		for n in self.frames.iter().rev() {
			match n.get(&name) {
				Some(s) => return Ok(s.clone()),
				None => ()
			}
		}
		Err(ErrorKind::NotDeclared(name))
		
	}

	pub fn push(&mut self) {
		self.frames.push(Frame::new())
	}

	pub fn pop(&mut self) {
		self.frames.pop();
	}
}