use tree::*;
use tokenizer::*;
use parser;


type Name = String;

fn parse(input: &str) -> Tree<Name> {
    fn deblock(t: Tree<Name>) -> Tree<Name> {
        match t.tree_type {
            TreeType::Block(ref stats, ref expr) if stats.is_empty() => *expr.clone(),
            _ => t,
        }
    }
    deblock(parser::parse(&mut Tokenizer::tokenize(input.chars())))
}

fn parse_no_error(input: &str) -> Tree<Name> {
    let t = parse(input);
    assert!(error(&t).is_none());
    t
}

fn error(t: &Tree<Name>) -> Option<&Tree<Name>> {
    let mut err = None;
    t.for_each(|t| if err.is_none() && t.is_error() { err = Some(t); });
    err
}

fn is_error(input: &str) -> bool {
    error(&parse(input)).is_some()
}

fn tuple_len(input: &str) -> Option<usize> {
    match parse_no_error(input).as_tree_type() {
        TreeType::Tuple(e) => Some(e.len()),
        _ => None
    }
}

fn is_def(input: &str) -> bool {
    match parse_no_error(input).as_tree_type() {
        TreeType::Def(..) => true,
        _ => false
    }
}

fn is_call(input: &str) -> bool {
    match parse_no_error(input).as_tree_type() {
        TreeType::Call(..) => true,
        _ => false
    }
}

fn is_cond(input: &str) -> bool {
    match parse_no_error(input).as_tree_type() {
        TreeType::If(..) => true,
        _ => false
    }
}

fn is_add(input: &str) -> bool {
    match parse_no_error(input).as_tree_type() {
        TreeType::Add(..) => true,
        _ => false
    }
}

#[test]
fn parse_anon_call() {
    assert!(is_call("(x => x + 1)(7)"));
}

#[test]
fn parse_trailing_tuple() {
    match parse_no_error("{ let x = a()\n(1, 3) }").as_tree_type() {
        TreeType::Block(_, expr) => 
            match expr.as_tree_type() {
                TreeType::Tuple(e) => assert_eq!(e.len(), 2),
                _ => assert!(false)
            },
        _ => assert!(false)
    }
    assert!(is_call("{ a() (1, 3) }"));
    assert!(!is_call("{ a()\n(1, 3) }"));
}

#[test]
fn parse_call_precedence() {
    assert!(is_add("a() + b"));
    assert!(is_add("a + b()"));
    assert!(is_call("(a + b)()"));
    assert!(is_def("let a = b()"));
    assert!(is_def("let a = b()()"));
}

#[test]
fn parse_def() {
    assert!(is_def("let f = x => x + 1"));
}

#[test]
fn parse_tuple_2() {
    assert_eq!(tuple_len("(x, y)"), Some(2));
    assert_eq!(tuple_len("(3, y, )"), Some(2));
}

#[test]
fn parse_tuple_1() {
    assert_eq!(tuple_len("(x, )"), Some(1));
}

#[test]
fn parse_tuple_0() {
    assert_eq!(tuple_len("()"), Some(0));
}

#[test]
fn parse_paren_expr() {
    assert!(is_add("(x + 4)"));
}

#[test]
fn parse_if() {
    assert!(is_cond("if 1 2"));
}

#[test]
fn parse_invalid_empty_tuple() {
    assert!(is_error("(,)"));
}