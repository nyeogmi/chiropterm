use std::collections::HashMap;

const UTF8_DATA: &[u8] = include_bytes!("data.bin");

lazy_static! {
    static ref TO_CP437: HashMap<char, u8> = {
        let mut m = HashMap::new();
        for (i, c) in std::str::from_utf8(UTF8_DATA).unwrap().chars().enumerate() {
            m.insert(c, i as u8);
        }
        assert_eq!(m.len(), 256);
        m
    };
}

pub fn encode_char(c: char) -> u8 {
    if let Some(x) = TO_CP437.get(&c) { return *x; }
    b'?'
}

pub fn encode_str(s: &str) -> impl '_+DoubleEndedIterator<Item=u8> {
    s.chars().map(encode_char)
}