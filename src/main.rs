use std::time::Instant;

mod position;
mod tokenizer;
mod token;
mod tree;
mod parser;
mod eval;
mod value;
mod map_in_place;
mod tests;

use tree::*;
use eval::*;
use tokenizer::*;

fn collect_errors<'a>(tree: &'a Tree<String>) -> Vec<&'a Tree<String>> {
    let mut err: Vec<&'a Tree<String>> = Vec::new();
    tree.for_each(|t| if t.is_error() { err.push(t); });
    err
}

fn print_errors(input: &str, tree: &Tree<String>) {
    let errors = collect_errors(tree);
    if !errors.is_empty() {
        println!("{} errors:", errors.len());
        for err in errors {
            println!("{}\n{}", err, err.pos.pos_string(input));
        }
    }
}

fn parse(input: &str) -> Tree<String> {
    let mut tokenizer = Tokenizer::tokenize(input.chars());
    let tree = parser::parse(&mut tokenizer);
    print_errors(input, &tree);
    tree
}


fn main() {
    println!("{}", parse("{ let x = a() (1, 3) }"));

    /*let input = r#"{
        let facto = (n, rec) =>
            if n == 1 {
                1
            } else {
                rec(n - 1, rec) * n
            }
        let x = facto(6, facto)
        let y = ((a, b) => a + b) (x, 1)
    }"#;
    let mut tokenizer = Tokenizer::tokenize(input.chars());
    let mut tree = parse(&mut tokenizer);


    /*tree = tree.transform(|x| {
        match x {
            TreeType::Mul(lhs, rhs) => TreeType::Add(lhs, rhs),
            t => t
        }
    });*/

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

    println!("\ndone in {}ms", (duration.as_secs() * 1000) as u32 + duration.subsec_nanos() / 1000000);
    println!("\n{:?}", env);*/
}