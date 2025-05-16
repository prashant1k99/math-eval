use std::cmp::Ordering;

use crate::tokenizer::{Operations, Tokens};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum ParsedElement {
    Atom(f64),
    CompoundItem(Compound),
}

#[derive(Debug, Clone)]
enum ParseList {
    El(ParsedElement),
    Op(Operations),
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Compound {
    pub operand1: Box<ParsedElement>,
    pub operand2: Box<ParsedElement>,
    pub operation: Operations,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Parser {
    root_compound: ParsedElement,
}

impl Parser {
    fn parse_exp(token_tree: &[Tokens]) -> Result<(ParsedElement, usize), &'static str> {
        // 1. There cannot be an operator in the very starting or at the very end
        // 2. If the open bracket appears, we need to call it recursively making it as a separate
        //    expression
        // 3. If the close bracket appears then simply return the AST partial tree
        // 4. If called recursively and there's no close bracket left then panic for invalid
        //    expression
        // 5. For bond priority check if the bond on left has higher or lower priority then the
        //    bond on the right. Based on the bond, the Compound will be created. If the bond on
        //    the right is higher than initial compound will be created for the right side of
        //    expression and the remaining compound with the left expression.
        // 6. If bond on both side has equal priority then give left one higher priority then the
        //    right one

        if token_tree.len() == 1 {
            if let Tokens::Operand(op) = token_tree[0] {
                return Ok((ParsedElement::Atom(op), 0));
            }
        } else if token_tree.len() < 2 {
            return Err("Invalid expression");
        }

        let mut compound_child_list: Vec<ParseList> = vec![];

        let mut current_index = 0;

        while current_index < token_tree.len() {
            let token = &token_tree[current_index];

            match token {
                Tokens::Operand(val) => {
                    compound_child_list.push(ParseList::El(ParsedElement::Atom(*val)))
                }
                Tokens::Expression(op) => compound_child_list.push(ParseList::Op(*op)),
                Tokens::BracketOpen => {
                    let (component, index_to_skip) =
                        Self::parse_exp(&token_tree[(current_index + 1)..])?;

                    compound_child_list.push(ParseList::El(component));

                    current_index += index_to_skip;

                    if let Tokens::BracketClose = &token_tree[current_index + 1] {
                        current_index += 1;
                    } else {
                        return Err("Invalid expression: Bracket Not Closed");
                    }
                }
                Tokens::BracketClose => break,
            }
            current_index += 1;
        }

        // Now start processing
        // First check 1st element and last element should not be Operand
        if let ParseList::Op(_) = compound_child_list[0].clone() {
            return Err("Invalid expression: expression cannot start with operator");
        } else if let ParseList::Op(_) = compound_child_list[compound_child_list.len() - 1].clone()
        {
            return Err("Invalid expression: expression cannot end with operator");
        }

        let mut current_processing_index = 0;
        while compound_child_list.len() >= 3 {
            if let (ParseList::El(operand1), ParseList::Op(operation1), ParseList::El(operand2)) = (
                compound_child_list[current_processing_index].clone(),
                compound_child_list[current_processing_index + 1].clone(),
                compound_child_list[current_processing_index + 2].clone(),
            ) {
                if compound_child_list.len() >= 4 {
                    if let ParseList::Op(operation2) =
                        compound_child_list[current_processing_index + 3].clone()
                    {
                        if operation1.cmp(&operation2) == Ordering::Less {
                            current_processing_index += 2;
                            continue;
                        } else {
                            compound_child_list.remove(current_processing_index);
                            compound_child_list.remove(current_processing_index);
                            compound_child_list.remove(current_processing_index);

                            compound_child_list.insert(
                                current_processing_index,
                                ParseList::El(ParsedElement::CompoundItem(Compound {
                                    operand1: Box::new(operand1),
                                    operand2: Box::new(operand2),
                                    operation: operation1,
                                })),
                            );
                            current_processing_index = 0;
                            continue;
                        }
                    } else {
                        return Err(
                            "Invalid expression: Order of expression invalid, Operand, Operation",
                        );
                    }
                } else {
                    return Ok((
                        ParsedElement::CompoundItem(Compound {
                            operand1: Box::new(operand1),
                            operand2: Box::new(operand2),
                            operation: operation1,
                        }),
                        current_index,
                    ));
                }
            } else {
                return Err(
                    "Invalid expression: Order of expression invalid, Operand, Operation, Operand",
                );
            }
            // Get elements - 0, 1, 2, Option(3)
            // If Some(3) then cmp
        }

        if let ParseList::El(comp) = compound_child_list[0].clone() {
            Ok((comp, current_index))
        } else {
            Err("Something went wrong while parsing")
        }
    }

    pub fn parse(token_tree: &[Tokens]) -> Self {
        match Self::parse_exp(token_tree) {
            Ok(val) => Self {
                root_compound: val.0,
            },
            Err(e) => panic!("Failed to parse {}", e),
        }
    }

    pub fn get(&self) -> ParsedElement {
        self.root_compound.clone()
    }
}
