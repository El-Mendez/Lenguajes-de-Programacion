#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Binary {
    Concat, Or
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Unary {
    Kleene
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    Binary(Binary),
    Unary(Unary),
    OpenParenthesis,
    CloseParenthesis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Symbol {
    Epsilon,
    Character(char)
}

impl Operator {
    fn order(self) -> usize {
        match self {
            Operator::Unary(operation) => {
                match operation {
                    Unary::Kleene => 3,
                }
            }
            Operator::Binary(operation) => {
                match operation {
                    Binary::Concat => 2,
                    Binary::Or => 1,
                }
            }
            Operator::OpenParenthesis | Operator::CloseParenthesis => 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegularExpressionToken {
    Operator(Operator),
    Symbol(Symbol)
}

pub fn tokenize_regular_expression(input: &str) -> Vec<RegularExpressionToken> {
    let mut output = Vec::new();

    let mut chars = input.chars();

    while let Some(c) = chars.next() {
        let new_token = match c {
            '(' => RegularExpressionToken::Operator(Operator::OpenParenthesis),
            ')' => RegularExpressionToken::Operator(Operator::CloseParenthesis),
            '|' => RegularExpressionToken::Operator(Operator::Binary(Binary::Or)),
            '*' => RegularExpressionToken::Operator(Operator::Unary(Unary::Kleene)),
            '\\' => RegularExpressionToken::Symbol(Symbol::Character(
                chars.next()
                .expect("Expected an special character after: '\\'"))),
            'ε' => RegularExpressionToken::Symbol(Symbol::Epsilon),
            x => RegularExpressionToken::Symbol(Symbol::Character(x))
        };

        // add an explicit concatenation token when needed
        if matches!(new_token, RegularExpressionToken::Symbol(_))
            || matches!(new_token, RegularExpressionToken::Operator(Operator::OpenParenthesis)) {

            if let Some(old_token) = output.last() {
                if matches!(old_token, RegularExpressionToken::Operator(Operator::Unary(_))) ||
                    matches!(old_token, RegularExpressionToken::Symbol(_)) ||
                    *old_token == RegularExpressionToken::Operator(Operator::CloseParenthesis) {

                    output.push(RegularExpressionToken::Operator(Operator::Binary(Binary::Concat)))
                }
            }
        }

        output.push(new_token)
    }

    output
}

pub fn to_postfix(input: Vec<RegularExpressionToken>) -> Vec<RegularExpressionToken> {
    let mut output: Vec<RegularExpressionToken> = Vec::new();
    let mut stack: Vec<Operator> = Vec::new();

    for token in input {
        match token {
            RegularExpressionToken::Symbol(x) => output.push(RegularExpressionToken::Symbol(x)),
            RegularExpressionToken::Operator(operation) => {
                match operation {
                    Operator::OpenParenthesis => stack.push(Operator::OpenParenthesis),
                    Operator::CloseParenthesis => {
                        loop {
                            let last = stack.pop().expect("Missing Opening Parenthesis");
                            if last == Operator::OpenParenthesis {
                                break;
                            }
                            output.push(RegularExpressionToken::Operator(last))
                        }
                    },

                    _ => {
                        while let Some(other) = stack.last() {
                            if operation.order() > other.order() {
                                break
                            }
                            output.push(RegularExpressionToken::Operator(stack.pop().unwrap()))
                        }
                        stack.push(operation)
                    }
                }
            }
        }
    }

    while let Some(operation) = stack.pop() {
        output.push(RegularExpressionToken::Operator(operation))
    }
    output
}

#[cfg(test)]
mod tests {
    use crate::lexer::tokenize::{RegularExpressionToken, to_postfix, tokenize_regular_expression};
    use crate::lexer::tokenize::Operator::{CloseParenthesis, OpenParenthesis, Unary, Binary};
    use crate::lexer::tokenize::Unary::Kleene;
    use crate::lexer::tokenize::Binary::{Concat, Or};
    use crate::lexer::tokenize::Symbol::Character;

    #[test]
    fn tokenization() {
        let actual = tokenize_regular_expression(r"abc*\*|\|(d)\(\)");
        let expected = vec![
            RegularExpressionToken::Symbol(Character('a')),
            RegularExpressionToken::Operator(Binary(Concat)),
            RegularExpressionToken::Symbol(Character('b')),
            RegularExpressionToken::Operator(Binary(Concat)),
            RegularExpressionToken::Symbol(Character('c')),
            RegularExpressionToken::Operator(Unary(Kleene)),
            RegularExpressionToken::Operator(Binary(Concat)),
            RegularExpressionToken::Symbol(Character('*')),
            RegularExpressionToken::Operator(Binary(Or)),
            RegularExpressionToken::Symbol(Character('|')),
            RegularExpressionToken::Operator(Binary(Concat)),
            RegularExpressionToken::Operator(OpenParenthesis),
            RegularExpressionToken::Symbol(Character('d')),
            RegularExpressionToken::Operator(CloseParenthesis),
            RegularExpressionToken::Operator(Binary(Concat)),
            RegularExpressionToken::Symbol(Character('(')),
            RegularExpressionToken::Operator(Binary(Concat)),
            RegularExpressionToken::Symbol(Character(')')),
        ];

        assert_eq!(expected, actual);
    }

    #[test]
    fn postfix() {
        let input = tokenize_regular_expression("(a|b)(c|d)*e");
        let actual = to_postfix(input);
        let expected = vec![
            RegularExpressionToken::Symbol(Character('a')),
            RegularExpressionToken::Symbol(Character('b')),
            RegularExpressionToken::Operator(Binary(Or)),
            RegularExpressionToken::Symbol(Character('c')),
            RegularExpressionToken::Symbol(Character('d')),
            RegularExpressionToken::Operator(Binary(Or)),
            RegularExpressionToken::Operator(Unary(Kleene)),
            RegularExpressionToken::Operator(Binary(Concat)),
            RegularExpressionToken::Symbol(Character('e')),
            RegularExpressionToken::Operator(Binary(Concat)),
        ];

        assert_eq!(expected, actual);
    }
}
