use std::io::Write;
use super::{Visitable, Visitor};
use super::operator::{UnaryOperator, BinaryOperator};
use super::symbols::{Symbol};
use super::lexing_tree::ReNode;

pub struct ReNodeVisualizer {
    last_id: usize,
    mermaid: String,
}

impl ReNodeVisualizer {
    pub fn new() -> Self {
        ReNodeVisualizer { last_id: 0, mermaid: String::new() }
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

    fn visit_unary(&mut self, value: UnaryOperator, child: &ReNode) {
        let id = self.last_id;

        let description = match value {
            UnaryOperator::Kleene => "*",
        };
        self.add_description(id, description, false);

        // graph the children.
        self.last_id += 1;
        self.add_connection(id, self.last_id);
        child.accept(self);
    }

    fn visit_binary(&mut self, value: BinaryOperator, left_child: &ReNode, right_child: &ReNode) {
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

    pub fn graph(mut self, node: &ReNode, file_name: &str) -> String {
        self.visit(node);
        let content = Self::generate_html(&self.mermaid);
        Self::write_to_file(&content, file_name);

        content
    }

    fn generate_html(diagram: &str) -> String {
        format!(r#"
<!DOCTYPE html>
<html lang="en"><head><meta charset="utf-8" /></head>
  <body>
    <pre class="mermaid">
      flowchart TD{diagram}
    </pre>
    <script type="module">
      import mermaid from 'https://cdn.jsdelivr.net/npm/mermaid@9/dist/mermaid.esm.min.mjs';
      mermaid.initialize({{ startOnLoad: true }});
    </script>
  </body>
</html>
        "#)
    }

    fn write_to_file(contents: &str, path: &str) {
        let mut f = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)
            .unwrap();
        f.write_all(contents.as_bytes()).unwrap();
        f.flush().unwrap();
    }
}

impl Visitor<ReNode> for ReNodeVisualizer {
    fn visit(&mut self, node: &ReNode) {
        match node {
            ReNode::Unary { value, child } =>
                self.visit_unary(*value, child),

            ReNode::Binary { value, left_child, right_child } =>
                self.visit_binary(*value, left_child, right_child),

            ReNode::Leaf { value } =>
                self.visit_leaf(*value)
        }
    }
}
