use std::io::{self, Write};

use eval::eval;

mod eval;
mod parser;
mod tokenizer;

fn main() {
    let mut input = String::new();
    loop {
        print!(">> ");
        io::stdout().flush().expect("Failed to flush stdout");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let tokens = tokenizer::Tokenizer::tokenize(&input);
        let token_tree = tokens.get();
        let ast = parser::Parser::parse(token_tree);

        let result = eval(ast.get());
        println!("{result}");

        input.clear();
    }
}
