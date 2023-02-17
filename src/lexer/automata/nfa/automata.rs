use std::collections::{HashMap, HashSet};
use super::builder::NFABuilder;
use super::super::super::tree::{Symbol, ReNode};
use super::super::{State, Automata};

pub struct NFAutomata {
    pub transitions: HashMap<(State, Symbol), HashSet<State>>,
    pub acceptance_state: State,
}

impl NFAutomata {
    fn epsilon_closure(&self, state: State) -> HashSet<State> {
        let mut visited_states = HashSet::from([state]);
        let mut new_states = self.single_movement(state, Symbol::Epsilon)
            .expect("epsilon transitions should always return at least return the caller!") - &visited_states;

        while !new_states.is_empty() {
            visited_states.extend(&new_states);
            new_states = &self.movement(&new_states, Symbol::Epsilon) - &visited_states;
        }

        visited_states
    }

    fn movement(&self, states: &HashSet<State>, symbol: Symbol) -> HashSet<State> {
        let mut result_states = HashSet::new();

        states.iter()
            .filter_map(|&state| self.single_movement(state, symbol))
            .for_each(|new_states| result_states.extend(new_states));

        result_states
    }

    fn single_movement(&self, state: State, symbol: Symbol) -> Option<&HashSet<State>>  {
        self.transitions.get(&(state, symbol))
    }
}

impl Automata for NFAutomata {
    fn test(&self, input: &str) -> bool {

        let final_states = input
            .chars()
            .fold(HashSet::from([0]), |states, c| self.movement(&states, Symbol::Character(c)));

        final_states.contains(&self.acceptance_state)
    }
}

impl From<ReNode> for NFAutomata {
    fn from(value: ReNode) -> Self {
        let builder = NFABuilder::build(&value);

        NFAutomata {
            transitions: builder.transitions,
            acceptance_state: builder.last_state,
        }
    }
}

impl From<&str> for NFAutomata {
    fn from(value: &str) -> Self {
        let node = ReNode::from(value);
        let builder = NFABuilder::build(&node);

        NFAutomata {
            transitions: builder.transitions,
            acceptance_state: builder.last_state,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_expression() {
        let automata = NFAutomata::from("a");
        assert!(automata.test("a"))
    }
}
