use std::{fmt::Display, write};

#[derive(Clone, Copy)]
pub enum AnsiCodes {
    Bell,
    CursorUp(u16),
    Reset,
    SGR(u16),
    SGR2(u16, u16),
}

impl Display for AnsiCodes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Bell => write!(f, "\x07"),
            Self::CursorUp(n) => write!(f, "\x1B[{}A", n),
            Self::Reset => write!(f, "\x1B[0m"),
            Self::SGR(n) => write!(f, "\x1B[{}m", n),
            Self::SGR2(n, m) => write!(f, "\x1B[{};{}m", n, m),
        }
    }
}

impl AnsiCodes {
    pub fn new(st: &str) -> Option<(Self, usize)> {
        if st.starts_with('\x07') {
            return Some((Self::Bell, 1));
        }
        if !st.starts_with("\x1B[") {
            return None;
        }

        if let Some(l) = st.find('m') {
            let div = st.find(';').unwrap_or(usize::MAX);
            if div < l {
                match (st[2..div].parse::<u16>(), st[(div+1)..l].parse::<u16>()) {
                    (Err(_), _) => return None,
                    (_, Err(_)) => return None,
                    (Ok(n), Ok(m)) => return Some((
                            Self::SGR2(n, m),
                            l
                    ))
                }
            } else {
                match st[2..l].parse() {
                    Err(_) => return None,
                    Ok(n) => return Some((
                            Self::SGR(n),
                            l
                    )),
                }
            }
        }
        return None
    }
}
