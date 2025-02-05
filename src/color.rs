use std::fmt::Display;

#[allow(dead_code)]
pub trait Color {
    fn set_fg(&self, ansi_color: u8) -> String;
    fn set_bg(&self, ansi_color: u8) -> String;
}

/*
 * black: 0, red: 1,     green: 2, yellow: 3,
 * blue: 4,  magenta: 5, cyan: 6,  white: 7
 */

impl<T: Display> Color for T {
    fn set_fg(&self, ansi_color: u8) -> String {
        match ansi_color {
            0..7 => format!("\x1b[3{}m{}\x1b[m", ansi_color, self),
            _ => format!("\x1b[38;5;{}m{}\x1b[m", ansi_color, self),
        }
    }

    fn set_bg(&self, ansi_color: u8) -> String {
        match ansi_color {
            0..7 => format!("\x1b[4{}m{}\x1b[m", ansi_color, self),
            _ => format!("\x1b[48;5;{}m{}\x1b[m", ansi_color, self),
        }
    }
}
