use lexer::automata::nfa::NFAutomata;
use lexer::automata::dfa::{DFAutomata, DFAVisualizer};


fn main() {
    // let tree = lexer::tree::ReNode::from("(a|b)*a(a|b)\\ε(a|ε)");
    // let graph = lexer::tree::ReNodeVisualizer::new();

    let automata = NFAutomata::from("a+bc?").into_determinate();
    let mut graph = DFAVisualizer::new();

    println!("{}", graph.graph(&automata, "./test.html"));
}
