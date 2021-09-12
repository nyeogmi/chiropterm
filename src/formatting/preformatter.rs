use euclid::point2;

use crate::{aliases::{CellPoint}, cp437, drawing::Stamp, rendering::Font};

use super::{FChar, FString};

pub struct Preformatter {
    pub font: Font,
    pub first_width_chars: Option<usize>,
    pub main_width_chars: Option<usize>,
    pub justification: Justification,
}

impl Preformatter {
    fn width(&self, y: usize) -> Option<usize> {
        if y == 0 { self.first_width_chars }
        else { self.main_width_chars }
    }

    fn leading(&self, y: usize) -> usize {
        match (self.main_width_chars, self.first_width_chars) {
            (Some(main_width), Some(first_width)) => {
                if y == 0 {
                    return 0
                }

                return main_width - first_width
            },
            (_, _) => 0
        }
    }

    pub fn to_stamp(&self, s: &str) -> Stamp {
        self.to_stamp_fstring(&self.to_fstring(s))
    }

    pub fn to_stamp_fstring(&self, fs: &FString) -> Stamp {
        let words = self.break_words(&fs);
        let lines = self.break_lines(&words);
        let stamp = self.to_stamp_internal(&lines, &words, fs);
        stamp
    }
    
    fn to_stamp_internal(
        &self,
        lines: &Vec<FLine>,
        words: &Vec<FWord>,
        characters: &FString,
    ) -> Stamp {
        let mut stamp = Stamp::new();

        let char_size = self.font.char_size();

        let mut x: usize = 0;
        let mut y: usize = 0;

        let mut forced_break: bool = false;
        while y < lines.len() {
            let line = &lines[y];
            x = 
                self.leading(y) +
                if let Some(w) = self.width(y) {
                    match self.justification {
                        Justification::Left => 0,
                        Justification::Right => w - line.width,
                        _ => (w - line.width) / 2
                    }
                } else { 0 };

            for w in line.lhs..line.rhs {
                let word = words[w];
                let rhs = if w == line.rhs - 1 {
                    word.whitespace_lhs
                } else {
                    word.word_rhs
                };

                for c in word.lhs..rhs {
                    // draw on stamp
                    // TODO: Depends on the font
                    let cell_x = x as isize * char_size.width;
                    let cell_y = y as isize * char_size.height;

                    self.font.draw_char(point2(cell_x, cell_y), characters.0[c], &mut stamp);
                    x += 1;
                }
            }

            forced_break = line.forced_break;
            y += 1; 
        }

        stamp.cursor_point = Some(if forced_break {
            CellPoint::new(0, y as isize * char_size.height)
        } else {
            CellPoint::new(x as isize * char_size.width, (y - 1) as isize * char_size.height)
        });

        stamp
    }

    fn break_lines(&self, words: &[FWord]) -> Vec<FLine> {
        let mut lines: Vec<FLine> = Vec::new();
        let mut i = 0;

        for y in 0.. {
            let mut line = FLine { lhs: i, rhs: i, width: 0, forced_break: false};
            let mut additional = 0;

            while i < words.len() {
                let word_length = words[i].whitespace_lhs - words[i].lhs;
                let additional_length = words[i].word_rhs - words[i].whitespace_lhs;

                if let Some(w) = self.width(y) {
                    if line.width + additional + word_length > w {
                        break;
                    }
                }

                line.rhs += 1;
                line.width += additional + word_length;
                additional = additional_length;

                let force_break = words[i].force_break;
                i += 1;

                if force_break {
                    line.forced_break = true;
                    break;
                }
            }

            lines.push(line);

            if i >= words.len() {
                break;
            }
        }

        lines
    }

    fn break_words(&self, fs: &FString) -> Vec<FWord> {
        let mut i = 0;
        let mut f_words_1: Vec<FWord> = Vec::new();

        // == corresponds to BreakWords2 ==
        loop {
            let mut word = FWord {
                lhs: i,
                whitespace_lhs: i,
                word_rhs: i,
                force_break: false,
            };

            // no characters yet
            // slurp whitespace at the beginning of the word
            while i < fs.0.len() && fs.0[i].sprite == Some(b' ' as u16) {
                word.whitespace_lhs += 1;
                word.word_rhs += 1;
                i += 1;
            }

            if i == fs.0.len() { f_words_1.push(word); break; }
            if fs.0[i].sprite == Some(b'\n' as u16) {
                word.force_break = true;
                f_words_1.push(word);
                i += 1;
                continue;
            }

            // slurp non-whitespace at the middle of a word
            while i < fs.0.len() && fs.0[i].sprite != Some(b' ' as u16) && fs.0[i].sprite != Some(b'\n' as u16) {
                word.whitespace_lhs += 1;
                word.word_rhs += 1;
                i += 1;
            }

            if i == fs.0.len() { f_words_1.push(word); break; }
            if fs.0[i].sprite == Some(b'\n' as u16) {
                word.force_break = true;
                f_words_1.push(word);
                i += 1;
                continue;
            }

            // slurp whitespace at the end of a word
            while i < fs.0.len() && fs.0[i].sprite == Some(b' ' as u16) {
                word.word_rhs += 1;
                i += 1;
            }

            if i == fs.0.len() { f_words_1.push(word); break; }
            if fs.0[i].sprite == Some(b'\n' as u16) {
                word.force_break = true;
                f_words_1.push(word);
                i += 1;
                continue;
            }
            f_words_1.push(word);
        }

        // corresponds to BreakWords1
        let mut f_words_2 = vec![];
        for mut word in f_words_1.into_iter() {
            if let Some(w) = self.main_width_chars {
                while word.whitespace_lhs - word.lhs > w {
                    f_words_2.push(FWord {
                        lhs: word.lhs,
                        whitespace_lhs: word.lhs + w,
                        word_rhs: word.lhs + w,
                        force_break: false,
                    });
                    word.lhs += w
                }
            }

            f_words_2.push(word)
        }

        // corresponds to last-ditch filtering in BreakWords
        let mut f_words_3 = vec![];
        for word in f_words_2.into_iter() {
            if word.word_rhs != word.lhs || word.force_break {
                f_words_3.push(word);
            }
        }

        f_words_3
    }

    pub fn to_fstring(&self, s: &str) -> FString {
        let mut bg: Option<u8> = None;
        let mut fg: Option<u8> = None;

        let mut fs: Vec<FChar> = Vec::new();
        let mut iter = s.chars();
        loop {
            let c = match iter.next() {
                Some(c) => c,
                None => { return FString(fs); }
            };

            if c == '\u{ffff}' {
                let code = match iter.next() {
                    Some(c) => c,
                    None => { return FString(fs); }
                };
                let arg = match iter.next() {
                    Some(c) => c,
                    None => { return FString(fs); }
                };

                let arg_value = arg as u32;
                let arg_value = if arg_value > u8::MAX as u32 {
                    None
                } else { Some(arg_value as u8) };

                if code == '\x00' {
                    // change bg
                    bg = arg_value
                } 
                else if code == '\x01' {
                    fg = arg_value
                }
            } else {
                fs.push(FChar { bg, fg, sprite: Some(cp437::encode_char(c) as u16) });
            }
        }
    }
}

#[derive(Clone, Copy)]
struct FWord {
    lhs: usize,
    whitespace_lhs: usize,
    word_rhs: usize,
    force_break: bool,
}

#[derive(Clone, Copy)]
struct FLine {
    lhs: usize,
    rhs: usize,
    width: usize,
    forced_break: bool,
}

#[derive(Clone, Copy)]
pub enum Justification {
    Left, Center, Right,
    // TODO: Justify?
}