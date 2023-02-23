use clap::{Parser, ValueEnum};
use lexer::automata::dfa::{DFAutomata, DFAVisualizer};
use lexer::automata::nfa::{NFAutomata, NFAVisualizer};
use lexer::tree::{LexTree, LexTreeVisualizer};
use lexer::automata::Automata;
use lexer::LexError;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    /// a regular expression defining a language
    #[arg(value_parser = valid_expression)]
    expression: String,
    /// optional string to test against the language
    string: Option<String>,
    /// the automata of tree to create from the input expression
    #[arg(short, long, value_enum, default_value_t = Mode::Nfa)]
    mode: Mode,
}

fn valid_expression(s: &str) -> Result<String, String> {
    match LexTree::try_from(s) {
        Ok(_) => Ok(s.to_string()),
        Err(err) => {
            let (tabs, err) = match err {
                LexError::MissingOpeningParenthesis(x, _) | LexError::MissingClosingParenthesis(x, _) =>
                    (x, "expected matching parenthesis"),
                LexError::MissingArgument(x, _) => (x, "expected argument")
            };

            let spaces = " ".repeat(tabs);
            Err(format!("\n\t{s}\n\t{spaces}↑\n\t{spaces}{err}"))
        }
    }
}

#[derive(Copy, Clone, ValueEnum)]
enum Mode {
    /// LexTree
    Tree,
    /// NDA built using Thompson
    Nfa,
    /// DFA built directly from re
    Dfa,
    /// DFA built from a Thompson NDA
    ThompsonDFA,
}

fn main() {
    let cli = Cli::parse();
    let tree = LexTree::try_from(cli.expression.as_str()).unwrap(); // because of the validation this won't fail

    if let Some(s) = &cli.string {
        let automata: Box<dyn Automata> = match cli.mode {
            Mode::Nfa => Box::new(NFAutomata::from(&tree)),
            Mode::Dfa => Box::new(DFAutomata::from(&tree)),
            Mode::ThompsonDFA => Box::new(DFAutomata::from(NFAutomata::from(&tree))),
            Mode::Tree => {
                eprintln!("cannot test a language against a tree.");
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
            Mode::Dfa => DFAVisualizer::new(&DFAutomata::from(&tree)).show("test.html"),
            Mode::Nfa => NFAVisualizer::new(&NFAutomata::from(&tree)).show("test.html"),
            Mode::ThompsonDFA =>
                DFAVisualizer::new(&DFAutomata::from(NFAutomata::from(&tree))).show("test.html"),
        };
    }
}
