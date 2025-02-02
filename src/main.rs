use std::io::{self, Result as IOResult, Write};

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
            "" => return Ok(()),
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

#[allow(unused)]
#[derive(Debug)]
enum Primitive<'a> {
    Int(i32),
    Float(f32),

    Char(char),
    String(&'a str),

    Bool(bool),
}

fn is_surrounded(value: &str, ch: char) -> bool {
    value.starts_with(ch) && value.ends_with(ch)
}

fn parse_to_type(value: &str) -> Result<Primitive, String> {
    const DIGITS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

    use Primitive as Type;
    match value {
        "true" => Ok(Type::Bool(true)),
        "false" => Ok(Type::Bool(false)),
        i if is_surrounded(i, '\'') => {
            // TODO: parse chars
            Err("TODO: Implement parsing to Primitive::Char(_)".to_string())
        }
        i if is_surrounded(i, '\"') => {
            // BUG: since argv (in main()) is split by space, there isn't a way right now to represent strings w/ more than 1 word
            // gets rid of the dbl quotes around 'i'
            Ok(Type::String(&i[1..(i.len() - 1)]))
        }
        i if i.contains(DIGITS) => match i.contains('.') {
            true => match i.parse::<f32>() {
                Ok(val) => Ok(Primitive::Float(val)),
                Err(e) => Err(e.to_string()),
            },
            false => match i.parse::<i32>() {
                Ok(val) => Ok(Primitive::Int(val)),
                Err(e) => Err(e.to_string()),
            },
        },
        _ => Err("Error: could not convert input to a internal/builtin type".to_string()),
    }
}
