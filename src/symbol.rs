
use std::rc::{Rc};
use std::fmt::{Display, Debug, Formatter, Result};

pub type SymbolId = usize;
pub type SymbolName = String;

#[derive(Debug)]
pub struct Symbol {
	name: SymbolName,
	id: SymbolId
}

#[derive(Clone)]
pub struct SymbolRef {
	symbol: Rc<Symbol>
}



impl Symbol {
	pub fn new(name: SymbolName, id: SymbolId) -> Symbol {
		Symbol {
			name: name,
			id: id
		}
	}
}

impl Into<SymbolRef> for Symbol {
    fn into(self) -> SymbolRef {
    	SymbolRef {
			symbol: Rc::new(self)
		}
    }
}

impl SymbolRef {
	pub fn name(&self) -> &SymbolName {
		&self.symbol.name
	}

	pub fn id(&self) -> SymbolId {
		self.symbol.id
	}
}



impl Display for Symbol {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "{}", self.name)
	}
}

impl Debug for SymbolRef {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "{:?}", *self.symbol)
	}
}

impl Display for SymbolRef {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "{}", *self.symbol)
	}
}
