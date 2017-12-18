
use std::iter::{Peekable};

use tree::*;
use token::*;
use position::*;

type Name = String;

const FORCE_BLOCK_BRACES: bool = false;
const ALLOW_BLOCK_IN_EXPR: bool = true;
const ALLOW_TRAILING_COMMA: bool = true;



pub fn parse<I: Iterator<Item = Token>>(tokens: &mut I) -> Tree<Name> {
    let peekable = &mut tokens.peekable();

    let block = parse_block(peekable);
    if block.is_error() {
        block 
    } else {
        if let Some(Token { token: _ , pos }) = peekable.peek().cloned() {
            let block_pos = block.pos.clone();
            TreeType::Block(vec![block], Box::new(TreeType::Error("expected EOF").with_pos(pos))).with_pos(block_pos)
        } else {
            block
        }
    }
}


fn parse_block<I: Iterator<Item = Token>>(tokens: &mut Peekable<I>) -> Tree<Name> {
    fn parse_statements<I: Iterator<Item = Token>>(tokens: &mut Peekable<I>) -> Vec<Tree<Name>> {
        tokens.next();
        let mut stats = Vec::new();
        loop {
            if let Some(Token { token, pos }) =  tokens.peek().cloned() {
                match token {
                    TokenType::RightBrace => {
                        tokens.next();
                        return stats;
                    },
                    TokenType::While => {
                        tokens.next();
                        let cond = parse_expr(tokens);
                        stats.push(TreeType::While(Box::new(cond), Box::new(parse_block(tokens))).with_pos(pos));
                    },
                    _ => stats.push(parse_expr(tokens))
                }
            } else {
                stats.push(eof_error());
                return stats;
            }
        }
    }

    let next = tokens.peek().cloned();
    if let Some(Token { token, pos }) = next {
        if token == TokenType::LeftBrace {
            Tree::block_from_vec(parse_statements(tokens), pos)
        } else if !FORCE_BLOCK_BRACES {
            Tree::block_from_expr(parse_expr(tokens), pos)
        } else {
            TreeType::Error("expected '{'").with_pos(pos)
        }
    } else {
        eof_error()
    }
}


fn parse_simple_expr<I: Iterator<Item = Token>>(tokens: &mut Peekable<I>) -> Tree<Name> {
    if ALLOW_BLOCK_IN_EXPR {
        if let Some(Token { token: TokenType::LeftBrace, pos: _ }) = tokens.peek().cloned() {
            return parse_block(tokens);
        }
    }
    match tokens.next() {
        Some(Token { token, pos }) =>

            match token {
                TokenType::Ident(name) => 
                    match tokens.peek() {
                        Some(&Token { token: TokenType::Assign, pos: _ }) => {
                            tokens.next();
                            TreeType::Assign(name, Box::new(parse_expr(tokens)))
                        },
                        _ => TreeType::Ident(name)
                    },

                TokenType::NumLit(num) => 
                    if let Ok(num) = num.parse::<i64>() {
                        TreeType::IntLit(num)
                    } else {
                        TreeType::Error("expected integer number")
                    },

                TokenType::StrLit(lit) => TreeType::StrLit(lit),

                TokenType::LeftPar => {
                    let mut elems = Vec::new();
                    match tokens.peek().cloned() {
                        Some(Token { token: TokenType::RightPar, pos: _ }) => {
                            tokens.next();
                            return Tree::tuple_from_vec(elems, pos);
                        },
                        _ => elems.push(parse_expr(tokens))
                    }

                    loop {
                        match tokens.next() {
                            Some(Token { token: TokenType::RightPar, pos: _ }) => return Tree::tuple_from_vec(elems, pos),
                            Some(Token { token: TokenType::Comma, pos: _ }) => {
                                if ALLOW_TRAILING_COMMA {
                                    if let Some(Token { token: TokenType::RightPar, pos: _ }) = tokens.peek().cloned() {
                                        tokens.next();
                                        return Tree::tuple_from_vec(elems, pos);
                                    }
                                }
                                elems.push(parse_expr(tokens));
                            },
                            tk => {
                                elems.push(TreeType::Error("expected ',' or ')'").with_pos(error_pos(tk)));
                                return Tree::tuple_from_vec(elems, pos);
                            }
                        }
                    }
                },

                TokenType::If => {
                    let cond = parse_expr(tokens);
                    let thenp = parse_block(tokens);

                    let elsep = if let Some(Token { token: TokenType::Else, pos: _ }) = tokens.peek().cloned() {
                        tokens.next();
                        parse_block(tokens)
                    } else {
                        TreeType::Empty.with_pos(Position::eof())
                    };

                    TreeType::If(Box::new(cond), Box::new(thenp), Box::new(elsep))
                },

                TokenType::Let => {
                    if let Some(Token { token: TokenType::Ident(name), pos: _ }) = tokens.next() {
                        if let Some(Token { token: TokenType::Assign, pos: _ }) = tokens.next() {
                            TreeType::Def(name, Box::new(parse_expr(tokens)))
                        } else {
                            TreeType::Error("expected '='")
                        }
                    } else {
                        TreeType::Error("expected identifier")
                    }
                },

                _ => TreeType::Error("expected expression or '('")
            }.with_pos(pos),

        _ => eof_error()
    }
}

