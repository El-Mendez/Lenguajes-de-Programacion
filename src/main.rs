use crate::lexer::automata::{NFAutomata, NFAVisualizer};

mod lexer;

fn main() {
    // let tree = lexer::tree::ReNode::from("(a|b)*a(a|b)\\ε(a|ε)");
    // let graph = lexer::tree::ReNodeVisualizer::new();

    let automata = NFAutomata::from("(a|ε)b(aa*)(c|ε)");
    let mut graph = NFAVisualizer::new();

    println!("{}", graph.graph(&automata, "./test.html"));
}
