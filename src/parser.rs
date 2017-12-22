
use std::iter::Peekable;
use std::rc::Rc;

use tree::*;
use token::*;
use position::*;

type Name = String;

const FORCE_BLOCK_BRACES: bool = false;
const FOLD_TUPLE_1: bool = true;


pub fn parse<I: Iterator<Item = Token>>(tokens: &mut I) -> Tree<Name> {
    let peekable = &mut tokens.peekable();

    // parse block
    let block = parse_block(peekable);

    // if error return immediatly, else check the iterator is empty
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
    // read statements until '}'
    fn parse_statements<I: Iterator<Item = Token>>(tokens: &mut Peekable<I>) -> Vec<Tree<Name>> {
        let mut stats = Vec::new();
        loop {
            if let Some(Token { token, pos }) =  tokens.peek().cloned() {
                match token {
                    // end of block, return
                    TokenType::RightBrace => {
                        tokens.next();
                        return stats;
                    },
                    // while can only happen in blocks, so we parse it here
                    TokenType::While => {
                        tokens.next();
                        let cond = parse_expr(tokens);
                        stats.push(TreeType::While(Box::new(cond), Box::new(parse_block(tokens))).with_pos(pos));
                    },
                    // generic statement
                    _ => stats.push(parse_expr(tokens))
                }
            } else {
                // we reached the end of the stream before the end of the block
                stats.push(eof_error());
                return stats;
            }
        }
    }

    // build a block from a vec by taking the last statement and using it as the block return value (or put Empty if stats empty)
    fn block_from_vec(mut stats: Vec<Tree<Name>>, pos: Position) -> Tree<Name> {
        let expr = stats.pop().unwrap_or(TreeType::Empty.with_pos(pos.clone()));
        TreeType::Block(stats, Box::new(expr)).with_pos(pos)
    }

    fn block_from_expr(expr: Tree<Name>, pos: Position) -> Tree<Name> {
        TreeType::Block(Vec::new(), Box::new(expr)).with_pos(pos)
    }


    if let Some(Token { token, pos }) = tokens.peek().cloned() {
        if token == TokenType::LeftBrace {
            tokens.next();
            block_from_vec(parse_statements(tokens), pos)
        } else if !FORCE_BLOCK_BRACES {
            block_from_expr(parse_expr(tokens), pos)
        } else {
            TreeType::Error("expected '{'").with_pos(pos)
        }
    } else {
        eof_error()
    }
}

// parse a 'simple' expression (without binops)
fn parse_simple_expr<I: Iterator<Item = Token>>(tokens: &mut Peekable<I>) -> Tree<Name> {        
    if let Some(Token { token: TokenType::LeftBrace, .. }) = tokens.peek().cloned() {
        return parse_block(tokens);
    }

    if let Some(Token { token, pos }) = tokens.next() {
        let expr = match token {

            // ident or assign
            TokenType::Ident(name) => 
                match tokens.peek() {
                    Some(&Token { token: TokenType::Assign, .. }) => {
                        tokens.next();
                        TreeType::Assign(name, Box::new(parse_expr(tokens)))
                    },
                    _ => TreeType::Ident(name)
                },

            // number
            TokenType::NumLit(num) => 
                if let Ok(num) = num.parse::<i64>() {
                    TreeType::IntLit(num)
                } else {
                    TreeType::Error("expected integer number")
                },

            // string
            TokenType::StrLit(lit) => TreeType::StrLit(lit),

            // parenthesised expression (like '(a + b)') or tuple
            TokenType::LeftPar => {
                let tuple = parse_tuple(tokens);
                if let Some(Token { token: TokenType::RightPar, .. }) = tokens.next() {
                    tuple
                } else {
                    TreeType::Error("expected ')'")
                }
            },

            // conditional branch
            TokenType::If => {
                let cond = parse_expr(tokens);
                let thenp = parse_block(tokens);

                let elsep = if let Some(Token { token: TokenType::Else, .. }) = tokens.peek().cloned() {
                    tokens.next();
                    parse_block(tokens)
                } else {
                    TreeType::Empty.with_pos(pos.clone())
                };

                TreeType::If(Box::new(cond), Box::new(thenp), Box::new(elsep))
            },

            // definition
            TokenType::Let => {
                if let Some(Token { token: TokenType::Ident(name), .. }) = tokens.next() {
                    if let Some(Token { token: TokenType::Assign, .. }) = tokens.next() {
                        TreeType::Def(name, Box::new(parse_expr(tokens)))
                    } else {
                        TreeType::Error("expected '='")
                    }
                } else {
                    TreeType::Error("expected identifier")
                }
            },

            // parse error
            _ => TreeType::Error("expected expression or '('")
        };

        // if the expression is followed by a parenthesied expression convert it to a call
        if let Some(Token { token: TokenType::LeftPar, pos }) = tokens.peek().cloned() {
            let args = parse_simple_expr(tokens);
            TreeType::Call(Box::new(expr.with_pos(pos)), to_vec(args))
        } else {
            expr
        }.with_pos(pos)

    } else {
        eof_error()
    }
}

