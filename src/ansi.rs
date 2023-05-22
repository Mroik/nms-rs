use std::fmt::Display;

pub enum AnsiCodes {
    CursorUp(u16),
}

impl Display for AnsiCodes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::CursorUp(n) => write!(f, "\x1B[{}A", n)
        }
    }
}
