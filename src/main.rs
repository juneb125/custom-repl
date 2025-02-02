use std::io::{self, Result as IOResult, Write};

fn main() -> IOResult<()> {
    println!("Hello, world!\n");

    let cl_prompt: &str = "(this-repl)> ";
    let mut stdout = io::stdout().lock();

    'main_loop: loop {
        write!(stdout, "{}", cl_prompt)?;
        stdout.flush()?;

        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let input_fmt: &str = input.as_str().trim();

        // inspiration from the nix repl for cmd's that begin w/ ':'
        match input_fmt {
            ":q" => {
                writeln!(stdout, "See you next time :)")?;
                break 'main_loop;
            }
            i if i.starts_with(':') => {
                // let stripped_i = i.strip_prefix(':').unwrap();
                //
                // handle special cmd's...
                // match stripped_i {}

                writeln!(stdout, "You entered: {}", i)?
            }
            "" => return Ok(()), // early return if empty
            i => writeln!(stdout, "You entered: {}", i)?,
        }

        stdout.flush()?
    }

    Ok(())
}
