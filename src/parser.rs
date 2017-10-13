
use std::iter::{Peekable};

use tokens::*;
use position::*;
use ast::*;
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
		_ => parse_expr(parse_simple_expr(tokens), tokens)
	}
}




fn parse_block<'a, T: Iterator<Item = Token<'a>>>(tokens: &mut Peekable<T>) -> Tree {
	let pos = expect_leftbrace(tokens);
	let mut stmts = Vec::new();
	loop {
		match tokens.peek() {
			Some(&Token::RightBrace(_)) => {
				tokens.next(); 
				let len = stmts.len();
				return match len {
					0 => Tree::Unit(pos),
					1 => {
						match stmts.first() { 
							Some(&Tree::DeclByName(_, _, _)) => Tree::Block(pos, stmts),
							_ => stmts.pop().unwrap()
						}
					}
					_ => Tree::Block(pos, stmts)
				}
			},
			_ => stmts.push(parse_tree(tokens)),
		}
	}
}

fn parse_let<'a, T: Iterator<Item = Token<'a>>>(tokens: &mut Peekable<T>) -> Tree {
	let pos = expect_let(tokens);
	let name = expect_ident(tokens);
	expect_assign(tokens);
	Tree::DeclByName(pos, name, box_(parse_tree(tokens)))
}

fn parse_if<'a, T: Iterator<Item = Token<'a>>>(tokens: &mut Peekable<T>) -> Tree {
	let pos = expect_if(tokens);
	let cond = box_(parse_tree(tokens));
	let th = box_(parse_tree(tokens));
	let el = box_(
		match tokens.peek().cloned() {
			Some(Token::Else(_)) => { tokens.next(); parse_tree(tokens) },
			_ => Tree::Unit(pos.clone())
		});
	Tree::If(pos, cond, th, el)
}

fn parse_while<'a, T: Iterator<Item = Token<'a>>>(tokens: &mut Peekable<T>) -> Tree {
	let pos = expect_while(tokens);
	let cond = box_(parse_tree(tokens));
	Tree::While(pos, cond, box_(parse_tree(tokens)))
}

fn parse_for<'a, T: Iterator<Item = Token<'a>>>(tokens: &mut Peekable<T>) -> Tree {
	let pos = expect_for(tokens);
	let name = expect_ident(tokens);
	expect_colon(tokens);
	let lst = box_(parse_tree(tokens));
	Tree::ForByName(pos, name, lst, box_(parse_tree(tokens)))
}




// ----------------------------------------- Tree -----------------------------------------

fn parse_simple_expr<'a, T: Iterator<Item = Token<'a>>>(tokens: &mut Peekable<T>) -> Tree {
	let e = match tokens.next() {
		Some(Token::StrLit(p, val)) => Tree::StrLit(p, val.to_string()),
		Some(Token::NumLit(p, val)) => Tree::NumLit(p, val.parse().unwrap()),

		Some(Token::Ident(p, name)) => {
			let lhs = parse_ident(name, p, tokens);
			try_parse_def(vec![lhs], tokens)
		}

		Some(Token::LeftPar(_)) => { 
			let lst = parse_paren(tokens); 
			expect_rightpar(tokens); 
			try_parse_def(lst, tokens)
		},

		Some(Token::LeftBracket(p)) => {
			let lst = parse_list(tokens);
			expect_rightbracket(tokens);
			Tree::ListLit(p, lst)
		},

		x => fatal_tk("Expected identifier, number or '('", x)
	};

	if let Some(Token::LeftPar(p)) = tokens.peek().cloned() {
		tokens.next();
		let args = parse_paren(tokens);
		expect_rightpar(tokens);
		Tree::Call(p.clone(), box_(e), box_(Tree::ListLit(p, args)))
	} else {
		e
	}
}

fn try_parse_def<'a, T: Iterator<Item = Token<'a>>>(args: Vec<Tree>, tokens: &mut Peekable<T>) -> Tree {
	let t = tokens.peek().cloned();
	if let Some(Token::Arrow(pos)) = t {
		tokens.next();
		let args = args.into_iter().map(|arg| {
			match arg {
				Tree::IdentByName(_, n) => n,
				t => fatal_pos("Expected identifier, got expression", t.position())
			}
		}).collect();
		Tree::FuncByName(pos, args, box_(parse_tree(tokens)))
	} else {
		match args.len() {
			1 => args.into_iter().next().unwrap(),
			_ => fatal_tk("Expected '=>'", t)
		}
	}
}

