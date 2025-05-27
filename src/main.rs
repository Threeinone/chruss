use chess::board::*;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

#[derive(PartialEq)]
enum ToMove {
    White,
    Black
}

static TO_MOVE: ToMove = ToMove::White;
static TURN: i32 = 1;
static USE_COORDS: bool = false;

fn main() -> Result<()> {
    //(ext) `()` can be used when no completer is required
    let mut rl = DefaultEditor::new()?;
    loop {
        let prompt = if TO_MOVE == ToMove::White {
            format!("White ({}) : ", TURN)
        } else {
            format!("Black ({}) : ", TURN)
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
            }
            /***/
            Ok(line) => {
                rl.add_history_entry(line.as_str())?;
                match line {
                    _ => {println!("Line: {}", line);}
                }
            },
        }
    }
    Ok(())
}