fn parse_expr<I: Iterator<Item = Token>>(tokens: &mut Peekable<I>) -> Tree<Name> {
    fn fetch_op<I: Iterator<Item = Token>>(tokens: &mut Peekable<I>) -> Option<Token> {
        if is_bin_op(tokens.peek()) {
            tokens.next()
        } else {
            None
        }
    }

    fn is_bin_op(token: Option<&Token>) -> bool {
        if let Some(token) = token {
            match &token.token {
                &TokenType::Eq | &TokenType::Neq => true,
                &TokenType::Plus | &TokenType::Minus | &TokenType::Star | &TokenType::Slash => true,
                _ => false
            }
        } else {
            false
        } 
    }

    fn bin_op_associativity(tk: &Token) -> i32 {
        match &tk.token {
            &TokenType::Eq | &TokenType::Neq => 0,
            &TokenType::Plus | &TokenType::Minus => 1,
            &TokenType::Star | &TokenType::Slash => 2,
            _ => unreachable!()
        }
    }

    let mut left = parse_simple_expr(tokens);
    let mut op = if let Some(op) = fetch_op(tokens) {
        op
    } else {
        return left;
    };
    let mut middle = parse_simple_expr(tokens);
    loop {
        let op_2 = if let Some(op) = fetch_op(tokens) {
            op
        } else {
            return create_bin_op(op, left, middle);
        };
        let right = parse_simple_expr(tokens);
        if bin_op_associativity(&op) < bin_op_associativity(&op_2) {
            middle = create_bin_op(op_2, middle, right);
        } else {
            left = create_bin_op(op, left, middle);
            middle = right;
            op = op_2;
        }
    }
}


fn create_bin_op(op: Token, lhs: Tree<Name>, rhs: Tree<Name>) -> Tree<Name> {
    match op.token {
        TokenType::Eq => TreeType::Eq(Box::new(lhs), Box::new(rhs)),
        TokenType::Neq => TreeType::Neq(Box::new(lhs), Box::new(rhs)),
        TokenType::Plus => TreeType::Add(Box::new(lhs), Box::new(rhs)),
        TokenType::Minus => TreeType::Sub(Box::new(lhs), Box::new(rhs)),
        TokenType::Star => TreeType::Mul(Box::new(lhs), Box::new(rhs)),
        TokenType::Slash => TreeType::Div(Box::new(lhs), Box::new(rhs)),
        _ => TreeType::Error("expected '+', '-', '*', '/', '==' or '!='")
    }.with_pos(op.pos)
}

fn eof_error() -> Tree<Name> {
    TreeType::Error("unexpected EOF").with_pos(Position::eof())
}

fn error_pos(tk: Option<Token>) -> Position {
    if let Some(Token { token: _, pos }) = tk {
        pos
    } else {
        Position::eof()
    }
}