fn parse_ident<'a, T: Iterator<Item = Token<'a>>>(name: &str, pos: Position, tokens: &mut Peekable<T>) -> Tree {
	let tk = tokens.peek().cloned();
	if let Some(Token::Assign(p)) = tk {
		tokens.next();
		Tree::AssignByName(p, name.to_owned(), box_(parse_tree(tokens)))
	} else {
		Tree::IdentByName(pos, name.to_owned())
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

fn parse_list<'a, T: Iterator<Item = Token<'a>>>(tokens: &mut Peekable<T>) -> Vec<Tree> {
	if let Some(&Token::RightBracket(_)) = tokens.peek() {
		return Vec::new()
	}
	let mut lst = Vec::new();
	loop {
		lst.push(parse_tree(tokens));
		match tokens.peek().cloned() {
			Some(Token::Comma(_)) => tokens.next(),
			Some(Token::RightBracket(_)) => return lst,

			x => fatal_tk("Expected ',' or ']'", x)
		};
	}
}


fn parse_expr<'a, T: Iterator<Item = Token<'a>>>(lhs: Tree, tokens: &mut Peekable<T>) -> Tree {
	fn is_op(tk: &Option<Token>) -> bool {
		match tk {
			&Some(Token::Plus(_)) | &Some(Token::Minus(_)) | &Some(Token::Times(_)) | &Some(Token::Div(_)) => true,
			&Some(Token::Eq(_)) | &Some(Token::Neq(_)) => true,
			&Some(Token::Assign(_)) => true,
			_ => false
		}
	}

	fn create_op(lhs: Tree, rhs: Tree, tk: Option<Token>) -> Tree {
		match tk {
			Some(Token::Plus(p)) => Tree::Add(p, box_(lhs), box_(rhs)),
			Some(Token::Minus(p)) => Tree::Sub(p, box_(lhs), box_(rhs)),
			Some(Token::Times(p)) => Tree::Mul(p, box_(lhs), box_(rhs)),
			Some(Token::Div(p)) => Tree::Div(p, box_(lhs), box_(rhs)),
			Some(Token::Eq(p)) => Tree::Eq(p, box_(lhs), box_(rhs)),
			Some(Token::Neq(p)) => Tree::Neq(p, box_(lhs), box_(rhs)),
			Some(Token::Assign(pos)) => {
				match lhs {
					Tree::IdentByName(_, name) => Tree::AssignByName(pos, name, box_(rhs)),
					t => fatal_pos("Expected identifier as left operand of assignation", t.position())
				}
			},
			_ => unreachable!()
		}
	} 

	fn assoc(tk: &Option<Token>) -> i32 {
		match tk {
			&Some(Token::Eq(_)) | &Some(Token::Neq(_)) => 0,
			&Some(Token::Plus(_)) | &Some(Token::Minus(_)) => 1,
			&Some(Token::Times(_)) | &Some(Token::Div(_)) => 2,
			_ => unreachable!()
		}
	}


	let mut first_op = tokens.peek().cloned();
	if !is_op(&first_op) {
		return lhs;
	}
	tokens.next();
	let mut lhs = lhs;
	let mut mhs = parse_simple_expr(tokens); 

	loop {
		let second_op = tokens.peek().cloned();

		if is_op(&second_op) {
			tokens.next();
		} else {
			return create_op(lhs, mhs, first_op)
		}

		let rhs = parse_simple_expr(tokens);

		if assoc(&second_op) > assoc(&first_op) {
			mhs = create_op(mhs, rhs, second_op);
		} else {
			lhs = create_op(lhs, mhs, first_op);
			mhs = rhs;
			first_op = second_op;
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


fn expect_let<'a, T: Iterator<Item = Token<'a>>>(tokens: &mut Peekable<T>) -> Position {
	match tokens.next() {
		Some(Token::Let(p)) => p,
		x => fatal_tk("Expected 'let'", x)
	}
}

fn expect_if<'a, T: Iterator<Item = Token<'a>>>(tokens: &mut Peekable<T>) -> Position {
	match tokens.next() {
		Some(Token::If(p)) => p,
		x => fatal_tk("Expected 'if'", x)
	}
}

fn expect_while<'a, T: Iterator<Item = Token<'a>>>(tokens: &mut Peekable<T>) -> Position {
	match tokens.next() {
		Some(Token::While(p)) => p,
		x => fatal_tk("Expected 'while'", x)
	}
}

fn expect_for<'a, T: Iterator<Item = Token<'a>>>(tokens: &mut Peekable<T>) -> Position {
	match tokens.next() {
		Some(Token::For(p)) => p,
		x => fatal_tk("Expected 'ForByName'", x)
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
		Some(Token::RightPar(_)) => {},
		x => fatal_tk("Expected ')'", x)
	}
}

fn expect_rightbracket<'a, T: Iterator<Item = Token<'a>>>(tokens: &mut Peekable<T>) {
	match tokens.next() {
		Some(Token::RightBracket(_)) => {},
		x => fatal_tk("Expected ']'", x)
	}
}

fn expect_leftbrace<'a, T: Iterator<Item = Token<'a>>>(tokens: &mut Peekable<T>) -> Position {
	match tokens.next() {
		Some(Token::LeftBrace(p)) => p,
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