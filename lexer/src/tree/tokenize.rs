use crate::UnaryOperator::*;
use crate::BinaryOperator::*;
use crate::Operator::*;
use crate::{Symbol, Operator, LexError};
use Symbol::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LexToken {
    Operator(Operator),
    Symbol(Symbol)
}

pub fn tokenize_regular_expression(input: &str) -> Result<Vec<LexToken>, LexError> {
    let mut output = Vec::new();

    let mut add_concat = false;
    let mut last_was_binary_operation = false;
    let mut must_be_char = false;
    let mut parenthesis_levels = Vec::new();

    for (index, c) in input.chars().enumerate() {
        let new_token = if must_be_char {
            must_be_char = false;
            LexToken::Symbol(Character(c))
        } else {
            match c {
                '(' => LexToken::Operator(OpenParenthesis),
                ')' => LexToken::Operator(CloseParenthesis),
                '|' => LexToken::Operator(Binary(Or)),
                '*' => LexToken::Operator(Unary(Kleene)),
                '?' => LexToken::Operator(Unary(Maybe)),
                '+' => LexToken::Operator(Unary(Many)),
                '\\' => {
                    must_be_char = true;
                    continue;
                }
                'ε' => LexToken::Symbol(Epsilon),
                x => LexToken::Symbol(Character(x))
            }
        };

        match &new_token {
            LexToken::Operator(operator) => {
                match *operator {
                    Binary(_) => {
                        if !add_concat {
                            return Err(LexError::MissingArgument(index, input.to_string()));
                        }
                        add_concat = false;
                        last_was_binary_operation = true;
                    },
                    Unary(_) => {
                        // if we should not add concat, then we must've started the string or just
                        // opened a parenthesis. An unary operator after that is error either way
                        if last_was_binary_operation || !add_concat {
                            return Err(LexError::MissingArgument(index, input.to_string()));
                        }
                    },
                    OpenParenthesis => {
                        if add_concat {
                            output.push(LexToken::Operator(Binary(Concat)));
                        }
                        last_was_binary_operation = false;
                        add_concat = false;
                        parenthesis_levels.push(index);
                    },
                    CloseParenthesis => {
                        if last_was_binary_operation {
                            return Err(LexError::MissingArgument(index, input.to_string()));
                        }

                        parenthesis_levels.pop()
                            .ok_or(LexError::MissingOpeningParenthesis(index, input.to_string()))?;

                        if !add_concat {
                            // we must've just opened a parenthesis. just add epsilon here and done.
                            output.push(LexToken::Symbol(Epsilon));
                            add_concat = true;
                        }
                    },
                }
            }
            LexToken::Symbol(_) => {
                if add_concat {
                    output.push(LexToken::Operator(Binary(Concat)))
                }
                add_concat = true;
                last_was_binary_operation = false;
            }
        }

        output.push(new_token);
    }

    if last_was_binary_operation | must_be_char {
        return Err(LexError::MissingArgument(input.len()-1, input.to_string()));
    }

    if let Some(x) = parenthesis_levels.pop() {
        return Err(LexError::MissingClosingParenthesis(x, input.to_string()));
    }

    if output.len() == 0 {
        output.push(LexToken::Symbol(Epsilon))
    }


    Ok(output)
}

pub fn to_postfix(input: Vec<LexToken>) -> Vec<LexToken> {
    let mut output: Vec<LexToken> = Vec::new();
    let mut stack: Vec<Operator> = Vec::new();

    for token in input {
        match token {
            LexToken::Symbol(x) => output.push(LexToken::Symbol(x)),
            LexToken::Operator(operation) => {
                match operation {
                    OpenParenthesis => stack.push(OpenParenthesis),
                    CloseParenthesis => {
                        loop {
                            let last = stack.pop().expect("the stack cannot be empty!");
                            if last == OpenParenthesis {
                                break;
                            }
                            output.push(LexToken::Operator(last))
                        }
                    },

                    _ => {
                        while let Some(other) = stack.last() {
                            if operation.order() > other.order() {
                                break
                            }
                            output.push(LexToken::Operator(stack.pop().expect("There cannot be a missing parenthesis!")))
                        }
                        stack.push(operation)
                    }
                }
            }
        }
    }

    while let Some(operation) = stack.pop() {
        output.push(LexToken::Operator(operation))
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenization() {
        let actual = tokenize_regular_expression(r"abc*\*|\|(d)\(\)").unwrap();
        let expected = vec![
            LexToken::Symbol(Character('a')),
            LexToken::Operator(Binary(Concat)),
            LexToken::Symbol(Character('b')),
            LexToken::Operator(Binary(Concat)),
            LexToken::Symbol(Character('c')),
            LexToken::Operator(Unary(Kleene)),
            LexToken::Operator(Binary(Concat)),
            LexToken::Symbol(Character('*')),
            LexToken::Operator(Binary(Or)),
            LexToken::Symbol(Character('|')),
            LexToken::Operator(Binary(Concat)),
            LexToken::Operator(OpenParenthesis),
            LexToken::Symbol(Character('d')),
            LexToken::Operator(CloseParenthesis),
            LexToken::Operator(Binary(Concat)),
            LexToken::Symbol(Character('(')),
            LexToken::Operator(Binary(Concat)),
            LexToken::Symbol(Character(')')),
        ];

        assert_eq!(expected, actual);
    }

    #[test]
    fn postfix() {
        let input = tokenize_regular_expression("(a|b)(c|d)*e").unwrap();
        let actual = to_postfix(input);
        let expected = vec![
            LexToken::Symbol(Character('a')),
            LexToken::Symbol(Character('b')),
            LexToken::Operator(Binary(Or)),
            LexToken::Symbol(Character('c')),
            LexToken::Symbol(Character('d')),
            LexToken::Operator(Binary(Or)),
            LexToken::Operator(Unary(Kleene)),
            LexToken::Operator(Binary(Concat)),
            LexToken::Symbol(Character('e')),
            LexToken::Operator(Binary(Concat)),
        ];

        assert_eq!(expected, actual);
    }
}
