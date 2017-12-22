use tree::*;
use tokenizer::*;
use position::*;
use parser;


type Name = String;


fn first_error(t: &Tree<Name>) -> Option<&Tree<Name>> {
    let mut err = None;
    t.for_each(|t| if err.is_none() && t.is_error() { err = Some(t); });
    err
}

fn parse_expr(input: &str) -> Tree<Name> {
    deblock(parse_block(input))
}

fn parse_block(input: &str) -> Tree<Name> {
    let tree = parser::parse(&mut Tokenizer::tokenize(input.chars()));
    if let Some(err) = first_error(&tree) {
        assert!(false, format!("\n{}\n{}\n", err, err.pos.pos_string(input)));
    }
    println!("{}", tree);
    tree
}

fn parse_err(input: &str) -> (&'static str, Position) {
    let tree = parser::parse(&mut Tokenizer::tokenize(input.chars()));
    let err = first_error(&tree);
    assert!(err.is_some(), format!("{:?} should have been an error", input));
    match err {
        Some(&Tree { tree: TreeType::Error(err), ref pos }) => (err, pos.clone()),
        _ => unreachable!()
    }
}

fn deblock(t: Tree<Name>) -> Tree<Name> {
    match t.tree {
        TreeType::Block(stats, expr) => {
            assert!(stats.is_empty(), "block is not an expression");
            *expr
        },
        _ => t,
    }
}

#[test]
fn parse_anon_call() {
    assert!(match parse_expr("(x => x + 1)(7)").tree {
        TreeType::Call(..) => true,
        _ => false
    });
}

/*#[test]
fn parse_trailing_tuple() {
    assert!(match parse_block("{ let x = a() (1, 3) }").tree {
        TreeType::Block(_, expr) => 
            match expr.tree {
                TreeType::Tuple(e) => e.len() == 2,
                _ => false
            },
        _ => false
    });
}*/

#[test]
fn parse_def() {
    assert!(match parse_expr("let f = x => x + 1").tree {
        TreeType::Def(..) => true,
        _ => false
    });
}

#[test]
fn parse_tuple_2() {
    assert!(match parse_expr("(x, y)").tree {
        TreeType::Tuple(e) => e.len() == 2,
        _ => false
    });

    assert!(match parse_expr("(3, y, )").tree {
        TreeType::Tuple(e) => e.len() == 2,
        _ => false
    });
}

#[test]
fn parse_tuple_1() {
    assert!(match parse_expr("(x, )").tree {
        TreeType::Tuple(e) => e.len() == 1,
        _ => false
    });
}

#[test]
fn parse_tuple_0() {
    assert!(match parse_expr("()").tree {
        TreeType::Tuple(e) => e.is_empty(),
        _ => false
    });
}

#[test]
fn parse_paren_expr() {
    assert!(match parse_expr("(x + 4)").tree {
        TreeType::Add(..) => true,
        _ => false
    });
}

#[test]
fn parse_if() {
    assert!(match parse_expr("if 1 2").tree {
        TreeType::If(..) => true,
        _ => false
    });
}

#[test]
fn parse_invalid_empty_tuple() {
    parse_err("(,)");
}