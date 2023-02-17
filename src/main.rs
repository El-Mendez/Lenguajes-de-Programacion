use crate::lexer::automata::nfa::NFAutomata;
use crate::lexer::automata::dfa::{DFAutomata, DFAVisualizer};

mod lexer;

fn main() {
    // let tree = lexer::tree::ReNode::from("(a|b)*a(a|b)\\ε(a|ε)");
    // let graph = lexer::tree::ReNodeVisualizer::new();

    let automata = NFAutomata::from("(b|b)*abb(a|b)*").into_determinate();
    let mut graph = DFAVisualizer::new();

    println!("{}", graph.graph(&automata, "./test.html"));
}
