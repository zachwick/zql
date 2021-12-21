use std::borrow::Cow::{self, Borrowed, Owned};

use rustyline_derive::{Helper, Completer};
use rustyline::error::ReadlineError;
use rustyline::config::OutputStreamType;
use rustyline::{CompletionType, Config, Context, EditMode};
use rustyline::validate::{MatchingBracketValidator, Validator};
use rustyline::validate::{ValidationContext, ValidationResult};
use rustyline::hint::{Hinter, HistoryHinter};
use rustyline::highlight::{Highlighter, MatchingBracketHighlighter};

// REPL Helper Struct with all functionalities
#[derive(Helper, Completer)]
pub struct REPLHelper {
    pub validator: MatchingBracketValidator,
    pub colored_prompt: String,
    pub hinter: HistoryHinter,
    pub highlighter: MatchingBracketHighlighter,
}

impl REPLHelper {
    // Default constructor
    pub fn new() -> Self {
        REPLHelper {
            // completer: FilenameCompleter::new(),
            highlighter: MatchingBracketHighlighter::new(),
            hinter: HistoryHinter {},
            colored_prompt: "".to_owned(),
            validator: MatchingBracketValidator::new(),
        }
    }
}

// Implementing trait responsible for providing hints
impl Hinter for REPLHelper {
    type Hint = String;

    // Takes the currently edited line with the cursor position and returns the string that should be 
    // displayed or None if no hint is available for the text the user currently typed
    fn hint(&self, line: &str, pos: usize, ctx: &Context<'_>) -> Option<String> {
        self.hinter.hint(line, pos, ctx)
    }
}

// Implementing trait responsible for determining whether the current input buffer is valid.
// Rustyline uses the method provided by this trait to decide whether hitting the enter key 
// will end the current editing session and return the current line buffer to the caller of 
// Editor::readline or variants.
impl Validator for REPLHelper {

    // Takes the currently edited input and returns a ValidationResult indicating whether it 
    // is valid or not along with an option message to display about the result.
    fn validate(&self, ctx: &mut ValidationContext) -> Result<ValidationResult, ReadlineError> {
        use ValidationResult::{Incomplete, /*Invalid,*/ Valid};
        let input = ctx.input();
        // let result = if !input.starts_with("SELECT") {
        //     Invalid(Some(" --< Expect: SELECT stmt".to_owned()))
        // } else 
        let result = if input.eq(".exit") {
            Valid(None)
        } else if !input.ends_with(';') {
            Incomplete
        } else {
            Valid(None)
        };
        Ok(result)
    }

    // Configure whether validation is performed while typing or only when user presses the Enter key.
    fn validate_while_typing(&self) -> bool {
        self.validator.validate_while_typing()
    }
}

// Implementing syntax highlighter with ANSI color.
impl Highlighter for REPLHelper {
    // Takes the prompt and returns the highlighted version (with ANSI color).
    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(&'s self, prompt: &'p str, default: bool,) -> Cow<'b, str> {
        if default {
            Borrowed(&self.colored_prompt)
        } else {
            Borrowed(prompt)
        }
    }

    // Takes the hint and returns the highlighted version (with ANSI color).
    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        Owned("\x1b[1m".to_owned() + hint + "\x1b[m")
    }

    // Takes the currently edited line with the cursor position and returns the highlighted version (with ANSI color).
    fn highlight<'l>(&self, line: &'l str, pos: usize) -> Cow<'l, str> {
        self.highlighter.highlight(line, pos)
    }

    // Tells if line needs to be highlighted when a specific char is typed or when cursor is moved under a specific char.
    // Used to optimize refresh when a character is inserted or the cursor is moved.
    fn highlight_char(&self, line: &str, pos: usize) -> bool {
        self.highlighter.highlight_char(line, pos)
    }
}

// Returns a Config::builder with basic Editor configuration
pub fn get_config() -> Config {
    Config::builder()
        .history_ignore_space(true)
        .completion_type(CompletionType::List)
        .edit_mode(EditMode::Emacs)
        .output_stream(OutputStreamType::Stdout)
        .build()
}