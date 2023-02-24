use std::collections::{HashMap, HashSet};
use crate::automata::dfa::DFAutomata;
use crate::automata::State;

pub struct DFAOptimizer{
    transitions: HashMap<(State, char), State>,
    partitions: Vec<HashSet<State>>,
}

impl DFAOptimizer {
    pub fn optimize(transitions: HashMap<(State, char), State>, old_acceptance_states: HashSet<State>, last_state: State, alphabet: Vec<char>) -> DFAutomata {
        if old_acceptance_states.is_empty() {
            return DFAutomata { transitions: HashMap::new(), acceptance_states: HashSet::new(), last_state: 0 }
        }

        let other_states: HashSet<State> = (0..=last_state)
            .into_iter()
            .filter(|x| !old_acceptance_states.contains(x))
            .collect();

        let mut optimizer = if other_states.is_empty() {
            // remember we still may have an implicit fail state! We still need to reduce stuff
            DFAOptimizer::new(transitions, vec![old_acceptance_states.clone()], &alphabet)
        } else {
            DFAOptimizer::new(transitions, vec![other_states, old_acceptance_states.clone()], &alphabet)
        };


        // make the partition containing the initial state the initial partition
        let initial_partition = optimizer.partitions
            .iter()
            .position(|p| p.contains(&0))
            .expect("there must be an initial state!");

        optimizer.partitions.swap(0, initial_partition);

        let mut transitions = HashMap::new();
        let mut acceptance_states = HashSet::new();

        // build the new automata from the partitions
        for (from, partition) in optimizer.partitions.iter().enumerate() {
            // get a representative state
            if !partition.is_disjoint(&old_acceptance_states) {
                acceptance_states.insert(from);
            }
            let state = partition.iter().next().expect("no empty partitions!");

            for &c in &alphabet {
                if let Some(to) = optimizer.partition_containing_transition(*state, c) {
                    transitions.insert((from, c), to);
                }
            }
        }

        DFAutomata { transitions, acceptance_states, last_state: optimizer.partitions.len()-1 }
    }

    fn new(transitions: HashMap<(State, char), State>, partitions: Vec<HashSet<State>>, alphabet: &Vec<char>) -> DFAOptimizer {
        let mut optimizer = DFAOptimizer { transitions, partitions };

        let mut changes_were_made = true;
        let mut changes_were_made_this_iteration = false;
        let mut partitions_going_to: HashMap<Option<usize>, usize> = HashMap::new();
        let mut new_partitions = optimizer.partitions.clone();

        while changes_were_made {
            changes_were_made = false;

            for &c in alphabet {

                for i in 0..optimizer.partitions.len() {
                    let current_partition = &optimizer.partitions[i];
                    if current_partition.len() == 1 {
                        // we can't really partition a partition of one and there shouldn't be empty partitions.
                        continue;
                    }
                    // make current partition into an iterator to avoid some hashsets iteration randomness
                    let mut current_partition = current_partition.iter();

                    let mut expected_partition_transition = optimizer.partition_containing_transition(*current_partition.next().unwrap(), c);

                    for &state in current_partition {
                        partitions_going_to.clear();
                        let partition_transition = optimizer.partition_containing_transition(state, c);
                        if partition_transition == expected_partition_transition {
                            continue;
                        }

                        changes_were_made = true;
                        changes_were_made_this_iteration = true;

                        let new_partition = partitions_going_to.entry(partition_transition)
                            .or_insert_with(|| {
                                new_partitions.push(HashSet::new());
                                new_partitions.len() - 1
                            });

                        new_partitions[*new_partition].insert(state);
                        new_partitions[i].remove(&state);
                    }
                }

                if changes_were_made_this_iteration {
                    changes_were_made_this_iteration = false;
                    optimizer.partitions = new_partitions.clone();
                }
            }
        }

        optimizer
    }

    fn partition_containing_transition(&self, state: State, c: char) -> Option<usize> {
        let new_state = self.transitions.get(&(state, c))?;
        Some(self.partition_containing(*new_state))
    }

    fn partition_containing(&self, state: State) -> usize {
        self.partitions.iter()
            .position(|partition| partition.contains(&state))
            .expect("all states should be in the partitions")
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