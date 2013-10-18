#[link(name = "prob0042", vers = "0.0")];
#[crate_type = "lib"];

extern mod common;
extern mod math;

use std::{io, result, vec};
use common::reader;
use math::sequence;

pub static EXPECTED_ANSWER: &'static str = "162";

fn word_value(word: &str) -> uint {
    let mut value = 0;
    for b in word.byte_iter() {
        value += (b - ('A' as u8) + 1) as uint;
    }
    return value;
}

pub fn solve() -> ~str {
    let result = io::read_whole_file_str(&Path::new("files/words.txt")).and_then(|input| {
        do reader::read_whole_word(input).map |words| { words.map(|w| word_value(*w)) }
    }).map(|values| {
        let mut is_tri = vec::from_elem(values.iter().max().unwrap() + 1, false);
        let mut it = sequence::triangle::<uint>().take_while(|&t| t < is_tri.len());
        for t in it { is_tri[t] = true; }

        values.iter().count(|&v| is_tri[v])
    });
    match result {
        result::Err(msg) => { fail!(msg) }
        result::Ok(cnt) => { return cnt.to_str(); }
    }
}
