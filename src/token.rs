use std::fmt;

#[allow(dead_code)]
#[derive(Debug)]
pub enum Token<'a> {
    // Literals
    Identifier(&'a str),

    Int(i32),
    Float(f32),

    Char(char),
    String(&'a str),

    Bool(bool),

    // Grouping
    LParen,   // (
    RParen,   // )
    LBracket, // [
    RBracket, // ]
    LBrace,   // {
    RBrace,   // }

    // Number Ops
    Plus,     // +
    Minus,    // -
    Star,     // *
    Slash,    // /
    Equal,    // =
    Percent,  // %
    StarStar, // **

    // Comparison
    EqualEqual,   // ==
    BangEqual,    // !=
    Greater,      // >
    Less,         // <
    GreaterEqual, // >=
    LessEqual,    // <=

    // Logical Ops
    Bang,       // !
    AmpAmp,     // &&
    VbarVbar,   // ||
    CarotCarot, // ^^

    // Punctuation
    Dot,         // .
    Comma,       // ,
    Colon,       // :
    Semicolon,   // ;
    Question,    // ?
    SingleQuote, // '
    DblQuote,    // "
    Underscore,  // _

    // Misc.
    PlusPlus, // ++
    LtGt,     // <>
    Pipe,     // |>
    DotDot,   // ..
    Vbar,     // |

    Eof,
}

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Token as T;
        let token_fmt: &str = match self {
            // literals
            T::Identifier(val) => *val,
            T::Int(val) => return write!(f, "{val}"),
            T::Float(val) => return write!(f, "{val}"),
            T::Char(val) => return write!(f, "{val}"),
            T::String(val) => *val,
            T::Bool(val) => return write!(f, "{val}"),

            // grouping
            T::LParen => "(",
            T::RParen => ")",
            T::LBracket => "[",
            T::RBracket => "]",
            T::LBrace => "{",
            T::RBrace => "}",

            // number ops
            T::Plus => "+",
            T::Minus => "-",
            T::Star => "*",
            T::Slash => "/",
            T::Equal => "=",
            T::Percent => "%",
            T::StarStar => "**",

            // comparison
            T::EqualEqual => "==",
            T::BangEqual => "!=",
            T::Greater => "<",
            T::Less => "<",
            T::GreaterEqual => ">=",
            T::LessEqual => "<=",

            // logical
            T::Bang => "!",
            T::AmpAmp => "&&",
            T::VbarVbar => "||",
            T::CarotCarot => "^^",

            // punctuation
            T::Dot => ".",
            T::Comma => ",",
            T::Colon => ":",
            T::Semicolon => ";",
            T::Question => "?",
            T::SingleQuote => "\'",
            T::DblQuote => "\"",
            T::Underscore => "_",

            // misc.
            T::PlusPlus => "++",
            T::LtGt => "<>",
            T::Pipe => "|>",
            T::DotDot => "..",
            T::Vbar => "|",
            T::Eof => "EOF",
        };

        write!(f, "{token_fmt}")
    }
}
