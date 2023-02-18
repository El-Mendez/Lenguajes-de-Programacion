use super::{Visitable, Visitor};
use crate::{UnaryOperator, BinaryOperator, Symbol, MermaidGraph};
use super::LexTree;

pub struct LexTreeVisualizer {
    last_id: usize,
    mermaid: String,
}

impl LexTreeVisualizer {
    pub fn new(node: &LexTree) -> Self {
        let mut visualizer = LexTreeVisualizer { last_id: 0, mermaid: String::new() };
        visualizer.visit(node);
        visualizer
    }

    fn add_description(&mut self, id: usize, description: &str, is_terminal: bool) {
        self.mermaid += &format!("\n        {id}((\"{description}\")) ");
        if is_terminal {
            self.mermaid += &format!("\n        style {id} fill:#f9f ");
        }
    }

    fn add_connection(&mut self, from: usize, to: usize) {
        self.mermaid += &format!("\n        {from} --> {to} ");
    }

    fn visit_unary(&mut self, value: UnaryOperator, child: &LexTree) {
        let id = self.last_id;

        let description = match value {
            UnaryOperator::Kleene => "*",
            UnaryOperator::Maybe => "?",
            UnaryOperator::Many => "+",
        };
        self.add_description(id, description, false);

        // graph the children.
        self.last_id += 1;
        self.add_connection(id, self.last_id);
        child.accept(self);
    }

    fn visit_binary(&mut self, value: BinaryOperator, left_child: &LexTree, right_child: &LexTree) {
        let id = self.last_id;

        let description = match value {
            BinaryOperator::Concat => ".",
            BinaryOperator::Or => "|",
        };
        self.add_description(id, description, false);

        // graph the children.
        self.last_id += 1;
        self.add_connection(id, self.last_id);
        left_child.accept(self);

        self.last_id += 1;
        self.add_connection(id, self.last_id);
        right_child.accept(self);
    }

    fn visit_leaf(&mut self, value: Symbol) {
        let description = match value {
            Symbol::Character(x) => x.to_string(),
            Symbol::Epsilon => "ε".to_string(),
        };

        self.add_description(self.last_id, &description, matches!(value, Symbol::Character(_)));
    }

    pub fn show(&self, path: &str) -> String {
        self.generate_and_open_graph(path)
    }
}

impl Visitor<LexTree> for LexTreeVisualizer {
    fn visit(&mut self, node: &LexTree) {
        match node {
            LexTree::Unary { value, child } =>
                self.visit_unary(*value, child),

            LexTree::Binary { value, left_child, right_child } =>
                self.visit_binary(*value, left_child, right_child),

            LexTree::Leaf { value } =>
                self.visit_leaf(*value)
        }
    }
}

impl MermaidGraph for LexTreeVisualizer {
    fn header(&self) -> &'static str {
        "flowchart TD"
    }

    fn get_mermaid_content(&self) -> &str {
        &self.mermaid
    }
}

