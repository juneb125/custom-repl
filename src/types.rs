pub mod types {
    use crate::color::color::Color;

    #[allow(unused)]
    #[derive(Debug)]
    pub enum Primitive<'a> {
        Int(i32),
        Float(f32),

        Char(char),
        String(&'a str),

        Bool(bool),
    }

    fn is_surrounded(value: &str, ch: char) -> bool {
        value.starts_with(ch) && value.ends_with(ch)
    }

    pub fn parse_to_type(value: &str) -> Result<Primitive, String> {
        const DIGITS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

        use Primitive as Type;
        match value {
            "true" => Ok(Type::Bool(true)),
            "false" => Ok(Type::Bool(false)),
            i if is_surrounded(i, '\'') => {
                // TODO: parse chars
                Err("Todo: parsing to Primitive::Char is not yet implemented".set_fg(3))
            }
            i if is_surrounded(i, '\"') => {
                // BUG: since argv (in main.rs) is split by space, there isn't a way right now to represent strings w/ more than 1 word
                // gets rid of the dbl quotes around 'i'
                Ok(Type::String(&i[1..(i.len() - 1)]))
            }
            i if i.contains(DIGITS) => match i.contains('.') {
                true => match i.parse::<f32>() {
                    Ok(val) => Ok(Type::Float(val)),
                    Err(e) => Err(format!("Error: {e}").set_fg(1)),
                },
                false => match i.parse::<i32>() {
                    Ok(val) => Ok(Type::Int(val)),
                    Err(e) => Err(format!("Error: {e}").set_fg(1)),
                },
            },
            _ => Err("Error: could not convert input to a internal/builtin type".set_fg(1)),
        }
    }
}
