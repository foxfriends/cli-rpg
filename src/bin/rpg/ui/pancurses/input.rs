use pancurses::Window;

#[derive(Copy, Clone, Debug)]
pub enum InputKind {
    Character(char),
    F(u8),
    Esc,
    Up,
    Left,
    Right,
    Down,
    Insert,
    Delete,
    Home,
    End,
    PageUp,
    PageDown,
    Unknown,
    ShiftTab,
}

#[derive(Debug)]
pub struct Input {
    raw: Vec<char>,
    pub kind: InputKind,
    pub alt: bool,
}

impl Input {
    pub fn read(window: &Window) -> Vec<Self> {
        let mut raw = vec![];
        while let Some(pancurses::Input::Character(ch)) = window.getch() {
            raw.push(ch);
        }
        let mut inputs = vec![];
        Self::parse(raw.as_slice(), &mut inputs);
        inputs
    }

    fn parse(mut raw: &[char], inputs: &mut Vec<Self>) {
        use InputKind::*;
        while !raw.is_empty() {
            let mut alt = false;
            let mut len = 1;
            let kind = match raw {
                ['\u{1b}', '[', '1', ';', '3', c, ..] => {
                    len = 6;
                    alt = true;
                    match c {
                        'A' => Up,
                        'B' => Down,
                        'C' => Right,
                        'D' => Left,
                        'F' => End,
                        'H' => Home,
                        _ => Unknown,
                    }
                }
                ['\u{1b}', '[', '1', c, '~', ..] => {
                    len = 5;
                    match c {
                        '5' => F(5),
                        '7' => F(6),
                        '8' => F(7),
                        '9' => F(8),
                        _ => Unknown,
                    }
                }
                ['\u{1b}', '[', '2', c, '~', ..] => {
                    len = 5;
                    match c {
                        '0' => F(9),
                        '1' => F(10),
                        '3' => F(11),
                        '4' => F(12),
                        _ => Unknown,
                    }
                }
                ['\u{1b}', '[', c, ';', '3', '~', ..] => {
                    len = 6;
                    match c {
                        '3' => Delete,
                        '4' => End,
                        '5' => PageUp,
                        '6' => PageDown,
                        _ => Unknown,
                    }
                }
                ['\u{1b}', 'O', c, ..] => {
                    len = 3;
                    match c {
                        'P' => F(1),
                        'Q' => F(2),
                        'R' => F(3),
                        'S' => F(4),
                        _ => Unknown,
                    }
                }
                ['\u{1b}', '[', c, '~', ..] => {
                    len = 4;
                    match c {
                        '1' => Home,
                        '2' => Insert,
                        '3' => Delete,
                        '4' => End,
                        '5' => PageUp,
                        '6' => PageDown,
                        _ => Unknown,
                    }
                }
                ['\u{1b}', '[', c, ..] => {
                    len = 3;
                    match c {
                        'A' => Up,
                        'B' => Down,
                        'C' => Right,
                        'D' => Left,
                        'Z' => ShiftTab,
                        _ => Unknown,
                    }
                }
                ['\u{1b}', '\u{1b}', ..] => Esc,
                ['\u{1b}', ch, ..] => {
                    len = 2;
                    alt = true;
                    InputKind::Character(*ch)
                }
                ['\u{1b}'] => InputKind::Esc,
                [ch, ..] => InputKind::Character(*ch),
                [] => unreachable!(),
            };

            inputs.push(Self {
                raw: raw[..len].to_vec(),
                kind,
                alt,
            });
            raw = &raw[len..];
        }
    }
}
