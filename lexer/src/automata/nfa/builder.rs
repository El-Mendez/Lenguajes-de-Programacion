use std::collections::{HashMap, HashSet};
use crate::{UnaryOperator, BinaryOperator, Symbol};
use super::super::super::tree::LexTree;
use super::super::State;


pub struct NFABuilder {
    pub last_state: State,
    pub transitions: HashMap<(State, Symbol), HashSet<State>>,
}

impl NFABuilder {
    pub fn build(node: &LexTree) -> NFABuilder {
        let mut builder = NFABuilder { transitions: HashMap::new(), last_state: 0 };

        // create the root state
        let root_state = builder.create_root();
        builder.build_automata(node, root_state);
        builder
    }

    fn connect(&mut self, from: State, to: State, symbol: Symbol) {
        if let Some(destination_states) = self.transitions.get_mut(&(from, symbol)) {
            destination_states.insert(to);
        } else {
            self.transitions.insert((from, symbol), HashSet::from([to]));
        }
    }

    fn create_state(&mut self) -> State {
        self.last_state += 1;
        self.connect(self.last_state, self.last_state, Symbol::Epsilon);
        self.last_state
    }

    fn create_root(&mut self) -> State {
        self.connect(self.last_state, self.last_state, Symbol::Epsilon);
        self.last_state
    }

    fn build_automata(&mut self, node: &LexTree, starting_state: State) -> State {
        match node {
            LexTree::Leaf { value } => {
                let next_state = self.create_state();
                self.connect(starting_state, next_state, *value);

                next_state
            }

            LexTree::Binary { value: operator, right_child: right_node, left_child: left_node } => {
                match operator {
                    BinaryOperator::Concat => {
                        let connection_state = self.build_automata(left_node, starting_state);
                        self.build_automata(right_node, connection_state)
                    },
                    BinaryOperator::Or => {
                        let top_start = self.create_state();
                        let top_end = self.build_automata(left_node, top_start);

                        let bottom_start = self.create_state();
                        let bottom_end = self.build_automata(right_node, bottom_start);

                        // connect to the start of both automatas
                        self.connect(starting_state, top_start, Symbol::Epsilon);
                        self.connect(starting_state, bottom_start, Symbol::Epsilon);

                        // connect the end of both automatas
                        let end_state = self.create_state();
                        self.connect(top_end, end_state, Symbol::Epsilon);
                        self.connect(bottom_end, end_state, Symbol::Epsilon);

                        end_state
                    }
                }
            },

            LexTree::Unary { value: operator, child } => {
                match operator {
                    UnaryOperator::Kleene => {
                        let next_start = self.create_state();
                        let next_end = self.build_automata(child, next_start);
                        let end = self.create_state();

                        self.connect(starting_state, end, Symbol::Epsilon);
                        self.connect(starting_state, next_start, Symbol::Epsilon);
                        self.connect(next_end, next_start, Symbol::Epsilon);
                        self.connect(next_end, end, Symbol::Epsilon);

                        end
                    },

                    UnaryOperator::Maybe => {
                        let end = self.build_automata(child, starting_state);
                        self.connect(starting_state, end, Symbol::Epsilon);

                        end
                    }

                    UnaryOperator::Many => {
                        let end = self.build_automata(child, starting_state);
                        self.connect(end, starting_state, Symbol::Epsilon);

                        end
                    }
                }
            }
        }
    }
}
