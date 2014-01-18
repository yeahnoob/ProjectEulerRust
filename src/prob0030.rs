#[crate_type = "rlib"];

extern mod common;
extern mod math;

use std::{num, vec};
use std::iter::AdditiveIterator;
use common::calc;
use math::numconv;

pub static EXPECTED_ANSWER: &'static str = "443839";

// 9^5     = 59049
// 9999    => 9^5 * 4 = 236196
// 99999   => 9^5 * 5 = 295245
// 999999  => 9^5 * 6 = 354294
// 9999999 => 9^5 * 7 = 413343

// 1-6 digits numbers meet conditions
pub fn solve() -> ~str {
    let len = 7;
    let pows = vec::from_fn(10, |i| num::pow(i, 5));

    let mut sum = 0;
    calc::combinate_overlap([0u, 1, 2, 3, 4, 5, 6, 7, 8, 9], len, |comb| {
            let num = comb.iter().map(|&e| pows[e]).sum();

            let mut ds = numconv::to_digits(num, 10).to_owned_vec();
            ds.sort_by(|a, b| a.cmp(b));

            let zero_len = len - ds.len();
            if comb.tailn(zero_len) == ds &&
                comb.iter().take(zero_len).all(|&x| x == 0) {
                sum += num;
            }
            true
        });

    return (sum - 1).to_str();  // remove 1
}
