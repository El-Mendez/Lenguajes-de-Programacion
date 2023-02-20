pub mod nfa;
pub mod dfa;

type State = usize;

pub trait Automata {
    fn test(&self, input: &str) -> bool;
}
