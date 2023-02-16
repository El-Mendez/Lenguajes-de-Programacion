#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOperator {
    Concat, Or
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOperator {
    Kleene
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    Binary(BinaryOperator),
    Unary(UnaryOperator),
    OpenParenthesis,
    CloseParenthesis,
}
impl Operator {
    pub(crate) fn order(self) -> usize {
        match self {
            Operator::Unary(operation) => {
                match operation {
                    UnaryOperator::Kleene => 3,
                }
            }
            Operator::Binary(operation) => {
                match operation {
                    BinaryOperator::Concat => 2,
                    BinaryOperator::Or => 1,
                }
            }
            Operator::OpenParenthesis | Operator::CloseParenthesis => 0,
        }
    }
}

