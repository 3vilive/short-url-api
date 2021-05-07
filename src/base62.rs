use lazy_static::lazy_static;
use std::collections::HashMap;
use std::ops::{Div, Rem};
use std::usize;

static BASE62_ALPHABET: [char; 62] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I',
    'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b',
    'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u',
    'v', 'w', 'x', 'y', 'z',
];

lazy_static! {
    static ref BASE62_ALPHABET_INDEX: HashMap<char, usize> = {
        let mut m = HashMap::new();
        for (i, &char) in BASE62_ALPHABET.iter().enumerate() {
            m.insert(char, i);
        }
        m
    };
}

#[derive(Debug)]
pub struct DecodeError {
    pub err: String,
}

impl DecodeError {
    fn new(err: &str) -> DecodeError {
        DecodeError {
            err: err.to_owned(),
        }
    }
}

fn div_mod<T>(i: T, j: T) -> (T, T)
where
    T: Div<Output = T> + Rem<Output = T> + Copy,
{
    (i / j, i % j)
}

pub fn encode(i: u64) -> String {
    if i == 0 {
        return BASE62_ALPHABET[0].into();
    }

    let mut chars: Vec<char> = Vec::new();
    let mut iter = i;
    while iter > 0 {
        let (next_iter, remainder) = div_mod(iter, BASE62_ALPHABET.len() as u64);
        iter = next_iter;
        chars.push(BASE62_ALPHABET[remainder as usize]);
    }

    chars.reverse();
    chars.into_iter().collect()
}

pub fn decode(s: &str) -> Result<u64, DecodeError> {
    if s.len() == 0 {
        Err(DecodeError::new("invalid input"))?
    }

    let mut decoded: u64 = 0;
    let rs: Vec<char> = s.chars().collect();

    for (i, c) in rs.iter().rev().enumerate() {
        if let Some(&index) = BASE62_ALPHABET_INDEX.get(c) {
            decoded += (index as u64) * 62_u64.pow(i as u32);
        } else {
            return Err(DecodeError::new("unexpected char"));
        }
    }

    Ok(decoded)
}
