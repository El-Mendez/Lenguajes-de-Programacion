mod nfa;
mod nfa_builder;
mod nfa_visualizer;

use super::visitor::{Visitable, Visitor};
pub use nfa::NFAutomata;
pub use nfa_visualizer::NFAVisualizer;


type State = usize;

pub trait Automata {
    fn test(&self, input: &str) -> bool;
}
