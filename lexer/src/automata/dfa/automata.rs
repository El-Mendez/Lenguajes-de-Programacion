use std::collections::{HashMap, HashSet};
use crate::automata::dfa::builder::DFABuilder;
use crate::LexError;
use crate::tree::LexTree;
use super::super::{Automata, State};
use super::super::nfa::NFAutomata;

pub struct DFAutomata {
    pub(super) transitions: HashMap<(State, char), State>,
    pub(super) acceptance_states: HashSet<State>,
    pub(super) last_state: State
}

impl DFAutomata {
    pub(crate) fn new(transitions: HashMap<(State, char), State>, acceptance_states: HashSet<State>, last_state: State) -> DFAutomata {
        DFAutomata { transitions, acceptance_states, last_state }
    }

    fn movement(&self, state: State, c: char) -> Option<State> {
        self.transitions.get(&(state, c)).copied()
    }
}

impl Automata for DFAutomata {
    fn test(&self, input: &str) -> bool {
        let final_state = input.chars()
            .try_fold(0, |state, x| self.movement(state, x));

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

impl TryFrom<&str> for DFAutomata {
    type Error = LexError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let node = LexTree::try_from(value)?;
        Ok(DFABuilder::build(&node))
    }
}

impl From<&LexTree> for DFAutomata {
    fn from(value: &LexTree) -> Self {
        DFABuilder::build(value)
    }
}
