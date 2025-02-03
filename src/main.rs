use std::io::{self, Result as IOResult, Write};

mod types;
use types::types::parse_to_type;

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
        let argv: Vec<&str> = formatted_input.split(' ').collect();

        // inspiration from the nix repl for cmd's that begin w/ ':'
        match argv[0] {
            ":q" => {
                writeln!(stdout, "See you next time :)")?;
                break 'main_loop;
            }
            i if i.starts_with(':') => {
                let stripped_i = i.strip_prefix(':').unwrap();
                parse_colon_cmd(stripped_i, &mut stdout)?
            }
            "" => (),
            i => match parse_to_type(i) {
                Ok(val) => {
                    writeln!(stdout, "You entered: {:?}", i)?;
                    writeln!(stdout, "Your parsed input is: {:?}", val)?;
                }
                Err(error) => writeln!(stdout, "{}", error)?,
            },
        }

        stdout.flush()?
    }

    Ok(())
}

fn parse_colon_cmd<T: Write>(cmd: &str, output: &mut T) -> IOResult<()> {
    match cmd {
        "h" | "help" | "?" => writeln!(output, "Print help info"),
        // more to come...
        _ => writeln!(output, "Error: command not found"),
    }
}
