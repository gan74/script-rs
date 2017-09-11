
use tokens::{Token};
use ast::{Statement, Expression};

use std::rc::{Rc};
use std::iter::{Peekable};

use fatal::*;




pub fn parse<'a, T: Iterator<Item = Token<'a>>>(tokens: &mut T) -> Statement {
	let mut peekable = tokens.peekable();
	let s = parse_stmt(&mut peekable);
	expect_eof(&mut peekable);
	s
}

fn parse_stmt<'a, T: Iterator<Item = Token<'a>>>(tokens: &mut Peekable<T>) -> Statement {
	match tokens.peek() {
		Some(&Token::Let(_)) => parse_let(tokens),
		Some(&Token::If(_)) => parse_if(tokens),
		Some(&Token::While(_)) => parse_while(tokens),
		Some(&Token::For(_)) => parse_for(tokens),
		Some(&Token::LeftBrace(_)) => parse_block(tokens),
		_ => {
			let e = parse_expr(tokens);
			Statement::Expr(e)
		}
	}
}

fn parse_expr<'a, T: Iterator<Item = Token<'a>>>(tokens: &mut Peekable<T>) -> Expression {
	try_parse_op(parse_simple_expr(tokens), tokens)
}







// ----------------------------------------- STATEMENTS -----------------------------------------

fn parse_block<'a, T: Iterator<Item = Token<'a>>>(tokens: &mut Peekable<T>) -> Statement {
	expect_leftbrace(tokens);
	let mut stmts = Vec::new();
	loop {
		match tokens.peek() {
			Some(&Token::RightBrace(_)) => {
				tokens.next(); 
				let len = stmts.len();
				return match len {
					0 => Statement::Unit,
					1 => {
						match stmts.first() { 
							Some(&Statement::Decl(_, _)) => Statement::Block(stmts),
							_ => stmts.pop().unwrap()
						}
					}
					_ => Statement::Block(stmts)
				}
			},
			_ => stmts.push(parse_stmt(tokens)),
		}
	}
}

fn parse_let<'a, T: Iterator<Item = Token<'a>>>(tokens: &mut Peekable<T>) -> Statement {
	expect_let(tokens);
	let name = expect_ident(tokens);
	expect_assign(tokens);
	Statement::Decl(name, parse_expr(tokens))
}

fn parse_if<'a, T: Iterator<Item = Token<'a>>>(tokens: &mut Peekable<T>) -> Statement {
	expect_if(tokens);
	let cond = parse_expr(tokens);
	let th = Box::new(parse_stmt(tokens));
	let el = Box::new(match tokens.peek().cloned() {
			Some(Token::Else(_)) => { tokens.next(); parse_stmt(tokens) },
			x => { Statement::Unit }
		});
	Statement::If(cond, th, el)
}

fn parse_while<'a, T: Iterator<Item = Token<'a>>>(tokens: &mut Peekable<T>) -> Statement {
	expect_while(tokens);
	let cond = parse_expr(tokens);
	Statement::While(cond, Box::new(parse_stmt(tokens)))
}

fn parse_for<'a, T: Iterator<Item = Token<'a>>>(tokens: &mut Peekable<T>) -> Statement {
	expect_for(tokens);
	let name = expect_ident(tokens);
	expect_colon(tokens);
	let lst = parse_expr(tokens);
	Statement::For(name, lst, Box::new(parse_stmt(tokens)))
}




// ----------------------------------------- EXPRESSIONS -----------------------------------------

fn parse_simple_expr<'a, T: Iterator<Item = Token<'a>>>(tokens: &mut Peekable<T>) -> Expression {
	let e = match tokens.next() {
		Some(Token::Ident(_, name)) => Expression::Ident(name.to_string()),
		Some(Token::StrLit(_, val)) => Expression::StrLit(val.to_string()),
		Some(Token::NumLit(_, val)) => Expression::NumLit(val.parse().unwrap()),

		Some(Token::LeftPar(_)) => { let e = parse_expr(tokens); expect_rightpar(tokens); e },

		x => panic!("expected identifier, number or '(', got {:?}", x)
	};

	match tokens.peek() {
		Some(&Token::LeftPar(_)) => Expression::Call(Box::new(e), Box::new(parse_simple_expr(tokens))),
		_ => return e
	} 
}

fn parse_op<'a, T: Iterator<Item = Token<'a>>>(e: Expression, tokens: &mut Peekable<T>) -> Expression {
	match tokens.next() {
		Some(Token::Plus(_)) => Expression::Add(Box::new(e), Box::new(parse_simple_expr(tokens))),
		Some(Token::Minus(_)) => Expression::Sub(Box::new(e), Box::new(parse_simple_expr(tokens))),

		Some(Token::Assign(_)) => 
			match e {
				Expression::Ident(name) => Expression::Assign(name, Box::new(parse_expr(tokens))),
				x => panic!("expected identifier as left operand of assignation, got {:?}", x)
			},

		x => panic!("expected '+' or '-', got {:?}", x)
	}
}

fn try_parse_op<'a, T: Iterator<Item = Token<'a>>>(e: Expression, tokens: &mut Peekable<T>) -> Expression {
	let mut e = e;
	let mut lst = Vec::new();
	loop {
		match tokens.peek() {
			Some(&Token::Plus(_)) | Some(&Token::Minus(_)) | Some(&Token::Assign(_)) => e = parse_op(e, tokens),
			Some(&Token::Comma(_)) => { lst.push(e); tokens.next(); e = parse_simple_expr(tokens) },
			Some(&Token::Arrow(_)) => { 
				tokens.next();
				e = Expression::Func(check_args(e), Rc::new(parse_stmt(tokens)));
			},
			_ => break
		}
	}

	if lst.is_empty() {
		e
	} else {
		lst.push(e);
		Expression::ListLit(lst)
	}
}

fn check_args(args: Expression) -> Vec<String> {
	match args {
		Expression::Ident(a) => vec![a.to_owned()],
		Expression::ListLit(lst) => lst.into_iter().map(|e| 
			match e { 
				Expression::Ident(a) => a, 
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


fn expect_leftpar<'a, T: Iterator<Item = Token<'a>>>(tokens: &mut Peekable<T>) {
	match tokens.next() {
		Some(Token::LeftPar(_)) => {}
		x => panic!("expected '(', got {:?}", x)
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

fn expect_rightbrace<'a, T: Iterator<Item = Token<'a>>>(tokens: &mut Peekable<T>) {
	match tokens.next() {
		Some(Token::RightBrace(_)) => {}
		x => panic!("expected '}}', got {:?}", x)
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