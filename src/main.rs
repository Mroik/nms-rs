use std::{println, print, io::{stdin, Read}, thread::sleep, time::Duration};
use ansi_escapes::{EraseLine, CursorUp};
use rand::{self, distributions::Uniform, prelude::Distribution};

const MASK_CHARS: [char; 253] = [
    '\u{263A}', '\u{263B}', '\u{2665}',
    '\u{2666}', '\u{2663}', '\u{2660}', '\u{2022}',
    '\u{25D8}', '\u{25CB}', '\u{25D9}', '\u{2642}',
    '\u{2640}', '\u{266A}', '\u{266B}', '\u{263C}',
    '\u{25BA}', '\u{25C4}', '\u{2195}', '\u{203C}',
    '\u{00B6}', '\u{00A7}', '\u{25AC}', '\u{21A8}',
    '\u{2191}', '\u{2193}', '\u{2192}', '\u{2190}',
    '\u{221F}', '\u{2194}', '\u{25B2}', '\u{25BC}',
    '!', '"', '#',
    '$', '%', '&', '\'',
    '(', ')', '*', '+',
    ',', '-', '.', '/',
    '0', '1', '2', '3',
    '4', '5', '6', '7',
    '8', '9', ':', ';',
    '<', '=', '>', '?',
    '@', 'A', 'B', 'C',
    'D', 'E', 'F', 'G',
    'H', 'I', 'J', 'K',
    'L', 'M', 'N', 'O',
    'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W',
    'X', 'Y', 'Z', '[',
    '\\', ']', '^', '_',
    '`', 'a', 'b', 'c',
    'd', 'e', 'f', 'g',
    'h', 'i', 'j', 'k',
    'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's',
    't', 'u', 'v', 'w',
    'x', 'y', 'z', '{',
    '|', '}', '~', '\u{2302}',
    '\u{00C7}', '\u{00FC}', '\u{00E9}', '\u{00E2}',
    '\u{00E4}', '\u{00E0}', '\u{00E5}', '\u{00E7}',
    '\u{00EA}', '\u{00EB}', '\u{00E8}', '\u{00EF}',
    '\u{00EE}', '\u{00EC}', '\u{00C4}', '\u{00C5}',
    '\u{00C9}', '\u{00E6}', '\u{00C6}', '\u{00F4}',
    '\u{00F6}', '\u{00F2}', '\u{00FB}', '\u{00F9}',
    '\u{00FF}', '\u{00D6}', '\u{00DC}', '\u{00A2}',
    '\u{00A3}', '\u{00A5}', '\u{20A7}', '\u{0192}',
    '\u{00E1}', '\u{00ED}', '\u{00F3}', '\u{00FA}',
    '\u{00F1}', '\u{00D1}', '\u{00AA}', '\u{00BA}',
    '\u{00BF}', '\u{2310}', '\u{00AC}', '\u{00BD}',
    '\u{00BC}', '\u{00A1}', '\u{00AB}', '\u{00BB}',
    '\u{2591}', '\u{2592}', '\u{2593}', '\u{2502}',
    '\u{2524}', '\u{2561}', '\u{2562}', '\u{2556}',
    '\u{2555}', '\u{2563}', '\u{2551}', '\u{2557}',
    '\u{255D}', '\u{255C}', '\u{255B}', '\u{2510}',
    '\u{2514}', '\u{2534}', '\u{252C}', '\u{251C}',
    '\u{2500}', '\u{253C}', '\u{255E}', '\u{255F}',
    '\u{255A}', '\u{2554}', '\u{2569}', '\u{2566}',
    '\u{2560}', '\u{2550}', '\u{256C}', '\u{2567}',
    '\u{2568}', '\u{2564}', '\u{2565}', '\u{2559}',
    '\u{2558}', '\u{2552}', '\u{2553}', '\u{256B}',
    '\u{256A}', '\u{2518}', '\u{250C}', '\u{2588}',
    '\u{2584}', '\u{258C}', '\u{2590}', '\u{2580}',
    '\u{03B1}', '\u{00DF}', '\u{0393}', '\u{03C0}',
    '\u{03A3}', '\u{03C3}', '\u{00B5}', '\u{03C4}',
    '\u{03A6}', '\u{0398}', '\u{03A9}', '\u{03B4}',
    '\u{221E}', '\u{03C6}', '\u{03B5}', '\u{2229}',
    '\u{2261}', '\u{00B1}', '\u{2265}', '\u{2264}',
    '\u{2320}', '\u{2321}', '\u{00F7}', '\u{2248}',
    '\u{00B0}', '\u{2219}', '\u{00B7}', '\u{221A}',
    '\u{207F}', '\u{00B2}', '\u{25A0}'
];

struct HiddenChar {
    src: char,
    mask: char,
}

fn parse_input(input: &String) -> Vec<Vec<HiddenChar>> {
    let mut rng = rand::thread_rng();
    let rr = Uniform::from(0..153);

    return input
        .split("\n")
        .filter(|line| line.len() > 0)
        .map(|line| line
             .chars()
             .map(|c| HiddenChar {src: c, mask: MASK_CHARS[rr.sample(&mut rng)]})
             .collect()
        )
        .collect();
}

fn print_hidden(text: &Vec<Vec<HiddenChar>>) {
    let lines = text
        .iter()
        .map(|line| {
            let t = line
                .iter()
                .map(|c| c.mask);
            String::from_iter(t)
        })
        .collect::<Vec<String>>();
    for line in lines {
        println!("{line}");
    }
}

fn decrypt(text: &mut Vec<Vec<HiddenChar>>) {
    let mut rng = rand::thread_rng();
    let rr = Uniform::from(0..153);

    let mut enc_lines: Vec<usize> = (0..text.len()).collect();
    for _ in 0..40 {
        sleep(Duration::from_millis(40));
        text
            .iter_mut()
            .for_each(|line| {
                line
                    .iter_mut()
                    .for_each(|c| c.mask = MASK_CHARS[rr.sample(&mut rng)])
            });
        for _ in 0..text.len() {
            print!("{}", CursorUp(1));
            print!("{}", EraseLine);
        }
        print_hidden(text);
    }

    // Start actual decrypt
    loop {
        let nn = Uniform::from(0..enc_lines.len());
        let chosen_line = enc_lines[nn.sample(&mut rng)];

        let non_enc: Vec<usize> = text[chosen_line]
            .iter()
            .zip((0..text[chosen_line].len()).collect::<Vec<usize>>())
            .filter(|(c, _)| c.mask != c.src)
            .map(|(_, i)| i)
            .collect();
        if non_enc.len() == 0 {
            enc_lines.retain(|&it| it != chosen_line);
            if enc_lines.len() == 0 {
                break;
            }
            continue;
        }

        let ll = Uniform::from(0..non_enc.len());
        let col = non_enc[ll.sample(&mut rng)];
        text[chosen_line][col].mask = text[chosen_line][col].src;

        // Pass again hidden
        for line in &mut *text {
            for c in line {
                if c.src != c.mask {
                    c.mask = MASK_CHARS[rr.sample(&mut rng)];
                }
            }
        }

        for _ in 0..text.len() {
            print!("{}", CursorUp(1));
            print!("{}", EraseLine);
        }
        print_hidden(text);
        sleep(Duration::from_millis(40));
    }
}

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut text = parse_input(&buf);
    print_hidden(&text);
    decrypt(&mut text);
}
