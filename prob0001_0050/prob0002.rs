use core::iterator::{ IteratorUtil };

use common::extiter::{ Fibonacci, sum_uint };
use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 2,
    answer: "4613732",
    solver: solve
};

fn solve() -> ~str {
    let max = 4000000;
    let it = Fibonacci::new::<uint>()
        .take_while(|&f| f < max)
        .filter(|&f| f % 2 == 0);
    return sum_uint(it).to_str();
}
