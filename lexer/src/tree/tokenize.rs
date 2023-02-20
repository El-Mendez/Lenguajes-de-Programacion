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

    let mut chars = input.chars();

    while let Some(c) = chars.next() {
        let new_token = match c {
            '(' => LexToken::Operator(OpenParenthesis),
            ')' => LexToken::Operator(CloseParenthesis),
            '|' => LexToken::Operator(Binary(Or)),
            '*' => LexToken::Operator(Unary(Kleene)),
            '?' => LexToken::Operator(Unary(Maybe)),
            '+' => LexToken::Operator(Unary(Many)),
            '\\' => LexToken::Symbol(Character(chars.next()
                .ok_or(LexError::MissingArgument)?)),
            'ε' => LexToken::Symbol(Epsilon),
            x => LexToken::Symbol(Character(x))
        };

        // add an explicit concatenation token when needed
        if matches!(new_token, LexToken::Symbol(_))
            || matches!(new_token, LexToken::Operator(OpenParenthesis)) {

            if let Some(old_token) = output.last() {
                if matches!(old_token, LexToken::Operator(Unary(_))) ||
                    matches!(old_token, LexToken::Symbol(_)) ||
                    *old_token == LexToken::Operator(CloseParenthesis) {

                    output.push(LexToken::Operator(Binary(Concat)))
                }
            }
        }

        output.push(new_token)
    }

    Ok(output)
}

pub fn to_postfix(input: Vec<LexToken>) -> Result<Vec<LexToken>, LexError> {
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
                            let last = stack.pop().ok_or(LexError::MissingOpeningParenthesis)?;
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
                            output.push(LexToken::Operator(stack.pop().unwrap()))
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
    Ok(output)
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
        let actual = to_postfix(input).unwrap();
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
