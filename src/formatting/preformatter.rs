use euclid::point2;

use crate::{aliases::CellPoint, cp437, rendering::Font, drawing::Brush};

use super::{FChar, FString, fstring::FBevels};

pub struct Preformatter {
    pub font: Font,
    pub indent: isize,
    pub first_line_fractional: isize,  // fractional offset for first line, used for big fonts
    pub width: Option<usize>,
    pub justification: Justification,
}

impl Preformatter {
    fn desired_width(&self, y: usize) -> Option<usize> {
        let indent = self.indent;
        self.width.map(|mut w| {
            if y == 0 && self.first_line_fractional > 0 { w -= 1 }
            if indent < 0 {
                if y == 0 { w } else { w - ((-indent) as usize) }
            } else {
                if y == 0 { w - (indent as usize) } else { w }
            }
        })
    }
    fn calculate_xoffset(&self, line_width: usize, y: usize) -> usize {
        let indent = self.indent;
        let dx = if indent < 0 {
            if y == 0 { 0 } else { (-indent) as usize }
        } else {
            if y == 0 { indent as usize } else { 0 }
        };
        self.width.map(|mut w| {
            if y == 0 && self.first_line_fractional > 0 { w -= 1 }
            w = w - dx;
            dx + match self.justification {
                Justification::Left => { 0 },
                Justification::Right => w - line_width,
                Justification::Center => line_width / 2,
            }
        }).unwrap_or(0)
    }

    pub fn draw(&self, s: &str, brush: &mut Brush) {
        let fs = self.to_fstring(s);
        let words = self.break_words(&fs);
        let lines = self.break_lines(&words);
        self.onto_brush_internal(&lines, &words, &fs, brush)
    }
    
    fn onto_brush_internal(
        &self,
        lines: &Vec<FLine>,
        words: &Vec<FWord>,
        characters: &FString,
        brush: &mut Brush,
    ) {
        let char_size = self.font.char_size();

        let mut cursor_dx = 0; // presumably handled with indent
        let cursor_dy = brush.cursor.y as usize;

        let mut x: usize = 0;
        let mut y: usize = 0;

        let mut forced_break: bool = false;
        while y < lines.len() {
            if y == 0 { 
                cursor_dx = 0; // no reason for future lines to be indented to match the cursor
            }  
            let line = &lines[y];
            x = self.calculate_xoffset(line.width, y);

            for w in line.lhs..line.rhs {
                let word = words[w];
                // NYEO NOTE: For the last line of the input, get _all_ the extra chars
                // Otherwise, drop the extra spaces
                let rhs = if w == line.rhs - 1 && w != words.len() - 1 {  
                    word.whitespace_lhs
                } else {
                    word.word_rhs
                };

                for c in word.lhs..rhs {
                    // draw on stamp
                    // TODO: Depends on the font
                    let mut cell_x = x as isize * char_size.width + cursor_dx as isize;
                    let cell_y = y as isize * char_size.height + cursor_dy as isize;

                    if y == 0 {
                        cell_x += self.first_line_fractional
                    }

                    self.font.draw_char(point2(cell_x, cell_y), characters.0[c], brush);
                    x += 1;
                }
            }

            forced_break = line.forced_break;
            y += 1; 
        }

        *brush = brush.at(if forced_break {
            CellPoint::new(cursor_dx as isize, (y as isize * char_size.height) + cursor_dy as isize)
        } else {
            CellPoint::new(
                x as isize * char_size.width + cursor_dx as isize, 
                ((y - 1) as isize * char_size.height) + cursor_dy as isize
            )
        });
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

                if let Some(w) = self.desired_width(y) {
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
            if let Some(w) = self.width {
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
                fs.push(FChar { bg, fg, sprite: Some(cp437::encode_char(c) as u16), interactor: None, scroll_interactor: None, bevels: FBevels::new() });
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

#[derive(Clone, Copy, Debug)]
struct FLine {
    lhs: usize,
    rhs: usize,
    width: usize,
    forced_break: bool,
}

#[derive(Clone, Copy, Debug)]
pub enum Justification {
    Left, 
    Center, 
    Right,
    // TODO: Justify?
}