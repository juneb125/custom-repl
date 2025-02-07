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

    // Int Arithmetic
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
        let token_fmt = match self {
            // TODO: figure out how to format literals
            // literals
            T::Identifier(val) => *val,
            // T::Int(i) => " ",
            // T::Float(i) => " ",
            // T::Char(i) => " ",
            T::String(i) => i,
            T::Bool(true) => "true",
            T::Bool(false) => "false",

            // grouping
            T::LParen => "(",
            T::RParen => ")",
            T::LBracket => "[",
            T::RBracket => "]",
            T::LBrace => "{",
            T::RBrace => "}",

            // int arithmetic
            T::Plus => "+",
            T::Minus => "-",
            T::Star => "*",
            T::Slash => "/",
            // misc. number ops
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

            _ => "== Unknown ==",
        };

        write!(f, "{token_fmt}")
    }
}
