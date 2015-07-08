//! [Problem 2](https://projecteuler.net/problem=2) solver.

#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results)]

#![feature(iter_arith)]

extern crate num;
#[macro_use(problem)] extern crate common;
extern crate seq;

use num::Integer;
use seq::Fibonacci;

fn compute(bound: u32) -> u32 {
    Fibonacci::<u32>::new()
        .take_while(|&f| f < bound)
        .filter(|&f| f.is_even())
        .sum()
}

fn solve() -> String { compute(4000000).to_string() }
problem!("4613732", solve);

#[cfg(test)]
mod tests {
    #[test]
    fn sum_below_90() { assert_eq!(44, super::compute(90)); }
}
