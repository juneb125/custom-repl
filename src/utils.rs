pub mod macros {
    #[macro_export]
    macro_rules! warnRepl {
        () => {{
            writeln!($crate::io::stdout(), "\x1b[33mWarning!\x1b[m")
            }};
        (todo, $($args:tt)*) => {{
            writeln!($crate::io::stdout(), "\x1b[33mTodo: {}\x1b[m", format_args!($($args)*))
            }};
        ($($args:tt)*) => {{
            writeln!($crate::io::stdout(), "\x1b[33m{}\x1b[m", format_args!($($args)*))
            }};
    }

    #[macro_export]
    macro_rules! errRepl {
        () => {{
            writeln!($crate::io::stdout(), "\x1b[31mError!\x1b[m")
            }};
        (error, $($args:tt)*) => {{
            writeln!($crate::io::stdout(), "\x1b[31mError: {}\x1b[m", format_args!($($args)*))
            }};
        ($($args:tt)*) => {{
            writeln!($crate::io::stdout(), "\x1b[31m{}\x1b[m", format_args!($($args)*))
            }};
    }
}
