#[link(name = "prob0037", vers = "0.0")];
#[crate_type = "lib"];

extern mod math;

use math::{numconv, prime};

pub static EXPECTED_ANSWER: &'static str = "748317";

fn is_r2l(n: uint) -> bool {
    let mut itr = n / 10;
    while itr > 0 {
        if !prime::contains(itr) { return false }
        itr /= 10;
    }
    true
}

pub fn solve() -> ~str {
    let mut l2r_mat = ~[ ~[ 2u, 3, 5, 7 ] ];
    let mut order = 10;

    loop {
        let mut result = ~[];
        for &p in l2r_mat.last().iter() {
            // 2 can only be appeared as the most left digits
            if numconv::to_digits(p, 10).next_back() == Some(2) { continue }

            let ds = [ 1u, 2, 3, 5, 7, 9 ];
            for &d in ds.iter() {
                let n = order * d + p;
                if prime::contains(n) { result.push(n); }
            }
        }

        if result.is_empty() { break }
        l2r_mat.push(result);
        order *= 10;
    }

    let l2r = l2r_mat.concat_vec();
    let mut sum = 0;
    for n in  l2r.iter() {
        if *n < 10 { continue }
        if is_r2l(*n) { sum += *n; }
    }

    sum.to_str()
}
