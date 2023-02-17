use std::collections::{HashMap, HashSet};
use std::io::Write;
use std::process::Command;
use super::automata::{DFAutomata};
use super::super::State;
use super::super::super::tree::Symbol;

pub struct DFAVisualizer {
    mermaid: String,
}

impl DFAVisualizer {
   pub fn new() -> DFAVisualizer {
       DFAVisualizer { mermaid: String::new() }
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

    fn add_descriptions(&mut self, last_id: State, accepted_states: &HashSet<State>) {
        (0..=last_id).for_each(|id| {
            if accepted_states.contains(&id) {
                self.mermaid += &format!("\n        {id}((({id})))");
            } else {
                self.mermaid += &format!("\n        {id}(({id}))")
            }
        });
    }

    fn add_transition(&mut self, from: State, to: State, symbols: HashSet<Symbol>) {
        let mut symbols: Vec<char> = symbols.into_iter()
            .filter_map(|s| {
                if let Symbol::Character(x) = s {
                    Some(x)
                } else {
                    None
                }
            })
            .collect();

        symbols.sort();

        let symbols: String = symbols.into_iter()
            .fold(String::new(), |acc, x| format!("{acc},{x}"))
            .chars()
            .skip(1)
            .collect();

        self.mermaid += &format!("\n        {from} -->|\"{symbols}\"| {to}");
    }

    fn add_transitions(&mut self, transitions: &HashMap<(State, Symbol), State>) {
        let mut new_transitions: HashMap<(State, State), HashSet<Symbol>> = HashMap::new();

        for ((from, symbol), to) in transitions.iter() {
            if let Some(symbols) = new_transitions.get_mut(&(*from, *to)) {
                symbols.insert(*symbol);
            } else {
                new_transitions.insert((*from, *to), HashSet::from([*symbol]));
            }
        }

        new_transitions.into_iter().for_each(|((from, to), symbols)| {
            self.add_transition(from, to, symbols)
        })
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

    pub fn graph(&mut self, automata: &DFAutomata, file: &str) -> String{
        self.add_descriptions(automata.last_state, &automata.acceptance_states);
        self.add_transitions(&automata.transitions);

        let html = DFAVisualizer::generate_html(&self.mermaid);
        DFAVisualizer::write_to_file(&html, file);
        Command::new(if cfg!(unix) { "open" } else { "start "})
            .arg(file)
            .spawn()
            .unwrap();
        html
    }
}