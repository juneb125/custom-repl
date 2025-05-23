use std::io::{self, Result as IOResult, Write};

mod color;
mod lexer;
mod token;
mod utils;

fn main() -> IOResult<()> {
    let mut stdout = io::stdout().lock();

    let cl_prompt: &str = "\n(this-repl)> ";
    let secondary_prompt: &str = ">> ";

    'main_loop: loop {
        write!(stdout, "{}", cl_prompt)?;
        stdout.flush()?;

        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).expect("Couldn't read input");

        while input.strip_end_nl_sp().ends_with('\\') {
            write!(stdout, "{}", secondary_prompt)?;
            stdout.flush()?;

            let mut new_buf: String = String::new();
            stdin.read_line(&mut new_buf).expect("Couldn't read input");

            input = format!("{}\n{}", input.prepare_for_append(), new_buf);
            new_buf = "".to_string();
        }

        let formatted_input: &str = input.as_str().trim();
        if formatted_input.len() == 0 {
            continue 'main_loop;
        }

        let argv: Vec<char> = formatted_input.chars().collect();

        // inspiration from the nix repl for cmd's that begin w/ ':'
        match argv[0] {
            ':' => match argv[1..] {
                [_, _, ..] => errRepl!(error, "unknown command")?,
                ['q', ..] => {
                    writeln!(stdout, "See you next time :)")?;
                    stdout.flush()?;
                    break 'main_loop;
                }
                ['h', ..] | ['?', ..] => writeln!(stdout, "Print help info")?,
                // more to come...
                _ => errRepl!(error, "unknown command")?,
            },
            '\'' => warnRepl!(todo, "character parsing")?,
            '\"' => warnRepl!(todo, "string parsing")?,
            _ => warnRepl!("TODO!: actually parse the input")?,
        }

        stdout.flush()?
    }

    Ok(())
}

trait CustomStripSuffix {
    fn strip_end_nl_sp(&self) -> Self;
    fn prepare_for_append(&self) -> Self;
}

impl CustomStripSuffix for String {
    /// pops off the last character until it hits a char that
    /// isn't any of these: `[' ', '\n', '\t']`
    fn strip_end_nl_sp(&self) -> Self {
        let mut abc = format!("{}", self);
        while let Some(ch) = abc.pop() {
            if !matches!(ch, ' ' | '\n' | '\t') {
                return format!("{}{}", abc, ch);
            }
        }
        return format!("{}", abc);
    }

    /// shorthand for:
    /// ```no_run
    /// self
    ///   .strip_end_nl_sp()
    ///   .strip_suffix('\\')
    ///   .unwrap_or(self.as_str())
    ///   .to_string()
    /// ```
    fn prepare_for_append(&self) -> Self {
        let foo = self.strip_end_nl_sp();
        match foo.strip_suffix('\\') {
            Some(i) => i.to_string(),
            None => foo,
        }
    }
}
