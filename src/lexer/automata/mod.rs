pub mod nfa;
pub mod dfa;

use super::visitor::{Visitable, Visitor};

type State = usize;

pub trait Automata {
    fn test(&self, input: &str) -> bool;
}
