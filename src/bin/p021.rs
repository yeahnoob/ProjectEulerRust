//! [Problem 21](https://projecteuler.net/problem=21) solver.

#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

#![feature(core)]

#[macro_use(problem)] extern crate common;
extern crate prime;

use std::iter::AdditiveIterator;
use prime::{PrimeSet, Factorize};

fn compute(limit: u64) -> u64 {
    let ps = PrimeSet::new();

    let sum_of_div = (0 .. limit).map(|n| n.sum_of_proper_divisor(&ps)).collect::<Vec<_>>();

    sum_of_div
        .iter()
        .cloned()
        .enumerate()
        .map(|(n, div)| (n as u64, div))
        .filter(|&(n, div)| div < n)
        .filter(|&(n, div)| sum_of_div[div as usize] == n)
        .map(|(a, b)| a + b)
        .sum()
}

fn solve() -> String { compute(10000).to_string() }

problem!("31626", solve);
