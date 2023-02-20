use lexer::automata::nfa::{NFAutomata, NFAVisualizer};
use lexer::automata::dfa::{DFAutomata, DFAVisualizer};


fn main() {
    let expression = "a*b|ε";

    match NFAutomata::try_from(expression) {
        Ok(automata) => { NFAVisualizer::new(&automata)
            .show("./test.html"); },

        Err(e) => println!("There was an error in your expression: {e:?}")
    };
}
