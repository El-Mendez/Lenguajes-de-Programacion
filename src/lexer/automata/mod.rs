pub mod nfa;

use super::visitor::{Visitable, Visitor};

type State = usize;

pub trait Automata {
    fn test(&self, input: &str) -> bool;
}
