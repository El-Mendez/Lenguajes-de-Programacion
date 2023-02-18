use std::fmt::Debug;
use super::Visitable;
use crate::{UnaryOperator, BinaryOperator, Operator};
use crate::Symbol;
use super::tokenize::{to_postfix, tokenize_regular_expression, LexToken};


#[derive(Debug, PartialEq, Eq)]
pub enum LexTree {
    Binary { value: BinaryOperator, left_child: Box<LexTree>, right_child: Box<LexTree> },
    Unary { value: UnaryOperator, child: Box<LexTree> },
    Leaf { value: Symbol }
}

impl LexTree {
    fn from_reference(stack: &mut Vec<LexToken>) -> LexTree {
        match stack.pop().expect("Not enough tokens!") {
            LexToken::Symbol(value) =>
                LexTree::Leaf { value },

            LexToken::Operator(value) => {
                match value {
                    Operator::Binary(value) =>
                        LexTree::Binary {
                            value,
                            // Because of postfix, the first pop would return the right child,
                            right_child: LexTree::from_reference(stack).into(),
                            left_child: LexTree::from_reference(stack).into(),
                        },

                    Operator::Unary(value) =>
                        LexTree::Unary {
                            value,
                            child: LexTree::from_reference(stack).into(),
                        },

                    Operator::OpenParenthesis | Operator::CloseParenthesis =>
                        panic!("postfix expressions should not have parenthesis!")
                }
            }
        }
    }
}

impl<T> Visitable<T> for LexTree {}

impl From<Vec<LexToken>> for LexTree {
    fn from(mut value: Vec<LexToken>) -> Self {
        LexTree::from_reference(&mut value)
    }
}


impl From<&str> for LexTree {
    fn from(value: &str) -> Self {
        to_postfix(tokenize_regular_expression(value)).into()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_tree() {
        let expected = LexTree::Binary {
            value: BinaryOperator::Or,
            left_child: Box::from(LexTree::Leaf { value: Symbol::Character('a') }),
            right_child: Box::from(LexTree::Leaf { value: Symbol::Character('b') }),
        };

        assert_eq!(expected, LexTree::from("a|b"))
    }

    #[test]
    fn unary_tree() {
        let expected = LexTree::Unary {
            value: UnaryOperator::Kleene,
            child: Box::from(LexTree::Leaf { value: Symbol::Character('a') }),
        };

        assert_eq!(expected, LexTree::from("a*"))
    }

    #[test]
    fn complex_tree() {
        let expected = LexTree::Binary {
            value: BinaryOperator::Concat,
            right_child: Box::from(LexTree::Leaf { value: Symbol::Character('c') }),
            left_child: Box::from(LexTree::Binary {
                value: BinaryOperator::Or,
                left_child: Box::from(LexTree::Unary {
                    value: UnaryOperator::Kleene,
                    child: Box::from(LexTree::Leaf { value: Symbol::Character('a') })
                }),
                right_child: Box::from(LexTree::Unary {
                    value: UnaryOperator::Kleene,
                    child: Box::from(LexTree::Leaf { value: Symbol::Character('b') })
                })
            })
        };

        assert_eq!(expected, LexTree::from("(a*|b*)c"))
    }
}
