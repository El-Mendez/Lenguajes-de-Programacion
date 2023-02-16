use crate::lexer::lexing_tree::{LexingTreeNode, Visitor};

struct TreeVisualizer {
    current: usize,
    show: String,
}

impl Visitor<usize> for TreeVisualizer {
    fn visit(&mut self, node: &LexingTreeNode) -> usize {

    }
}
