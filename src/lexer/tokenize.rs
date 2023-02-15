#[derive(Debug, Clone, Copy, PartialEq)]
enum Binary {
    Concat, Or
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Unary {
    Kleene
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Operator {
    Binary(Binary),
    Unary(Unary),
    OpenParenthesis,
    CloseParenthesis,
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

#[derive(Debug, Clone, Copy, PartialEq)]
enum RegularExpressionTokens {
    Operator(Operator),
    Symbol(char)
}

fn tokenize_regular_expression(input: &str) -> Vec<RegularExpressionTokens> {
    let mut output = Vec::new();

    let mut chars = input.chars();

    while let Some(c) = chars.next() {
        let new_token = match c {
            '(' => RegularExpressionTokens::Operator(Operator::OpenParenthesis),
            ')' => RegularExpressionTokens::Operator(Operator::CloseParenthesis),
            '|' => RegularExpressionTokens::Operator(Operator::Binary(Binary::Or)),
            '*' => RegularExpressionTokens::Operator(Operator::Unary(Unary::Kleene)),
            '\\' => RegularExpressionTokens::Symbol(chars.next()
                .expect("Expected an special character after: '\\'")),
            x => RegularExpressionTokens::Symbol(x)
        };

        // add an explicit concatenation token when needed
        if matches!(new_token, RegularExpressionTokens::Symbol(_))
            || matches!(new_token, RegularExpressionTokens::Operator(Operator::OpenParenthesis)) {

            if let Some(old_token) = output.last() {
                if matches!(old_token, RegularExpressionTokens::Operator(Operator::Unary(_))) ||
                    matches!(old_token, RegularExpressionTokens::Symbol(_)) ||
                    *old_token == RegularExpressionTokens::Operator(Operator::CloseParenthesis) {

                    output.push(RegularExpressionTokens::Operator(Operator::Binary(Binary::Concat)))
                }
            }
        }

        output.push(new_token)
    }

    output
}

fn to_postfix(input: Vec<RegularExpressionTokens>) -> Vec<RegularExpressionTokens> {
    let mut output: Vec<RegularExpressionTokens> = Vec::new();
    let mut stack: Vec<Operator> = Vec::new();

    for token in input {
        match token {
            RegularExpressionTokens::Symbol(x) => output.push(RegularExpressionTokens::Symbol(x)),
            RegularExpressionTokens::Operator(operation) => {
                match operation {
                    Operator::OpenParenthesis => stack.push(Operator::OpenParenthesis),
                    Operator::CloseParenthesis => {
                        loop {
                            let last = stack.pop().expect("Missing Opening Parenthesis");
                            if last == Operator::OpenParenthesis {
                                break;
                            }
                            output.push(RegularExpressionTokens::Operator(last))
                        }
                    },

                    _ => {
                        while let Some(other) = stack.last() {
                            if operation.order() > other.order() {
                                break
                            }
                            output.push(RegularExpressionTokens::Operator(stack.pop().unwrap()))
                        }
                        stack.push(operation)
                    }
                }
            }
        }
    }

    while let Some(operation) = stack.pop() {
        output.push(RegularExpressionTokens::Operator(operation))
    }
    output
}

#[cfg(test)]
mod tests {
    use crate::lexer::tokenize::{RegularExpressionTokens, to_postfix, tokenize_regular_expression};
    use crate::lexer::tokenize::Operator::{CloseParenthesis, OpenParenthesis, Unary, Binary};
    use crate::lexer::tokenize::Unary::Kleene;
    use crate::lexer::tokenize::Binary::{Concat, Or};

    #[test]
    fn tokenization() {
        let actual = tokenize_regular_expression(r"abc*\*|\|(d)\(\)");
        let expected = vec![
            RegularExpressionTokens::Symbol('a'),
            RegularExpressionTokens::Operator(Binary(Concat)),
            RegularExpressionTokens::Symbol('b'),
            RegularExpressionTokens::Operator(Binary(Concat)),
            RegularExpressionTokens::Symbol('c'),
            RegularExpressionTokens::Operator(Unary(Kleene)),
            RegularExpressionTokens::Operator(Binary(Concat)),
            RegularExpressionTokens::Symbol('*'),
            RegularExpressionTokens::Operator(Binary(Or)),
            RegularExpressionTokens::Symbol('|'),
            RegularExpressionTokens::Operator(Binary(Concat)),
            RegularExpressionTokens::Operator(OpenParenthesis),
            RegularExpressionTokens::Symbol('d'),
            RegularExpressionTokens::Operator(CloseParenthesis),
            RegularExpressionTokens::Operator(Binary(Concat)),
            RegularExpressionTokens::Symbol('('),
            RegularExpressionTokens::Operator(Binary(Concat)),
            RegularExpressionTokens::Symbol(')'),
        ];

        assert_eq!(expected, actual);
    }

    #[test]
    fn postfix() {
        let input = tokenize_regular_expression("(a|b)(c|d)*e");
        let actual = to_postfix(input);
        let expected = vec![
            RegularExpressionTokens::Symbol('a'),
            RegularExpressionTokens::Symbol('b'),
            RegularExpressionTokens::Operator(Binary(Or)),
            RegularExpressionTokens::Symbol('c'),
            RegularExpressionTokens::Symbol('d'),
            RegularExpressionTokens::Operator(Binary(Or)),
            RegularExpressionTokens::Operator(Unary(Kleene)),
            RegularExpressionTokens::Operator(Binary(Concat)),
            RegularExpressionTokens::Symbol('e'),
            RegularExpressionTokens::Operator(Binary(Concat)),
        ];

        assert_eq!(expected, actual);
    }
}
