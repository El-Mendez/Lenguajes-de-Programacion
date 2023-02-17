use std::collections::{HashMap, HashSet};
use super::builder::NFABuilder;
use super::super::super::tree::{Symbol, ReNode};
use super::super::{State, Automata};
use super::super::dfa::DFAutomata;

pub struct NFAutomata {
    pub transitions: HashMap<(State, Symbol), HashSet<State>>,
    pub acceptance_state: State,
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

    pub fn into_determinate(self) -> DFAutomata {
        let symbols: HashSet<Symbol> = self.transitions
            .keys()
            .map(|(_, x)| *x)
            .filter(|x| matches!(x, Symbol::Character(_)))
            .collect();


        let mut acceptance_states = HashSet::new();
        let mut transitions = HashMap::new();

        let mut known_states = vec![
            self.epsilon_closure(HashSet::from([0]))
        ];

        let mut current_state_id = 0;
        loop {
            let current_state = &known_states[current_state_id].clone();

            if current_state.contains(&self.acceptance_state) {
                acceptance_states.insert(current_state_id);
            }

            for x in &symbols {
                let new_state = self.epsilon_closure(self.movement(current_state, *x));
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

        DFAutomata { transitions, acceptance_states, last_state: current_state_id-1 }
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
