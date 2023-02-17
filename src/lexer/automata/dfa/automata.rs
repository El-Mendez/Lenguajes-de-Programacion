use std::collections::{HashMap, HashSet};
use super::super::Automata;
use super::super::nfa::NFAutomata;
use super::super::State;
use super::super::super::tree::Symbol;

pub struct DFAutomata {
    pub transitions: HashMap<(State, Symbol), State>,
    pub acceptance_states: HashSet<State>,
    pub last_state: State
}

impl DFAutomata {
    fn movement(&self, state: State, symbol: Symbol) -> Option<State> {
        match symbol {
            Symbol::Epsilon => Some(state),
            Symbol::Character(_) => self.transitions.get(&(state, symbol)).copied()
        }
    }
}

impl Automata for DFAutomata {
    fn test(&self, input: &str) -> bool {
        let final_state = input.chars()
            .try_fold(0, |state, x| self.movement(state, Symbol::Character(x)));

        if let Some(final_state) = final_state {
            return self.acceptance_states.contains(&final_state)
        }
        false
    }
}

impl From<NFAutomata> for DFAutomata {
    fn from(value: NFAutomata) -> Self {
        value.into_determinate()
    }
}
