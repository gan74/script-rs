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
        let shit = 7
        if 1 + 2 {
            let b = shit + 2
            shit = b
        } else { pwet }
        let x = { 7 }
    }"#;
    let mut tokenizer = Tokenizer::tokenize(input.chars());
    let tree = parse(&mut tokenizer);

    let errors = collect_errors(&tree);
    println!("{} errors:", errors.len());
    for err in errors {
        println!("{}\n{}", err, err.pos.pos_string(input));
    }

    println!("{}", tree);

    let mut env = Env::new();
    eval(&tree, &mut env);

    println!("{:?}", env);
}