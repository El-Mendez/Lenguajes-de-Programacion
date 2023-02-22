use clap::{Parser, ValueEnum};
use lexer::automata::dfa::{DFAutomata, DFAVisualizer};
use lexer::automata::nfa::{NFAutomata, NFAVisualizer};
use lexer::tree::{LexTree, LexTreeVisualizer};
use lexer::automata::Automata;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    /// a regular expression defining a language
    #[arg(value_parser = valid_expression)]
    expression: String,
    /// optional string to test against the language
    string: Option<String>,
    /// the automata of tree to create from the input expression
    #[arg(short, long, value_enum, default_value_t = Mode::NFA)]
    mode: Mode,
}

fn valid_expression(s: &str) -> Result<String, String> {
    if let Ok(_) = LexTree::try_from(s) {
        Ok(s.into())
    } else {
        Err("\n\n\tinvalid re expression".into())
    }
}

#[derive(Copy, Clone, ValueEnum)]
enum Mode {
    /// LexTree
    Tree,
    /// NDA built using Thompson
    NFA,
    /// DFA built from a Thompson NDA
    ThompsonDFA,
}

fn main() {
    let cli = Cli::parse();
    let tree = LexTree::try_from(cli.expression.as_str()).unwrap(); // because of the validation this won't fail

    if let Some(s) = &cli.string {
        let automata: Box<dyn Automata> = match cli.mode {
            Mode::NFA => Box::new(NFAutomata::from(tree)),
            Mode::ThompsonDFA => Box::new(DFAutomata::from(NFAutomata::from(tree))),
            Mode::Tree => {
                println!("cannot test a language against a tree.");
                return; // early return
            },
        };

        if automata.test(s) {
            println!("the inputted string matches the language");
        } else {
            println!("the inputted string does not match the language");
        }

    } else {
        match cli.mode {
            Mode::Tree => LexTreeVisualizer::new(&tree).show("test.html"),
            Mode::NFA => NFAVisualizer::new(&NFAutomata::from(tree)).show("test.html"),
            Mode::ThompsonDFA =>
                DFAVisualizer::new(&DFAutomata::from(NFAutomata::from(tree))).show("test.html"),
        };
    }
}