// parse a complex expression, (with binops and stuff)
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
                &TokenType::FatArrow => true,
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
            &TokenType::FatArrow => 0,
            &TokenType::Eq | &TokenType::Neq => 1,
            &TokenType::Plus | &TokenType::Minus => 2,
            &TokenType::Star | &TokenType::Slash => 3,
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

// parse a list of comma separated trees
fn parse_tuple<I: Iterator<Item = Token>>(tokens: &mut Peekable<I>) -> TreeType<Name> {
    fn is_end(token: &Option<Token>) -> bool {
        if let &Some(ref token) = token {
            match token.token {
                TokenType::RightPar | TokenType::RightBrace => true,
                _ => false
            }
        } else {
            true
        } 
    }

    fn is_comma(token: &Option<Token>) -> bool {
        if let Some(Token { token: TokenType::Comma, .. }) = *token {
            true 
        } else {
            false
        }
    }

    let mut elems = Vec::new();
    if is_end(&tokens.peek().cloned()) {
        return tuple_from_vec(elems);
    } else {
        elems.push(parse_expr(tokens));
    }

    loop {
        match tokens.peek().cloned() {
            ref t if is_end(&t) => return tuple_from_vec(elems),
            ref t if is_comma(&t) => {
                tokens.next();
                if is_end(&tokens.peek().cloned()) {
                    return TreeType::Tuple(elems);
                }
                elems.push(parse_expr(tokens));
            },
            t => {
                elems.push(TreeType::Error("expected ',', ')' or '}'").with_pos(error_pos(t)));
                return tuple_from_vec(elems);
            } 
        }  
    }
}



// ------------------------------------------- helpers -------------------------------------------

fn create_bin_op(op: Token, lhs: Tree<Name>, rhs: Tree<Name>) -> Tree<Name> {
    match op.token {
        TokenType::FatArrow => TreeType::Func(to_vec(lhs), Rc::new(rhs)),
        TokenType::Eq => TreeType::Eq(Box::new(lhs), Box::new(rhs)),
        TokenType::Neq => TreeType::Neq(Box::new(lhs), Box::new(rhs)),
        TokenType::Plus => TreeType::Add(Box::new(lhs), Box::new(rhs)),
        TokenType::Minus => TreeType::Sub(Box::new(lhs), Box::new(rhs)),
        TokenType::Star => TreeType::Mul(Box::new(lhs), Box::new(rhs)),
        TokenType::Slash => TreeType::Div(Box::new(lhs), Box::new(rhs)),
        _ => TreeType::Error("expected '+', '-', '*', '/', '==', '!=' or '=>'")
    }.with_pos(op.pos)
}

fn tuple_from_vec(elems: Vec<Tree<Name>>) -> TreeType<Name> {
    if FOLD_TUPLE_1 && elems.len() == 1 {
        return  elems.into_iter().next().unwrap().tree;
    } else {
        return TreeType::Tuple(elems);
    }
}

fn to_vec(tpl: Tree<Name>) -> Vec<Tree<Name>> {
    match tpl.tree {
        TreeType::Tuple(elems) => elems,
        _ => vec![tpl]
    }
}

fn eof_error() -> Tree<Name> {
    TreeType::Error("unexpected EOF").with_pos(Position::eof())
}

fn error_pos(tk: Option<Token>) -> Position {
    if let Some(Token { token: _, ref pos }) = tk {
        pos.clone()
    } else {
        Position::eof()
    }
}