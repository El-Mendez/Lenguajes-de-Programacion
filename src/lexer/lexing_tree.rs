use std::fmt::Debug;
use crate::lexer::tokenize::{Binary, RegularExpressionToken, Operator, Unary, tokenize_regular_expression, to_postfix, Symbol};

pub trait Visitor<T> {
    fn visit(&mut self, node: &LexingTreeNode) -> T;
}

#[derive(Debug, PartialEq, Eq)]
pub enum LexingTreeNode {
    Binary { value: Binary, left_child: Box<LexingTreeNode>, right_child: Box<LexingTreeNode> },
    Unary { value: Unary, child: Box<LexingTreeNode> },
    Leaf { value: Symbol }
}

impl LexingTreeNode {
    fn from_reference(stack: &mut Vec<RegularExpressionToken>) -> LexingTreeNode {
        match stack.pop().expect("Not enough tokens!") {
            RegularExpressionToken::Symbol(value) =>
                LexingTreeNode::Leaf { value },

            RegularExpressionToken::Operator(value) => {
                match value {
                    Operator::Binary(value) =>
                        LexingTreeNode::Binary {
                            value,
                            // Because of postfix, the first pop would return the right child,
                            right_child: LexingTreeNode::from_reference(stack).into(),
                            left_child: LexingTreeNode::from_reference(stack).into(),
                        },

                    Operator::Unary(value) =>
                        LexingTreeNode::Unary {
                            value,
                            child: LexingTreeNode::from_reference(stack).into(),
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

impl From<Vec<RegularExpressionToken>> for LexingTreeNode {
    fn from(mut value: Vec<RegularExpressionToken>) -> Self {
        LexingTreeNode::from_reference(&mut value)
    }
}


impl From<&str> for LexingTreeNode {
    fn from(value: &str) -> Self {
        to_postfix(tokenize_regular_expression(value)).into()
    }
}


#[cfg(test)]
mod tests {
    use crate::lexer::lexing_tree::LexingTreeNode;
    use crate::lexer::tokenize::{Binary, Symbol, Unary};

    #[test]
    fn binary_tree() {
        let expected = LexingTreeNode::Binary {
            value: Binary::Or,
            left_child: Box::from(LexingTreeNode::Leaf { value: Symbol::Character('a') }),
            right_child: Box::from(LexingTreeNode::Leaf { value: Symbol::Character('b') }),
        };

        assert_eq!(expected, LexingTreeNode::from("a|b"))
    }

    #[test]
    fn unary_tree() {
        let expected = LexingTreeNode::Unary {
            value: Unary::Kleene,
            child: Box::from(LexingTreeNode::Leaf { value: Symbol::Character('a') }),
        };

        assert_eq!(expected, LexingTreeNode::from("a*"))
    }

    #[test]
    fn complex_tree() {
        let expected = LexingTreeNode::Binary {
            value: Binary::Concat,
            right_child: Box::from(LexingTreeNode::Leaf { value: Symbol::Character('c') }),
            left_child: Box::from(LexingTreeNode::Binary {
                value: Binary::Or,
                left_child: Box::from(LexingTreeNode::Unary {
                    value: Unary::Kleene,
                    child: Box::from(LexingTreeNode::Leaf { value: Symbol::Character('a') })
                }),
                right_child: Box::from(LexingTreeNode::Unary {
                    value: Unary::Kleene,
                    child: Box::from(LexingTreeNode::Leaf { value: Symbol::Character('b') })
                })
            })
        };

        assert_eq!(expected, LexingTreeNode::from("(a*|b*)c"))
    }
}
