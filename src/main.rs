extern crate clap;
mod repl;

use repl::{REPLHelper, get_config};

use rustyline::error::ReadlineError;
use rustyline::Editor;

use clap::{App, crate_version};

fn main() -> rustyline::Result<()> {
    env_logger::init();

    // Basic implementation using no arguments.
    let _matches = App::new("zql")
                        .version("0.0.1")
                        .author("zach wick <zach@zachwick.com>")
                        .about("A minimalist sqlite-alike")
                        .get_matches();

    // Start rustyline with default configuration
    let config = get_config();

    // Get a new rustyline helper
    let helper = REPLHelper::new();

    // This line creates an Editor with the default configuration options
    let mut repl = Editor::with_config(config);
    repl.set_helper(Some(helper));

    // This if statement loads a file with the history of commands
    // If the file does not exist, then one is created
    if repl.load_history("history.txt").is_err() {
        println!("no previous history");
    }

    // Use a counter to show how many commands the user has run
    let mut count = 1;

    // This is the infinite loop that runs until the user terminates the program
    loop {
        if count == 1 {
            // Show a message to the user on their first command
            println!("{}{}{}{}{}",
                format!("zql - {}\n", crate_version!()),
                "Use .exit to quit.\n",
                "Use .help for usage hints.\n",
                "Connected to a transient in-memory database.\n",
                "Use '.open FILENAME' to reopen a persisten database.");
        }

        let p = format!("zql | {}> ", count);
        repl.helper_mut()
            .expect("No helper found")
            .colored_prompt = format!("\x1b[1;32m{}\x1b[0m", p);

        // Ask the user to input a command
        let readline = repl.readline(&p);

        // The readline method returns a Result type oblect. This object is used
        // as a match statement to filter the result.
        match readline {
            Ok(command) => {
                repl.add_history_entry(command.as_str());
                if command.eq(".exit") {
                    break;
                } else {
                    println!("Error: unknown command or invalid arguments: '{}'. Use '.help' for usage hints.", &command);
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
        count += 1;
    }

    // Write the command history from memory to disk
    repl.save_history("history.txt").unwrap();

    Ok(())
}
