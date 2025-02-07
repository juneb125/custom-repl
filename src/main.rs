use std::io::{self, Result as IOResult, Write};

mod color;
mod lexer;
mod token;

use crate::color::Color;

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
                ['q', _] => {
                    writeln!(stdout, "See you next time :)")?;
                    break 'main_loop;
                }
                _ => parse_colon_cmd(&argv[1..], &mut stdout)?,
            },
            '\'' => writeln!(stdout, "{}", "Todo: character parsing".set_fg(3))?,
            '\"' => writeln!(stdout, "{}", "Todo: string parsing".set_fg(3))?,
            _ => writeln!(stdout, "????")?,
        }

        stdout.flush()?
    }

    Ok(())
}

fn parse_colon_cmd<T: Write>(cmd: &[char], output: &mut T) -> IOResult<()> {
    match cmd {
        ['h', 'e', 'l', 'p', ..] => writeln!(output, "Print help info"),
        ['h', ..] | ['?', ..] => writeln!(output, "Print help info"),
        // more to come...
        _ => writeln!(output, "{}", "Error: command not found".set_fg(1)),
    }
}
