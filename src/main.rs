use regex::Regex;
use crate::Token::{OperationToken, OperandToken};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operator {
    OpenParenthesis, CloseParenthesis, // No priority
    Multiplicate, Divide,              // Mid priority
    Sum, Subtract,                     // Least Priority
}

impl Operator {
    fn higher_order(self, other: &Operator) -> bool {
        ((self == Operator::Multiplicate || self == Operator::Divide) &&
            (*other == Operator::Subtract || *other == Operator::Sum)) ||
            *other == Operator::OpenParenthesis
    }
}

#[derive(Debug, Clone, Copy)]
enum Token {
    OperationToken(Operator),
    OperandToken(isize),
}


fn infix_to_posix(input: Vec<Token>) -> Vec<Token> {
    let mut output: Vec<Token> = Vec::new();
    let mut stack: Vec<Operator> = Vec::new();

    for token in input {
        match token {
            OperandToken(x) => output.push(OperandToken(x)),
            OperationToken(operation) => {
                match operation {
                    Operator::OpenParenthesis => stack.push(Operator::OpenParenthesis),
                    Operator::CloseParenthesis => {
                        loop {
                            let last = stack.pop().expect("Missing Opening parenthesis");
                            if last == Operator::OpenParenthesis {
                                break
                            }
                            output.push(OperationToken(last));
                        }
                    }

                    _ => {
                        while let Some(other) = stack.last() {
                            if operation.higher_order(other) {
                                break
                            }
                            output.push(OperationToken(stack.pop().unwrap()))
                        }
                        stack.push(operation);
                    }
                }
            },
        }
    }

    while let Some(operation) = stack.pop() {
        output.push(OperationToken(operation));
    }

    output
}

fn show(input: &[Token]) -> String {
    let mut string: String = input
        .iter()
        .map(|token| {
            match token {
                OperandToken(x) => x.to_string(),
                OperationToken(operation) => {
                    match *operation {
                        Operator::OpenParenthesis => "(",
                        Operator::CloseParenthesis => ")",
                        Operator::Multiplicate => "*",
                        Operator::Divide => "/",
                        Operator::Sum => "+",
                        Operator::Subtract => "-",
                    }.to_string()
                }
            }
        })
        .map(|x| x + ", ")
        .collect();

    string.pop();
    string.pop();
    string
}

fn from_string(input: &str) -> Vec<Token> {
    let expression = Regex::new(r"(\d+|\+|-|/|\*|\(|\))").expect("NOT A VALID REGEX");
    let mut output = Vec::new();

    for c in expression.find_iter(input) {
        match c.as_str() {
            "(" => output.push(OperationToken(Operator::OpenParenthesis)),
            ")" => output.push(OperationToken(Operator::CloseParenthesis)),
            "*" => output.push(OperationToken(Operator::Multiplicate)),
            "/" => output.push(OperationToken(Operator::Divide)),
            "+" => output.push(OperationToken(Operator::Sum)),
            "-" => output.push(OperationToken(Operator::Subtract)),
            x => output.push(OperandToken(x.parse().unwrap())),
        }
    };
    output
}

fn eval_postfix(input: Vec<Token>) -> isize {
    let mut stack = Vec::new();

    for token in input {
        match token {
            OperandToken(x) => stack.push(x),
            OperationToken(operation) => {
                let op2 = stack.pop().unwrap();
                let op1 = stack.pop().unwrap();

                let result = match operation {
                    Operator::Multiplicate => op1 * op2,
                    Operator::Divide => op1 / op2,
                    Operator::Sum => op1 + op2,
                    Operator::Subtract => op1 - op2,
                    _ => panic!("postfix should not have parenthesis!")
                };

                stack.push(result);
            }
        }
    }

    stack.pop().unwrap()
}

fn main() {
    let input = "27+45-33*(5*8+33)-5/4".to_string();
    println!("input {:?}", input);

    let tokens = from_string(&input);
    println!("tokens {:?}", &tokens);

    let postfix = infix_to_posix(tokens);
    println!("postfix {:?}", show(&postfix));

    let result = eval_postfix(postfix);
    println!("result {:?}", result);
}
