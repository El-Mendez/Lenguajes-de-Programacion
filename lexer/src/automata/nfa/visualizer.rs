use std::collections::{HashMap, HashSet};
use std::io::Write;
use super::automata::{NFAutomata};
use super::super::State;
use super::super::super::tree::Symbol;

pub struct NFAVisualizer {
    mermaid: String,
}

impl NFAVisualizer {
   pub fn new() -> NFAVisualizer {
       NFAVisualizer { mermaid: String::new() }
   }

    fn generate_html(diagram: &str) -> String {
        format!(r#"
<!DOCTYPE html>
<html lang="en"><head><meta charset="utf-8" /></head>
  <body>
    <pre class="mermaid">
      graph LR{diagram}
    </pre>
    <script type="module">
      import mermaid from 'https://cdn.jsdelivr.net/npm/mermaid@9/dist/mermaid.esm.min.mjs';
      mermaid.initialize({{ startOnLoad: true }});
    </script>
  </body>
</html>
        "#)
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
                    self.mermaid += &format!("\n        {from} --> {to}")
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

    fn write_to_file(contents: &str, path: &str) {
        let mut f = std::fs::OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(path)
            .unwrap();
        f.write_all(contents.as_bytes()).unwrap();
        f.flush().unwrap();
    }

    pub fn graph(&mut self, automata: &NFAutomata, file: &str) -> String{
        self.add_descriptions(automata.acceptance_state);
        self.add_transitions(&automata.transitions);

        let html = NFAVisualizer::generate_html(&self.mermaid);
        NFAVisualizer::write_to_file(&html, file);
        html
    }
}