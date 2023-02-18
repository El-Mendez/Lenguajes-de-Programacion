use std::collections::{HashMap, HashSet};
use crate::mermaid_graph::MermaidGraph;
use super::automata::{DFAutomata};
use super::super::State;

pub struct DFAVisualizer {
    mermaid: String,
}

impl DFAVisualizer {
   pub fn new(automata: &DFAutomata) -> DFAVisualizer {
       let mut visualizer = DFAVisualizer { mermaid: String::new() };
       visualizer.add_descriptions(automata.last_state, &automata.acceptance_states);
       visualizer.add_transitions(&automata.transitions);

       visualizer
   }

    fn add_descriptions(&mut self, last_id: State, accepted_states: &HashSet<State>) {
        (0..=last_id).for_each(|id| {
            if accepted_states.contains(&id) {
                self.mermaid += &format!("\n        {id}((({id})))");
            } else {
                self.mermaid += &format!("\n        {id}(({id}))")
            }
        });
    }

    fn add_transition(&mut self, from: State, to: State, chars: HashSet<char>) {
        let mut chars: Vec<char> = chars.into_iter()
            .collect();
        chars.sort();

        // concat all chars into a string with commas in between
        let chars: String = chars.into_iter()
            .fold(String::new(), |acc, x| format!("{acc},{x}"))
            .chars()
            .skip(1)
            .collect();

        self.mermaid += &format!("\n        {from} -->|\"{chars}\"| {to}");
    }

    fn add_transitions(&mut self, transitions: &HashMap<(State, char), State>) {
        let mut new_transitions: HashMap<(State, State), HashSet<char>> = HashMap::new();

        for ((from, c), to) in transitions.iter() {
            if let Some(chars) = new_transitions.get_mut(&(*from, *to)) {
                chars.insert(*c);
            } else {
                new_transitions.insert((*from, *to), HashSet::from([*c]));
            }
        }

        new_transitions.into_iter().for_each(|((from, to), chars)| {
            self.add_transition(from, to, chars)
        })
    }

    pub fn show(&self, path: &str) -> String {
        self.generate_and_open_graph(path)
    }
}

impl MermaidGraph for DFAVisualizer {
    fn header(&self) -> &'static str {
        "graph LR"
    }

    fn get_mermaid_content(&self) -> &str {
        &self.mermaid
    }
}
