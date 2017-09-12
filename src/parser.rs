

use tokens::{Token};
use position::{Position};
use tree::{Tree};

use std::rc::{Rc};
use std::iter::{Peekable};

use utils::*;



pub fn parse<'a, T: Iterator<Item = Token<'a>>>(tokens: &mut T) -> Tree {
	let mut peekable = tokens.peekable();
	let s = parse_tree(&mut peekable);
	expect_eof(&mut peekable);
	s
}

fn parse_tree<'a, T: Iterator<Item = Token<'a>>>(tokens: &mut Peekable<T>) -> Tree {
	match tokens.peek() {
		Some(&Token::Let(_)) => parse_let(tokens),
		Some(&Token::If(_)) => parse_if(tokens),
		Some(&Token::While(_)) => parse_while(tokens),
		Some(&Token::For(_)) => parse_for(tokens),
		Some(&Token::LeftBrace(_)) => parse_block(tokens),
		_ => parse_bin_op(parse_simple_expr(tokens), tokens)
	}
}




fn parse_block<'a, T: Iterator<Item = Token<'a>>>(tokens: &mut Peekable<T>) -> Tree {
	expect_leftbrace(tokens);
	let mut stmts = Vec::new();
	loop {
		match tokens.peek() {
			Some(&Token::RightBrace(_)) => {
				tokens.next(); 
				let len = stmts.len();
				return match len {
					0 => Tree::Unit,
					1 => {
						match stmts.first() { 
							Some(&Tree::Decl(_, _)) => Tree::Block(stmts),
							_ => stmts.pop().unwrap()
						}
					}
					_ => Tree::Block(stmts)
				}
			},
			_ => stmts.push(parse_tree(tokens)),
		}
	}
}

fn parse_let<'a, T: Iterator<Item = Token<'a>>>(tokens: &mut Peekable<T>) -> Tree {
	expect_let(tokens);
	let name = expect_ident(tokens);
	expect_assign(tokens);
	Tree::Decl(name, box_(parse_tree(tokens)))
}

fn parse_if<'a, T: Iterator<Item = Token<'a>>>(tokens: &mut Peekable<T>) -> Tree {
	expect_if(tokens);
	let cond = box_(parse_tree(tokens));
	let th = box_(parse_tree(tokens));
	let el = box_(
		match tokens.peek().cloned() {
			Some(Token::Else(_)) => { tokens.next(); parse_tree(tokens) },
			_ => Tree::Unit
		});
	Tree::If(cond, th, el)
}

fn parse_while<'a, T: Iterator<Item = Token<'a>>>(tokens: &mut Peekable<T>) -> Tree {
	expect_while(tokens);
	let cond = box_(parse_tree(tokens));
	Tree::While(cond, box_(parse_tree(tokens)))
}

fn parse_for<'a, T: Iterator<Item = Token<'a>>>(tokens: &mut Peekable<T>) -> Tree {
	expect_for(tokens);
	let name = expect_ident(tokens);
	expect_colon(tokens);
	let lst = box_(parse_tree(tokens));
	Tree::For(name, lst, box_(parse_tree(tokens)))
}




// ----------------------------------------- Tree -----------------------------------------

fn parse_simple_expr<'a, T: Iterator<Item = Token<'a>>>(tokens: &mut Peekable<T>) -> Tree {
	let e = match tokens.next() {
		Some(Token::StrLit(_, val)) => Tree::StrLit(val.to_string()),
		Some(Token::NumLit(_, val)) => Tree::NumLit(val.parse().unwrap()),

		Some(Token::Ident(pos, name)) => try_parse_def(vec![Tree::Ident(name.to_owned())], tokens, pos),

		Some(t @ Token::LeftPar(_)) => { 
			let lst = parse_paren(tokens); 
			expect_rightpar(tokens); 
			try_parse_def(lst, tokens, t.position())
		},

		x => fatal_tk("Expected identifier, number or '('", x)
	};

	if let Some(Token::LeftPar(_)) = tokens.peek().cloned() {
		tokens.next();
		let args = parse_paren(tokens);
		expect_rightpar(tokens);
		Tree::Call(box_(e), box_(Tree::ListLit(args)))
	} else {
		e
	}
}

