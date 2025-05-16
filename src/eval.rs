use std::thread;

use crate::{parser::ParsedElement, tokenizer::Operations};

pub fn eval(exp: ParsedElement) -> f64 {
    match exp {
        ParsedElement::Atom(val) => val,
        ParsedElement::CompoundItem(comp) => {
            let left = *comp.operand1;
            let right = *comp.operand2;
            let left_val = thread::spawn(move || eval(left))
                .join()
                .expect("Expected valid f64");
            let right_val = thread::spawn(move || eval(right))
                .join()
                .expect("Expected valid f64");

            match comp.operation {
                Operations::Add => left_val + right_val,
                Operations::Subtract => left_val - right_val,
                Operations::Mul => left_val * right_val,
                Operations::Div => left_val / right_val,
            }
        }
    }
}
