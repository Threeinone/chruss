use chess::engine::*;
use chess::pgn::*
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

const HELPSTRING: &str = "Invalid move. use \"help\" for a list of valid commands."

fn main() -> Result<()> {
    //(ext) `()` can be used when no completer is required
    let mut rl = DefaultEditor::new()?;
    let mut current_turn: u8 = 1;
    let mut to_move = ToMove::White;
    let gameboard = Board::new();
    loop {
        print!("{}\n", gameboard);
        let prompt = if to_move == ToMove::White {
            format!("White ({}) :", current_turn)
        } else {
            format!("Black ({}) :", current_turn)
        };
        let readline = rl.readline(&prompt);
        match readline {
            /* guard cases */
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                continue
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            },
            /***/
            Ok(line) => {
                rl.add_history_entry(line.as_str())?;
                handle_input(line);
            }
        }
        let (current_turn, to_move) = tick(current_turn, to_move);
    }
    Ok(())
}

// TODO
fn handle_input(input: &str) {
    let mut input = input.split(' ');
    let Some(cmd) = input.next() else {
        println!("{}", &HELPSTRING);
        return;
    };
    match cmd {
        "help" => {
            if let Some(arg) = input {
                // check if there is more than one argument to 'help'
                if let Some(_) = input {
                    help("help");
                    return
                }
                help(arg);
            } else {
                help("");
            }
        }
        "pgnsave" => {
            if let Some(path) = input {
                //check if there is more than one argument to 'pgnsave'
                if let Some(_) = input {
                    help("pgnsave");
                    return;
                }
            }
        }
        // we assume initially that it is a chess move, but we'll see
        _ => {
            // chess moves aren't supposed to have args
            if let Some(_) = input {
                help("move");
                return;
            }
            make_move(cmd);
        }
    }
}

fn tick(current_turn: u8, to_move: ToMove) -> (u8, ToMove) {

}