fn try_parse_def<'a, T: Iterator<Item = Token<'a>>>(args: Vec<Tree>, tokens: &mut Peekable<T>, pos: Position) -> Tree {
	if let Some(&Token::Arrow(_)) = tokens.peek() {
		tokens.next();
		let args = args.into_iter().map(|arg| {
			match arg {
				Tree::Ident(n) => n,
				_ => fatal_pos("Expected identifier, got expression", pos.clone())
			}
		}).collect();
		Tree::Func(args, Rc::new(parse_tree(tokens)))
	} else {
		match args.len() {
			1 => args.into_iter().next().unwrap(),
			_ => fatal_pos("Expected expression, got list", pos)
		}
	}
}

fn parse_paren<'a, T: Iterator<Item = Token<'a>>>(tokens: &mut Peekable<T>) -> Vec<Tree> {
	if let Some(&Token::RightPar(_)) = tokens.peek() {
		return Vec::new()
	}
	let mut lst = Vec::new();
	loop {
		lst.push(parse_tree(tokens));
		match tokens.peek().cloned() {
			Some(Token::Comma(_)) => tokens.next(),
			Some(Token::RightPar(_)) => return lst,

			x => fatal_tk("Expected ',' or ')'", x)
		};
	}
}

fn parse_bin_op<'a, T: Iterator<Item = Token<'a>>>(e: Tree, tokens: &mut Peekable<T>) -> Tree {
	let mut e = e;
	loop {
		match tokens.peek().cloned() {
			Some(Token::Plus(_)) => { tokens.next(); e = Tree::Add(box_(e), box_(parse_simple_expr(tokens))) },
			Some(Token::Minus(_)) => { tokens.next(); e = Tree::Sub(box_(e), box_(parse_simple_expr(tokens))) },

			x @ Some(Token::Assign(_)) => {
				tokens.next();
				e = match e {
					Tree::Ident(name) => Tree::Assign(name, box_(parse_tree(tokens))),
					_ => fatal_tk("Expected identifier as left operand of assignation", x)
				}
			},

			_ => return e
		}
	}
}



// ----------------------------------------- HELPERS -----------------------------------------

fn expect_ident<'a, T: Iterator<Item = Token<'a>>>(tokens: &mut Peekable<T>) -> String {
	match tokens.next() {
		Some(Token::Ident(_, name)) => name.to_string(),
		x => fatal_tk("Expected identifier", x)
	}
}


fn expect_let<'a, T: Iterator<Item = Token<'a>>>(tokens: &mut Peekable<T>) {
	match tokens.next() {
		Some(Token::Let(_)) => {},
		x => fatal_tk("Expected 'let'", x)
	}
}

fn expect_if<'a, T: Iterator<Item = Token<'a>>>(tokens: &mut Peekable<T>) {
	match tokens.next() {
		Some(Token::If(_)) => {},
		x => fatal_tk("Expected 'if'", x)
	}
}

fn expect_while<'a, T: Iterator<Item = Token<'a>>>(tokens: &mut Peekable<T>) {
	match tokens.next() {
		Some(Token::While(_)) => {},
		x => fatal_tk("Expected 'while'", x)
	}
}

fn expect_for<'a, T: Iterator<Item = Token<'a>>>(tokens: &mut Peekable<T>) {
	match tokens.next() {
		Some(Token::For(_)) => {},
		x => fatal_tk("Expected 'for'", x)
	}
}

fn expect_colon<'a, T: Iterator<Item = Token<'a>>>(tokens: &mut Peekable<T>) {
	match tokens.next() {
		Some(Token::Colon(_)) => {},
		x => fatal_tk("Expected ':'", x)
	}
}

fn expect_rightpar<'a, T: Iterator<Item = Token<'a>>>(tokens: &mut Peekable<T>) {
	match tokens.next() {
		Some(Token::RightPar(_)) => {}
		x => fatal_tk("Expected ')'", x)
	}
}

fn expect_leftbrace<'a, T: Iterator<Item = Token<'a>>>(tokens: &mut Peekable<T>) {
	match tokens.next() {
		Some(Token::LeftBrace(_)) => {}
		x => fatal_tk("Expected '{{'", x)
	}
}

fn expect_assign<'a, T: Iterator<Item = Token<'a>>>(tokens: &mut Peekable<T>) {
	match tokens.next() {
		Some(Token::Assign(_)) => {}
		x => fatal_tk("Expected '='", x)
	}
}

fn expect_eof<'a, T: Iterator<Item = Token<'a>>>(tokens: &mut Peekable<T>) {
	match tokens.next() {
		Some(x) => fatal(&format!("Unexpected {:?} at end of stream", x)),
		None => {}
	}
}