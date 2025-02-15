use std::io::{self, Result as IOResult, Write};

mod color;
mod lexer;
mod token;
mod utils;

fn main() -> IOResult<()> {
    println!("Hello, world!");

    let cl_prompt: &str = "\n(this-repl)> ";
    let mut stdout = io::stdout().lock();

    'main_loop: loop {
        write!(stdout, "{}", cl_prompt)?;
        stdout.flush()?;

        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let formatted_input: &str = input.as_str().trim();
        let argv: Vec<char> = formatted_input.chars().collect();

        if argv.len() == 0 {
            continue 'main_loop;
        }

        // inspiration from the nix repl for cmd's that begin w/ ':'
        match argv[0] {
            ':' => match argv[1..] {
                [_, _, ..] => errRepl!(error, "unknown command")?,
                ['q', ..] => {
                    writeln!(stdout, "See you next time :)")?;
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
