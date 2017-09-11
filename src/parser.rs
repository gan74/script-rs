
use tokens::{Token};
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
		_ => try_parse_bin_op(parse_simple_expr(tokens), tokens)
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
		Some(Token::Ident(_, name)) => Tree::Ident(name.to_string()),
		Some(Token::StrLit(_, val)) => Tree::StrLit(val.to_string()),
		Some(Token::NumLit(_, val)) => Tree::NumLit(val.parse().unwrap()),

		Some(Token::LeftPar(_)) => { let e = parse_tree(tokens); expect_rightpar(tokens); e },

		x => panic!("expected identifier, number or '(', got {:?}", x)
	};

	match tokens.peek() {
		Some(&Token::LeftPar(_)) => Tree::Call(box_(e), box_(parse_simple_expr(tokens))),
		_ => return e
	} 
}

fn parse_bin_op<'a, T: Iterator<Item = Token<'a>>>(e: Tree, tokens: &mut Peekable<T>) -> Tree {
	match tokens.next() {
		Some(Token::Plus(_)) => Tree::Add(box_(e), box_(parse_simple_expr(tokens))),
		Some(Token::Minus(_)) => Tree::Sub(box_(e), box_(parse_simple_expr(tokens))),

		Some(Token::Assign(_)) => 
			match e {
				Tree::Ident(name) => Tree::Assign(name, box_(parse_tree(tokens))),
				x => panic!("expected identifier as left operand of assignation, got {:?}", x)
			},

		x => panic!("expected '+' or '-', got {:?}", x)
	}
}

fn try_parse_bin_op<'a, T: Iterator<Item = Token<'a>>>(e: Tree, tokens: &mut Peekable<T>) -> Tree {
	let mut e = e;
	let mut lst = Vec::new();
	loop {
		match tokens.peek() {
			Some(&Token::Plus(_)) | Some(&Token::Minus(_)) | Some(&Token::Assign(_)) => e = parse_bin_op(e, tokens),
			Some(&Token::Comma(_)) => { lst.push(e); tokens.next(); e = parse_simple_expr(tokens) },
			Some(&Token::Arrow(_)) => { 
				tokens.next();
				e = Tree::Func(collect_args(e), Rc::new(parse_tree(tokens)));
			},
			_ => break
		}
	}

	if lst.is_empty() {
		e
	} else {
		lst.push(e);
		Tree::ListLit(lst)
	}
}

fn collect_args(args: Tree) -> Vec<String> {
	match args {
		Tree::Ident(a) => vec![a.to_owned()],
		Tree::ListLit(lst) => lst.into_iter().map(|e| 
			match e { 
				Tree::Ident(a) => a, 
				_ => panic!("{:?} is not valid as an argument", e) 
			}).collect(),
		e => panic!("{:?} is not valid as an argument", e)
	}
}




// ----------------------------------------- HELPERS -----------------------------------------

fn expect_ident<'a, T: Iterator<Item = Token<'a>>>(tokens: &mut Peekable<T>) -> String {
	match tokens.next() {
		Some(Token::Ident(_, name)) => name.to_string(),
		x => panic!("expected identifier, got {:?}", x)
	}
}


fn expect_let<'a, T: Iterator<Item = Token<'a>>>(tokens: &mut Peekable<T>) {
	match tokens.next() {
		Some(Token::Let(_)) => {},
		x => panic!("expected 'let', got {:?}", x)
	}
}

fn expect_if<'a, T: Iterator<Item = Token<'a>>>(tokens: &mut Peekable<T>) {
	match tokens.next() {
		Some(Token::If(_)) => {},
		x => panic!("expected 'if', got {:?}", x)
	}
}

fn expect_while<'a, T: Iterator<Item = Token<'a>>>(tokens: &mut Peekable<T>) {
	match tokens.next() {
		Some(Token::While(_)) => {},
		x => panic!("expected 'while', got {:?}", x)
	}
}

fn expect_for<'a, T: Iterator<Item = Token<'a>>>(tokens: &mut Peekable<T>) {
	match tokens.next() {
		Some(Token::For(_)) => {},
		x => panic!("expected 'for', got {:?}", x)
	}
}

fn expect_colon<'a, T: Iterator<Item = Token<'a>>>(tokens: &mut Peekable<T>) {
	match tokens.next() {
		Some(Token::Colon(_)) => {},
		x => panic!("expected ':', got {:?}", x)
	}
}

fn expect_rightpar<'a, T: Iterator<Item = Token<'a>>>(tokens: &mut Peekable<T>) {
	match tokens.next() {
		Some(Token::RightPar(_)) => {}
		x => panic!("expected ')', got {:?}", x)
	}
}

fn expect_leftbrace<'a, T: Iterator<Item = Token<'a>>>(tokens: &mut Peekable<T>) {
	match tokens.next() {
		Some(Token::LeftBrace(_)) => {}
		x => panic!("expected '{{', got {:?}", x)
	}
}

fn expect_assign<'a, T: Iterator<Item = Token<'a>>>(tokens: &mut Peekable<T>) {
	match tokens.next() {
		Some(Token::Assign(_)) => {}
		x => panic!("expected '=', got {:?}", x)
	}
}

fn expect_eof<'a, T: Iterator<Item = Token<'a>>>(tokens: &mut Peekable<T>) {
	match tokens.next() {
		Some(x) => panic!("unexpected {:?} at end of stream", x),
		None => {}
	}
}