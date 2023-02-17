use super::operator::UnaryOperator::*;
use super::operator::BinaryOperator::*;
use super::operator::Operator::*;
use super::symbols::Symbol::*;
use super::symbols::Symbol;
use super::operator::Operator;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReToken {
    Operator(Operator),
    Symbol(Symbol)
}

pub fn tokenize_regular_expression(input: &str) -> Vec<ReToken> {
    let mut output = Vec::new();

    let mut chars = input.chars();

    while let Some(c) = chars.next() {
        let new_token = match c {
            '(' => ReToken::Operator(OpenParenthesis),
            ')' => ReToken::Operator(CloseParenthesis),
            '|' => ReToken::Operator(Binary(Or)),
            '*' => ReToken::Operator(Unary(Kleene)),
            '?' => ReToken::Operator(Unary(Maybe)),
            '+' => ReToken::Operator(Unary(Many)),
            '\\' => ReToken::Symbol(Character(
                chars.next()
                .expect("Expected an special character after: '\\'"))),
            'ε' => ReToken::Symbol(Epsilon),
            x => ReToken::Symbol(Character(x))
        };

        // add an explicit concatenation token when needed
        if matches!(new_token, ReToken::Symbol(_))
            || matches!(new_token, ReToken::Operator(OpenParenthesis)) {

            if let Some(old_token) = output.last() {
                if matches!(old_token, ReToken::Operator(Unary(_))) ||
                    matches!(old_token, ReToken::Symbol(_)) ||
                    *old_token == ReToken::Operator(CloseParenthesis) {

                    output.push(ReToken::Operator(Binary(Concat)))
                }
            }
        }

        output.push(new_token)
    }

    output
}

pub fn to_postfix(input: Vec<ReToken>) -> Vec<ReToken> {
    let mut output: Vec<ReToken> = Vec::new();
    let mut stack: Vec<Operator> = Vec::new();

    for token in input {
        match token {
            ReToken::Symbol(x) => output.push(ReToken::Symbol(x)),
            ReToken::Operator(operation) => {
                match operation {
                    OpenParenthesis => stack.push(OpenParenthesis),
                    CloseParenthesis => {
                        loop {
                            let last = stack.pop().expect("Missing Opening Parenthesis");
                            if last == OpenParenthesis {
                                break;
                            }
                            output.push(ReToken::Operator(last))
                        }
                    },

                    _ => {
                        while let Some(other) = stack.last() {
                            if operation.order() > other.order() {
                                break
                            }
                            output.push(ReToken::Operator(stack.pop().unwrap()))
                        }
                        stack.push(operation)
                    }
                }
            }
        }
    }

    while let Some(operation) = stack.pop() {
        output.push(ReToken::Operator(operation))
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenization() {
        let actual = tokenize_regular_expression(r"abc*\*|\|(d)\(\)");
        let expected = vec![
            ReToken::Symbol(Character('a')),
            ReToken::Operator(Binary(Concat)),
            ReToken::Symbol(Character('b')),
            ReToken::Operator(Binary(Concat)),
            ReToken::Symbol(Character('c')),
            ReToken::Operator(Unary(Kleene)),
            ReToken::Operator(Binary(Concat)),
            ReToken::Symbol(Character('*')),
            ReToken::Operator(Binary(Or)),
            ReToken::Symbol(Character('|')),
            ReToken::Operator(Binary(Concat)),
            ReToken::Operator(OpenParenthesis),
            ReToken::Symbol(Character('d')),
            ReToken::Operator(CloseParenthesis),
            ReToken::Operator(Binary(Concat)),
            ReToken::Symbol(Character('(')),
            ReToken::Operator(Binary(Concat)),
            ReToken::Symbol(Character(')')),
        ];

        assert_eq!(expected, actual);
    }

    #[test]
    fn postfix() {
        let input = tokenize_regular_expression("(a|b)(c|d)*e");
        let actual = to_postfix(input);
        let expected = vec![
            ReToken::Symbol(Character('a')),
            ReToken::Symbol(Character('b')),
            ReToken::Operator(Binary(Or)),
            ReToken::Symbol(Character('c')),
            ReToken::Symbol(Character('d')),
            ReToken::Operator(Binary(Or)),
            ReToken::Operator(Unary(Kleene)),
            ReToken::Operator(Binary(Concat)),
            ReToken::Symbol(Character('e')),
            ReToken::Operator(Binary(Concat)),
        ];

        assert_eq!(expected, actual);
    }
}
