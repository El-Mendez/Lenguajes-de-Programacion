mod lexer;

fn main() {
    let tree = lexer::tree::ReNode::from("(a|b)*a(a|b)\\ε(a|ε)");
    let graph = lexer::tree::ReNodeVisualizer::new();

    println!("{}", graph.graph(&tree, "./test.html"));
}
