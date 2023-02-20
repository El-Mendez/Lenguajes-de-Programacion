use dialoguer::{theme::ColorfulTheme, Select, Input};
use lexer::LexError;
use lexer::tree::{LexTree, LexTreeVisualizer};
use lexer::automata::dfa::{DFAutomata, DFAVisualizer};
use lexer::automata::nfa::{NFAutomata, NFAVisualizer};

fn main() {
    let theme = ColorfulTheme::default();

    let selections = vec![
        "Árbol de expresiones",
        "Autómata finito no determinista",
        "Autómata finito determinista",
        "Salir",
    ];

    loop {
        let selection = Select::with_theme(&theme)
            .with_prompt("¿Qué quieres visualizar?")
            .default(0)
            .items(&selections)
            .interact()
            .unwrap();

        if selection == 3 {
            return;
        }

        let expression = Input::with_theme(&theme)
            .with_prompt("Tu expresión")
            .validate_with(move |input: &String| -> Result<(), &str> {
                match LexTree::try_from(input.as_str()) {
                    Ok(_) => Ok(()),
                    Err(err) => {
                        match err {
                            LexError::MissingOpeningParenthesis => Err("Olvidaste poner un símbolo `(`!"),
                            LexError::MissingClosingParenthesis => Err("Olvidaste cerrar un paréntesis!"),
                            LexError::MissingArgument => Err("No colocaste los argumentos de un operando"),
                            LexError::Unknown => Err("uhhhmm, algo raro pasó"),
                        }
                    }
                }
            }).interact_text()
            .unwrap();

        match selection {
            0 => {
                let tree = LexTree::try_from(expression.as_str()).unwrap();
                LexTreeVisualizer::new(&tree)
                    .show("test.html");
            },
            1 => {
                let non_deterministic = NFAutomata::try_from(expression.as_str()).unwrap();
                NFAVisualizer::new(&non_deterministic)
                    .show("test.html");
            },
            2 => {
                let non_deterministic = NFAutomata::try_from(expression.as_str()).unwrap();
                let deterministic = DFAutomata::from(non_deterministic);
                DFAVisualizer::new(&deterministic)
                    .show("test.html");
            },
            _ => panic!("this option does not exist"),
        }

        println!("\nFelicidades! Puedes ver tu árbol en el archivo test.html\n\n")
    }
}
