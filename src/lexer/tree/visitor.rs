use super::lexing_tree::ReNode;

pub trait Visitor<T> {
    fn visit(&mut self, node: &ReNode) -> T;
}
