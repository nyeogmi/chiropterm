use std::convert::TryInto;

#[derive(Clone, Copy)]
pub struct Swatch {
    pub colors: [u32; 0x100],
    pub default_bg: u8,
    pub default_fg: u8,
}

impl Swatch {
    pub fn get(&self, color: u8) -> u32 {
        self.colors[color as usize]
    }
}

const SWATCH_DATA: &[u8; 0x300] = include_bytes!("swatch.bin");

lazy_static! {
    pub static ref DEFAULT_SWATCH: Swatch = {
        let mut full_colors = vec![];
        for i in 0..256 {
            full_colors.push(
                SWATCH_DATA[i * 3] as u32 * 0x10000 | 
                SWATCH_DATA[i * 3 + 1] as u32 * 0x100 | 
                SWATCH_DATA[i * 3 + 2] as u32 
            );
        }
        Swatch {
            colors: (&full_colors[..]).try_into().unwrap(),
            default_bg: 11,
            default_fg: 0,
        }
    };
}