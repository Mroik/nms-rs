use rand::{self, distributions::Uniform, prelude::Distribution};
use std::{
    fmt::Display,
    io::{stdin, stdout, Read, Write},
    print,
    thread::sleep,
    time::Duration,
    write,
};

mod ansi;
use ansi::AnsiCodes::{self, CursorUp, Reset};

const PAUSE_TIME: u64 = 25;
const MASK_CHARS: [char; 253] = [
    '\u{263A}', '\u{263B}', '\u{2665}', '\u{2666}', '\u{2663}', '\u{2660}', '\u{2022}', '\u{25D8}',
    '\u{25CB}', '\u{25D9}', '\u{2642}', '\u{2640}', '\u{266A}', '\u{266B}', '\u{263C}', '\u{25BA}',
    '\u{25C4}', '\u{2195}', '\u{203C}', '\u{00B6}', '\u{00A7}', '\u{25AC}', '\u{21A8}', '\u{2191}',
    '\u{2193}', '\u{2192}', '\u{2190}', '\u{221F}', '\u{2194}', '\u{25B2}', '\u{25BC}', '!', '"',
    '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', '0', '1', '2', '3', '4', '5',
    '6', '7', '8', '9', ':', ';', '<', '=', '>', '?', '@', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H',
    'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '[',
    '\\', ']', '^', '_', '`', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n',
    'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '{', '|', '}', '~', '\u{2302}',
    '\u{00C7}', '\u{00FC}', '\u{00E9}', '\u{00E2}', '\u{00E4}', '\u{00E0}', '\u{00E5}', '\u{00E7}',
    '\u{00EA}', '\u{00EB}', '\u{00E8}', '\u{00EF}', '\u{00EE}', '\u{00EC}', '\u{00C4}', '\u{00C5}',
    '\u{00C9}', '\u{00E6}', '\u{00C6}', '\u{00F4}', '\u{00F6}', '\u{00F2}', '\u{00FB}', '\u{00F9}',
    '\u{00FF}', '\u{00D6}', '\u{00DC}', '\u{00A2}', '\u{00A3}', '\u{00A5}', '\u{20A7}', '\u{0192}',
    '\u{00E1}', '\u{00ED}', '\u{00F3}', '\u{00FA}', '\u{00F1}', '\u{00D1}', '\u{00AA}', '\u{00BA}',
    '\u{00BF}', '\u{2310}', '\u{00AC}', '\u{00BD}', '\u{00BC}', '\u{00A1}', '\u{00AB}', '\u{00BB}',
    '\u{2591}', '\u{2592}', '\u{2593}', '\u{2502}', '\u{2524}', '\u{2561}', '\u{2562}', '\u{2556}',
    '\u{2555}', '\u{2563}', '\u{2551}', '\u{2557}', '\u{255D}', '\u{255C}', '\u{255B}', '\u{2510}',
    '\u{2514}', '\u{2534}', '\u{252C}', '\u{251C}', '\u{2500}', '\u{253C}', '\u{255E}', '\u{255F}',
    '\u{255A}', '\u{2554}', '\u{2569}', '\u{2566}', '\u{2560}', '\u{2550}', '\u{256C}', '\u{2567}',
    '\u{2568}', '\u{2564}', '\u{2565}', '\u{2559}', '\u{2558}', '\u{2552}', '\u{2553}', '\u{256B}',
    '\u{256A}', '\u{2518}', '\u{250C}', '\u{2588}', '\u{2584}', '\u{258C}', '\u{2590}', '\u{2580}',
    '\u{03B1}', '\u{00DF}', '\u{0393}', '\u{03C0}', '\u{03A3}', '\u{03C3}', '\u{00B5}', '\u{03C4}',
    '\u{03A6}', '\u{0398}', '\u{03A9}', '\u{03B4}', '\u{221E}', '\u{03C6}', '\u{03B5}', '\u{2229}',
    '\u{2261}', '\u{00B1}', '\u{2265}', '\u{2264}', '\u{2320}', '\u{2321}', '\u{00F7}', '\u{2248}',
    '\u{00B0}', '\u{2219}', '\u{00B7}', '\u{221A}', '\u{207F}', '\u{00B2}', '\u{25A0}',
];

struct HiddenChar {
    src: char,
    mask: Option<char>,
    ansi_code: AnsiCodes,
}

impl Display for HiddenChar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.mask {
            None => write!(f, "{}{}", self.ansi_code, self.src),
            Some(c) => write!(f, "{}{}", Reset, c),
        }
    }
}

fn parse_input(input: &str) -> Vec<HiddenChar> {
    let mut rng = rand::thread_rng();
    let gen = Uniform::from(0..253);
    let mut current_code = Reset;

    let cc = input.replace('\t', "        ");

    let mut i = 0;
    let mut ris = Vec::new();

    while i < cc.len() {
        let found_c = cc.chars().nth(i).unwrap();
        let n_mask = if found_c == ' ' || found_c == '\n' {
            None
        } else {
            Some(MASK_CHARS[gen.sample(&mut rng)])
        };
        let hc = if let Some((code, new_i)) = AnsiCodes::new(&cc[i..]) {
            i += new_i + 1;
            current_code = code;
            HiddenChar {
                src: cc.chars().nth(new_i).unwrap(),
                mask: n_mask,
                ansi_code: current_code,
            }
        } else {
            i += 1;
            HiddenChar {
                src: cc.chars().nth(i - 1).unwrap(),
                mask: n_mask,
                ansi_code: current_code,
            }
        };
        ris.push(hc);
    }
    return ris;
}

fn print_hidden(text: &Vec<HiddenChar>) {
    let mut s = String::new();
    for c in text {
        s.push_str(&c.to_string());
    }
    print!("{s}");
}

fn decrypt(text: &mut Vec<HiddenChar>) {
    let mut rng = rand::thread_rng();
    let rr = Uniform::from(0..253);

    for _ in 0..40 {
        sleep(Duration::from_millis(PAUSE_TIME));
        text.iter_mut().for_each(|c| match c.mask {
            None => (),
            Some(_) => c.mask = Some(MASK_CHARS[rr.sample(&mut rng)]),
        });
        print!(
            "{}",
            CursorUp(text.iter().filter(|c| c.src == '\n').count() as u16)
        );
        print_hidden(text);
    }

    // Start actual decrypt
    let mut non_enc: Vec<usize> = text
        .iter()
        .zip((0..text.len()).collect::<Vec<usize>>())
        .filter(|(c, _)| c.mask.is_some())
        .map(|(_, i)| i)
        .collect();

    loop {
        if non_enc.is_empty() {
            break;
        }

        let ll = Uniform::from(0..non_enc.len());
        let selected = ll.sample(&mut rng);
        text[non_enc[selected]].mask = None;
        non_enc.remove(selected);

        // Pass again hidden
        for c in &mut *text {
            match c.mask {
                None => (),
                _ => c.mask = Some(MASK_CHARS[rr.sample(&mut rng)]),
            }
        }

        print!(
            "{}",
            CursorUp(text.iter().filter(|c| c.src == '\n').count() as u16)
        );
        print_hidden(text);
        sleep(Duration::from_millis(PAUSE_TIME));
    }
}

fn main() -> std::io::Result<()> {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut text = parse_input(&buf);

    for c in &text {
        match c.mask {
            None => print!("{}", c.src),
            Some(cc) => print!("{cc}"),
        }
        stdout().flush()?;
        sleep(Duration::from_millis(PAUSE_TIME));
    }

    decrypt(&mut text);
    return Ok(());
}
