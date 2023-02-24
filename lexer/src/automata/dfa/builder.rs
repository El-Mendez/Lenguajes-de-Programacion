use std::collections::{HashMap, HashSet};
use crate::automata::dfa::DFAutomata;
use crate::automata::State;
use crate::operator::{BinaryOperator, UnaryOperator};
use crate::symbols::Symbol;
use crate::tree::LexTree;

struct NodeValues {
    first_positions: HashSet<usize>,
    last_positions: HashSet<usize>,
    nullable: bool,
}

pub struct DFABuilder {
    follow_positions: Vec<HashSet<usize>>,
    leaf_values: HashMap<char, HashSet<usize>>,
}
impl DFABuilder {
    pub fn build(node: &LexTree) -> DFAutomata {
        let mut builder = DFABuilder {
            follow_positions: Vec::new(),
            leaf_values: HashMap::new(),
        };

        // build the follow position table
        let mut last_node = builder.initialize_values(node);

        // add acceptance state
        let acceptance_state = builder.follow_positions.len();
        builder.follow_positions.push(HashSet::new());
        last_node.last_positions
            .into_iter()
            .for_each(|x| {
                builder
                    .follow_positions[x]
                    .insert(acceptance_state);
            });

        if last_node.nullable {
            last_node.first_positions.insert(acceptance_state);
        }

        // build the automata;
        let mut acceptance_states = HashSet::new();
        let mut transitions: HashMap<(State, char), usize> = HashMap::new();
        let mut current_state_id = 0;

        let mut known_states: Vec<HashSet<usize>> = vec![last_node.first_positions];

        loop {
            let current_state: HashSet<usize> = known_states[current_state_id].clone();

            if current_state.contains(&acceptance_state) {
                acceptance_states.insert(current_state_id);
            }

            for (c, current_char_positions) in &builder.leaf_values {
                let positions_with_cars: Vec<_> = current_state
                    .intersection(current_char_positions)
                    .collect();


                if positions_with_cars.is_empty() {
                    continue;
                }

                let new_state = positions_with_cars
                    .into_iter()
                    .fold(HashSet::new(),
                          |mut states, state| { states.extend(&builder.follow_positions[*state]); states });

                let to = known_states.iter()
                    .position(|other| other == &new_state)
                    .unwrap_or_else(|| {
                        known_states.push(new_state);
                        known_states.len() - 1
                    });

                transitions.insert((current_state_id, *c), to);
            }

            current_state_id += 1;
            if current_state_id == known_states.len() {
                break
            }
        }

        DFAutomata::new(transitions, acceptance_states, current_state_id-1)
    }

    fn connect_positions(&mut self, from_positions: &HashSet<usize>, follow_positions: &HashSet<usize>) {
        from_positions
            .iter()
            .for_each(|x| {
                self.follow_positions[*x]
                    .extend(follow_positions);
            });
    }

    fn initialize_values(&mut self, node: &LexTree) -> NodeValues {
        match node {
            LexTree::Leaf { value } => {
                match value {
                    Symbol::Epsilon => NodeValues {
                        first_positions: HashSet::new(),
                        last_positions: HashSet::new(),
                        nullable: true,
                    },

                    Symbol::Character(x) => {
                        let position = self.follow_positions.len();

                        self.leaf_values.entry(*x)
                            .or_insert_with(HashSet::new)
                            .insert(position);

                        self.follow_positions.push(HashSet::new());

                        let mut first_positions = HashSet::new();
                        first_positions.insert(position);

                        let mut last_positions = HashSet::new();
                        last_positions.insert(position);

                        NodeValues { nullable: false, first_positions, last_positions }
                    }
                }
            },
            LexTree::Unary { value, child } => {
                let mut node_values = self.initialize_values(child);

                match value {
                    UnaryOperator::Kleene => {
                        self.connect_positions(
                            &node_values.last_positions,
                            &node_values.first_positions
                        );

                        node_values.nullable = true;
                    }
                    UnaryOperator::Maybe => {
                        node_values.nullable = true;
                    }
                    UnaryOperator::Many => {
                        self.connect_positions(
                            &node_values.last_positions,
                            &node_values.first_positions
                        );
                    }
                }
                node_values
            }
            LexTree::Binary { value, left_child, right_child } => {
                let mut left_child = self.initialize_values(left_child);
                let right_child = self.initialize_values(right_child);

                match value {
                    BinaryOperator::Concat => {
                        self.connect_positions(
                            &left_child.last_positions,
                            &right_child.first_positions,
                        );

                        if left_child.nullable {
                            left_child.first_positions.extend(&right_child.first_positions)
                        }

                        if right_child.nullable {
                            left_child.last_positions.extend(&right_child.last_positions)
                        } else {
                            left_child.last_positions = right_child.last_positions;
                            left_child.nullable = false;
                        }

                        left_child
                    }
                    BinaryOperator::Or => {
                        left_child.nullable |= right_child.nullable;
                        left_child.last_positions.extend(&right_child.last_positions);
                        left_child.first_positions.extend(&right_child.first_positions);

                        left_child
                    }
                }
            }
        }
    }
}
