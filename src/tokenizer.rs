#[derive(Debug)]
pub enum Operations {
    Subtract = 1,
    Add = 2,
    Mul = 3,
    Div = 4,
}

#[derive(Debug)]
pub enum Tokens {
    Operand(f64),
    Expression(Operations),
    BracketOpen,
    BracketClose,
}

#[derive(Debug)]
pub struct Tokenizer {
    token_tree: Vec<Tokens>,
}

impl Tokenizer {
    pub fn tokenize(input: &str) -> Self {
        let mut token_tree: Vec<Tokens> = Vec::new();

        let char_iter = input.trim().chars();

        let mut current_count = String::new();

        for char in char_iter {
            if char.is_ascii_digit() || char == '.' {
                current_count.push(char);
                continue;
            } else if !current_count.is_empty() {
                let operand = current_count
                    .parse::<f64>()
                    .expect("Failed to parse operand");
                token_tree.push(Tokens::Operand(operand));
                current_count.clear();
            }
            let val = match char {
                '+' => Tokens::Expression(Operations::Add),
                '-' => Tokens::Expression(Operations::Subtract),
                '*' => Tokens::Expression(Operations::Mul),
                '/' => Tokens::Expression(Operations::Div),
                '(' => Tokens::BracketOpen,
                ')' => Tokens::BracketClose,
                ' ' => {
                    continue;
                }
                unknown_char => {
                    panic!("Unknown expression: {}", unknown_char)
                }
            };
            token_tree.push(val);
        }

        if !current_count.is_empty() {
            let operand = current_count
                .parse::<f64>()
                .expect("Failed to parse operand");
            token_tree.push(Tokens::Operand(operand));
        }

        Self { token_tree }
    }

    pub fn get(&self) -> &Vec<Tokens> {
        &self.token_tree
    }
}
