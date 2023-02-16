use std::fmt::Debug;
use super::operator::{UnaryOperator, BinaryOperator, Operator};
use super::symbols::{Symbol};
use super::tokenize::{to_postfix, tokenize_regular_expression, ReToken};
use super::visitor::Visitor;


#[derive(Debug, PartialEq, Eq)]
pub enum ReNode {
    Binary { value: BinaryOperator, left_child: Box<ReNode>, right_child: Box<ReNode> },
    Unary { value: UnaryOperator, child: Box<ReNode> },
    Leaf { value: Symbol }
}

impl ReNode {
    fn from_reference(stack: &mut Vec<ReToken>) -> ReNode {
        match stack.pop().expect("Not enough tokens!") {
            ReToken::Symbol(value) =>
                ReNode::Leaf { value },

            ReToken::Operator(value) => {
                match value {
                    Operator::Binary(value) =>
                        ReNode::Binary {
                            value,
                            // Because of postfix, the first pop would return the right child,
                            right_child: ReNode::from_reference(stack).into(),
                            left_child: ReNode::from_reference(stack).into(),
                        },

                    Operator::Unary(value) =>
                        ReNode::Unary {
                            value,
                            child: ReNode::from_reference(stack).into(),
                        },

                    Operator::OpenParenthesis | Operator::CloseParenthesis =>
                        panic!("postfix expressions should not have parenthesis!")
                }
            }
        }
    }

    pub fn accept<T>(&self, visitor: &mut impl Visitor<T>) -> T {
        visitor.visit(&self)
    }
}

impl From<Vec<ReToken>> for ReNode {
    fn from(mut value: Vec<ReToken>) -> Self {
        ReNode::from_reference(&mut value)
    }
}


impl From<&str> for ReNode {
    fn from(value: &str) -> Self {
        to_postfix(tokenize_regular_expression(value)).into()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_tree() {
        let expected = ReNode::Binary {
            value: BinaryOperator::Or,
            left_child: Box::from(ReNode::Leaf { value: Symbol::Character('a') }),
            right_child: Box::from(ReNode::Leaf { value: Symbol::Character('b') }),
        };

        assert_eq!(expected, ReNode::from("a|b"))
    }

    #[test]
    fn unary_tree() {
        let expected = ReNode::Unary {
            value: UnaryOperator::Kleene,
            child: Box::from(ReNode::Leaf { value: Symbol::Character('a') }),
        };

        assert_eq!(expected, ReNode::from("a*"))
    }

    #[test]
    fn complex_tree() {
        let expected = ReNode::Binary {
            value: BinaryOperator::Concat,
            right_child: Box::from(ReNode::Leaf { value: Symbol::Character('c') }),
            left_child: Box::from(ReNode::Binary {
                value: BinaryOperator::Or,
                left_child: Box::from(ReNode::Unary {
                    value: UnaryOperator::Kleene,
                    child: Box::from(ReNode::Leaf { value: Symbol::Character('a') })
                }),
                right_child: Box::from(ReNode::Unary {
                    value: UnaryOperator::Kleene,
                    child: Box::from(ReNode::Leaf { value: Symbol::Character('b') })
                })
            })
        };

        assert_eq!(expected, ReNode::from("(a*|b*)c"))
    }
}
