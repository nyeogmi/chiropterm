use std::collections::{BTreeMap};

const UTF8_DATA: &[u8] = include_bytes!("data.bin");

lazy_static! {
    static ref TO_CP437: BTreeMap<char, u8> = {
        let mut m = BTreeMap::new();
        for (i, c) in std::str::from_utf8(UTF8_DATA).unwrap().chars().enumerate() {
            let result = m.insert(c, i as u8);
            assert!(result.is_none());
        }
        assert_eq!(m.len(), 256);
        m
    };

    static ref FROM_CP437: Vec<char> = {
        let m: Vec<char> = std::str::from_utf8(UTF8_DATA).unwrap().chars().collect();
        assert_eq!(m.len(), 256);
        m
    };
}

pub fn encode_char(c: char) -> u8 {
    if let Some(x) = TO_CP437.get(&c) { 
        return *x; 
    }
    b'?'
}

pub fn decode_char(u: u8) -> char {
    FROM_CP437[u as usize]
}

pub fn encode_str(s: &str) -> impl '_+DoubleEndedIterator<Item=u8> {
    s.chars().map(encode_char)
}