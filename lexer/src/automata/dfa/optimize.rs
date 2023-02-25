use std::collections::{HashMap, HashSet};
use crate::automata::dfa::DFAutomata;
use crate::automata::State;

pub struct DFAOptimizer{
    old_transitions: HashMap<(State, char), State>,
    partitions: Vec<HashSet<State>>,
    alphabet: Vec<char>,
    old_acceptance_states: HashSet<State>,
}

impl DFAOptimizer {
    pub fn optimize(transitions: HashMap<(State, char), State>, old_acceptance_states: HashSet<State>, last_state: State, alphabet: Vec<char>) -> DFAutomata {
        if old_acceptance_states.is_empty() {
            // if the expression accepts nothing then let's just return an automata that does that
            return DFAutomata { transitions: HashMap::new(), acceptance_states: HashSet::new(), last_state: 0 }
        }

        let mut optimizer = DFAOptimizer::new(transitions, old_acceptance_states, alphabet, last_state);

        let (new_transitions, new_acceptance_states) = optimizer.new_transitions();

        DFAutomata { transitions: new_transitions,
            acceptance_states: new_acceptance_states,
            last_state: optimizer.partitions.len()-1
        }
    }

    fn new(old_transitions: HashMap<(State, char), State>, acceptance_states: HashSet<State>, alphabet: Vec<char>, last_state: State) -> DFAOptimizer {
        let mut optimizer = if acceptance_states.len()-1 == last_state {
            DFAOptimizer {
                old_transitions,
                partitions: vec![acceptance_states.clone()],
                alphabet,
                old_acceptance_states: acceptance_states
            }
        } else {
            let other_states: HashSet<State> = (0..=last_state)
                .into_iter()
                .filter(|x| !acceptance_states.contains(x))
                .collect();

            DFAOptimizer {
                old_transitions,
                partitions: vec![other_states, acceptance_states.clone()],
                alphabet,
                old_acceptance_states: acceptance_states
            }
        };

        optimizer.fix_partitions();
        optimizer.remove_dead_partitions();
        optimizer
    }

    fn fix_partitions(&mut self) {
        let mut changes_were_made = true;
        while changes_were_made {
            changes_were_made = false;

            for &c in &self.alphabet {
                let mut new_partitions = Vec::new();

                for partition in &self.partitions {
                    let mut current_partitions_splits = HashMap::new();

                    // can't split a partition of one. There shouldn't be partitions of zero too
                    if partition.len() == 1 {
                        new_partitions.push(partition.clone());
                        continue
                    }

                    for &state in partition {
                        let transition_destination = self
                            .partition_containing_transition(state, c);

                        current_partitions_splits
                            .entry(transition_destination)
                            .or_insert_with(HashSet::new)
                            .insert(state);
                    }

                    // add all the discovered new states
                    new_partitions.extend(current_partitions_splits.into_values());
                }

                if new_partitions.len() != self.partitions.len() {
                    changes_were_made = true;
                    self.partitions = new_partitions;
                }
            }
        }
    }

    fn remove_dead_partitions(&mut self) {
        // because it's already reduced we can only have one dead partition
        let dead_partition = self.partitions.iter()
            // we only really need a representative state
            .map(|partition| partition.iter().next().unwrap())
            .enumerate()
            .position(|(i, &state)| self.is_dead_partition(i, state));

        if let Some(dead_position) = dead_partition {
            self.partitions.remove(dead_position);
        }
    }

    fn is_dead_partition(&self, index: usize, state: State) -> bool {
        // we can't really remove a dead accepting state
        !self.old_acceptance_states.contains(&state) && self.alphabet.iter()
            .all(|&c|
                if let Some(x) = self.partition_containing_transition(state, c) {
                    x == index // the transitions goes to itself
                } else {
                    true // the transition goes to nowhere
                }
            )
    }

    fn new_transitions(&mut self) -> (HashMap<(State, char), State>, HashSet<usize>) {
        // make the partition containing the initial state the initial partition
        let initial_partition = self.partitions
            .iter()
            .position(|p| p.contains(&0))
            .expect("there must be an initial state!");

        self.partitions.swap(0, initial_partition);

        let mut transitions = HashMap::new();
        let mut acceptance_states = HashSet::new();

        // build the new automata from the partitions
        for (from, partition) in self.partitions.iter().enumerate() {
            // get a representative state
            if !partition.is_disjoint(&self.old_acceptance_states) {
                acceptance_states.insert(from);
            }
            let representative_state = partition.iter().next().expect("no empty partitions!");

            for &c in &self.alphabet {
                if let Some(to) = self.partition_containing_transition(*representative_state, c) {
                    transitions.insert((from, c), to);
                }
            }
        }

        (transitions, acceptance_states)
    }

    fn partition_containing_transition(&self, state: State, c: char) -> Option<usize> {
        let new_state = self.old_transitions.get(&(state, c))?;
        let new_state = self.partition_containing(*new_state)?;
        Some(new_state)
    }

    fn partition_containing(&self, state: State) -> Option<usize> {
        self.partitions.iter()
            .position(|partition| partition.contains(&state))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::DFAVisualizer;

    #[test]
    fn test() {
        DFAVisualizer::new(&DFAOptimizer::optimize(
            HashMap::from([
                ((0, '0'), 1),
                ((0, '1'), 5),
                ((1, '0'), 6),
                ((1, '1'), 2),
                ((2, '0'), 0),
                ((2, '1'), 2),
                ((3, '0'), 2),
                ((3, '1'), 6),
                ((4, '0'), 7),
                ((4, '1'), 5),
                ((5, '0'), 2),
                ((5, '1'), 6),
                ((6, '0'), 6),
                ((6, '1'), 4),
                ((7, '0'), 6),
                ((7, '1'), 2),
            ]),
            HashSet::from([2]),
            7,
            vec!['0', '1'],
        )).show("test.html");
    }

    #[test]
    fn try_automata() {
        DFAutomata::try_from("a(a|b)*a(a|b)").unwrap();
    }
}