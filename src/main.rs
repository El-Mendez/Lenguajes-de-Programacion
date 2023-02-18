use lexer::automata::nfa::{NFAutomata, NFAVisualizer};
use lexer::automata::dfa::{DFAutomata, DFAVisualizer};


fn main() {
    let automata = NFAutomata::from("a+bc?");//.into_determinate();
    let graph = NFAVisualizer::new(&automata);


    // println!("{}", graph.show("./test.html"));
}
