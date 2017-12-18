use std::time::{Instant};

mod position;
mod tokenizer;
mod token;
mod tree;
mod parser;
mod eval;
mod value;

use parser::*;
use tree::*;
use eval::*;
use tokenizer::*;

fn collect_errors<'a>(tree: &'a Tree<String>) -> Vec<&'a Tree<String>> {
    let mut err: Vec<&'a Tree<String>> = Vec::new();
    tree.for_each_subtree(|t| if t.is_error() { err.push(t); });
    err
}

fn main() {
    let input = r#"{
        let fun = (a, b) => { a + b }
        let a = fun(1, 2)
        let b = fun
        let c = b(3, 0)
    }"#;
    let mut tokenizer = Tokenizer::tokenize(input.chars());
    let tree = parse(&mut tokenizer);

    let errors = collect_errors(&tree);
    println!("{} errors:", errors.len());
    for err in errors {
        println!("{}\n{}", err, err.pos.pos_string(input));
    }

    println!("{}", tree);

    let now = Instant::now();

    let mut env = Env::new();
    eval(&tree, &mut env);

    let duration = now.elapsed();

    println!("done in {}ms", (duration.as_secs() * 1000) as u32 + duration.subsec_nanos() / 1000000);
    println!("{:?}", env);
}