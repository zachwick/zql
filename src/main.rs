use rustyline::error::ReadlineError;
use rustyline::Editor;

fn main() {
    // This line creates an Editor with the defaul configuration options
    let mut repl = Editor::<()>::new();

    // This if statement loads a file with the history of commands
    // If the file does not exist, then one is created
    if repl.load_history("history.txt").is_err() {
        println!("no previous history");
    }

    // This is the infinite loop that runs until the user terminates the program
    loop {
        // Ask the user to input a command
        let readline = repl.readline(">> ");

        // The readline method returns a Result type oblect. This object is used
        // as a match statement to filter the result.
        match readline {
            Ok(line) => {
                repl.add_history_entry(line.as_str());
                println!("Line: {}", line);
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
    }

    // Write the command history from memory to disk
    repl.save_history("history.txt").unwrap();
}
