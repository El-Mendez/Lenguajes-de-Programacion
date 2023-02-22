use std::collections::{HashMap, HashSet};
use super::automata::{NFAutomata};
use super::super::State;
use crate::{Symbol, MermaidGraph};

pub struct NFAVisualizer {
    mermaid: String,
}

impl NFAVisualizer {
   pub fn new(automata: &NFAutomata) -> NFAVisualizer {
       let mut visualizer = NFAVisualizer { mermaid: String::new() };
       visualizer.add_descriptions(automata.acceptance_state);
       visualizer.add_transitions(&automata.transitions);

       visualizer
   }

    fn add_descriptions(&mut self, last_id: State) {
        (0..last_id).for_each(|id| self.mermaid += &format!("\n        {id}(({id}))"));
        self.mermaid += &format!("\n        {last_id}((({last_id})))");
    }

    fn add_transition(&mut self, from: State, to: State, symbol: Symbol) {
        match symbol {
            Symbol::Character(x) => self.mermaid += &format!("\n        {from} -->|\"{x}\"| {to}"),
            Symbol::Epsilon => {
                if from != to {
                    self.mermaid += &format!("\n        {from} -->|ε| {to}")
                }
            },
        }
    }

    fn add_transitions(&mut self, transitions: &HashMap<(State, Symbol), HashSet<State>>) {
        for (key, to) in transitions.iter() {
            let (from, symbol) = key;

            to.iter()
                .for_each(|state| self.add_transition(*from, *state, *symbol));
        }
    }

    pub fn show(&self, path: &str) -> String {
        self.generate_and_open_graph(path)
    }
}

impl MermaidGraph for NFAVisualizer {
    fn header(&self) -> &'static str {
        "graph LR"
    }

    fn get_mermaid_content(&self) -> &str {
        &self.mermaid
    }
}
