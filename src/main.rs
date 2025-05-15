use std::io;

mod tokenizer;

use tokenizer::Tokenizer;

fn main() {
    let mut input = String::new();
    loop {
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let tokens = Tokenizer::tokenize(&input);
        println!("Token Tree: {:?}", tokens.get());

        input.clear();
    }
}
