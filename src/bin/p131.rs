//! [Problem 131](https://projecteuler.net/problem=131) solver.
//! # 解析
//!
//! ```math
//! n^3 + n^2p = m^3
//! ```
//!
//! とおく。
//!
//! ## 定理 1
//! `n` と `p` は互いに素である。
//!
//! ## 証明
//!
//! `p` は素数なので、`n` と `p` が互いに素でない場合、
//! ある自然数 `k` を使って `n = kp` と書ける。
//! このとき、
//!
//! ```math
//! n^3 + n^2p = p^3k^2(k + 1) = m^3
//! k^3 + k^2 = (m / p)^3
//! ```
//!
//! となる。`k^3` の次に大きい立方数は `(k+1)^3` なので、
//! `k^3 + k^2` は立方数ではなく、矛盾する。
//! よって、`n` と `p` は互いに素である■
//!
//! ## 定理 2
//!
//! `n` は立方数である。また、`p` は立方数の差として表される。
//!
//! ## 証明
//!
//! `n` は、互いに素である因数 `s0`, `s1`, `s2` を用いて、以下のように書ける。
//!
//! ```math
//! n = s0^(3e0) * s1^(3e1+1) * s2^(3e2+2)
//! ```
//!
//! このとき、`n` と `p` は互いに素であるため、
//! `n+p` は以下のように因数分解できなければならない。
//!
//! ```math
//! n + p = s0^(3e0) * s1^(3e1+1) * s2^(3e2+2) + p
//!       = s1^(3e'1+1) * s2^(3e'2+2) * k^3
//! ```
//!
//! 上式を整理して、以下を得る。
//!
//! ```math
//! p = s1^(3e''1+1) * s2^(3e''2+2) * (k^3 - p^3)
//! ```
//!
//! 右辺は合成数ではないため、`s1^(3e''1+1) * s2(3e''2+2) = 1` である。
//! すなわち、`n = s0^(3e0)` と書け、立方数である■
//!
//! ## 定理3
//!
//! `p` は任意の数 `q` を用いて以下のように表される。
//!
//! `p = 3q^2 + 3q + 1`
//!
//! ## 証明
//!
//! 定理2 より、`p` は立方根の差として表される素数である。
//! `p = r^3 - q^3` と置くと、以下を得る。
//!
//! ```math
//! p = (r-q)(r^2+rq+q^2)
//! ```
//!
//! `r^2 + rq + q^2 > 1` より、 `r - q = 1` である。
//! すなわち、
//!
//! ```math
//! p = (q+1)^2 + q(q+1) + q^2
//!   = 3q^2 + 3q + 1
//! ```
//!
//! である ■
//!
//! # 解法
//!
//! `3q^2 + 3q + 1` を `q` について計算し、素数のものを列挙する。

#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results)]

#[macro_use(problem)]
extern crate common;
extern crate prime;

use prime::PrimeSet;

fn compute(limit: u64) -> usize {
    let ps = PrimeSet::new();

    (1..)
        .map(|q| 3 * q * q + 3 * q + 1)
        .take_while(|&p| p <= limit)
        .filter(|&p| ps.contains(p))
        .count()
}

fn solve() -> String {
    compute(1000000).to_string()
}

problem!("173", solve);

#[cfg(test)]
mod tests {
    #[test]
    fn hundred() {
        assert_eq!(4, super::compute(100));
    }
}
