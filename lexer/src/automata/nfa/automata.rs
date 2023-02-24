use std::collections::{HashMap, HashSet};
use super::builder::NFABuilder;
use crate::{LexError, Symbol};
use crate::tree::LexTree;
use super::super::{State, Automata};
use super::super::dfa::DFAutomata;

pub struct NFAutomata {
    pub(super) transitions: HashMap<(State, Symbol), HashSet<State>>,
    pub(super) acceptance_state: State,
}

impl NFAutomata {
    fn epsilon_closure(&self, state: HashSet<State>) -> HashSet<State> {
        let mut visited_states = state;
        let mut new_states = self.movement(&visited_states, Symbol::Epsilon);

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

    pub(crate) fn into_determinate(self) -> DFAutomata {
        let chars_with_transitions: HashSet<char> = self.transitions
            .keys()
            .filter_map(|(_, x)| if let Symbol::Character(x) = x { Some(*x) } else { None })
            .collect();


        let mut acceptance_states = HashSet::new();
        let mut transitions = HashMap::new();
        let mut current_state_id = 0;

        let mut known_states = vec![
            self.epsilon_closure(HashSet::from([0]))
        ];

        loop {
            let current_state = &known_states[current_state_id].clone();

            if current_state.contains(&self.acceptance_state) {
                acceptance_states.insert(current_state_id);
            }

            for x in &chars_with_transitions {
                let new_state = self.epsilon_closure(self.movement(current_state, Symbol::Character(*x)));
                if new_state.is_empty() {
                    continue;
                }

                let to = known_states.iter()
                    .position(|other| other == &new_state)
                    .unwrap_or_else(|| {
                        known_states.push(new_state);
                        known_states.len() - 1
                    });

                transitions.insert((current_state_id, *x), to);
            }

            current_state_id += 1;
            if current_state_id == known_states.len() {
                break;
            }
        }

        DFAutomata::new(transitions, acceptance_states, current_state_id-1)
    }
}

impl Automata for NFAutomata {
    fn test(&self, input: &str) -> bool {

        let final_states = input
            .chars()
            .fold(
                self.epsilon_closure(HashSet::from([0])),
                |states, c| {
                    self.epsilon_closure(self.movement(&states, Symbol::Character(c)))
            });

        final_states.contains(&self.acceptance_state)
    }
}

impl From<&LexTree> for NFAutomata {
    fn from(value: &LexTree) -> Self {
        let builder = NFABuilder::build(value);

        NFAutomata {
            transitions: builder.transitions,
            acceptance_state: builder.last_state,
        }
    }
}

impl TryFrom<&str> for NFAutomata {
    type Error = LexError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let node = LexTree::try_from(value)?;
        let builder = NFABuilder::build(&node);

        Ok(NFAutomata {
            transitions: builder.transitions,
            acceptance_state: builder.last_state,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_expression() {
        let automata = NFAutomata::try_from("a").unwrap();
        assert!(automata.test("a"))
    }
}
