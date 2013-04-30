use core::iterator::{ Iterator, IteratorUtil };
use core::num::{ Zero, One };

pub struct UintRange {
    cur: uint,
    cnt: uint,
    step: int,
}

impl UintRange {
    pub fn new(start: uint, stop: uint, step: int) -> UintRange {
        if step == 0 {
            fail!("UintRange::new called with step == 0");
        }

        let mut cnt = 0;
        if step > 0 && start < stop {
            let diff = (stop - start);
            cnt = diff / (step as uint);
            if diff % (step as uint) != 0 { cnt += 1; }
        }
        if step < 0 && start > stop {
            let diff = (start - stop);
            cnt = diff / ((-step) as uint);
            if diff % ((-step) as uint) != 0 { cnt += 1; }
        }
        UintRange { cur: start, cnt: cnt, step: step }
    }
}

pub fn uint_range(start: uint, stop: uint) -> UintRange {
    UintRange::new(start, stop, 1)
}

impl Iterator<uint> for UintRange {
    fn next(&mut self) -> Option<uint> {
        if self.cnt == 0 { return None; }

        let val = self.cur;

        match self.step.cmp(&0) {
            Greater => {
                self.cnt -= 1;
                self.cur += (self.step as uint);
                return Some(val);
            },
            Less => {
                self.cnt -= 1;
                self.cur -= ((- self.step) as uint);
                return Some(val);
            },
            Equal => { fail!() }
        }
    }
}

pub struct Fibonacci<T> {
    prev: T,
    cur: T
}

impl<T: Zero + One> Fibonacci<T> {
    pub fn new() -> Fibonacci<T> { Fibonacci { prev: Zero::zero(), cur: One::one() } }
}

// Copy must be Clone
impl<T: Add<T,T> + Copy> Iterator<T> for Fibonacci<T> {
    fn next(&mut self) -> Option<T> {
        let next = self.prev + self.cur;
        // let cur  = self.cur.clone();
        let cur  = self.cur;
        // self.prev = cur.clone();
        self.prev = cur;
        self.cur  = next;
        // return Some(cur);
        return Some(cur);
    }
}

// This cannot use because of ICE.
// pub fn sum<T: Add<T, T> + Zero, IT: Iterator<T>>(mut it: IT) -> T {
//     let mut sum = Zero::zero::<T>();
//     for it.advance |n| { sum += n; }
//     return sum;
// }
pub fn sum_uint<IT: Iterator<uint>>(mut it: IT) -> uint {
    let mut sum = 0;
    for it.advance |n| { sum += n; }
    return sum;
}

pub fn count_elem<T, IT: Iterator<T>>(mut it: IT) -> uint {
    let mut cnt = 0;
    for it.advance |_| { cnt += 1; }
    return cnt;
}


#[cfg(test)]
mod tests {
    use super::*;
    use common::extvec;

    #[test]
    fn test_uint_range() {
        fn gen(start: uint, end: uint, step: int) -> ~[uint] {
            extvec::from_iter(UintRange::new(start, end, step))
        }
        assert_eq!(gen(0, 3, 1), ~[0, 1, 2]);
        assert_eq!(gen(13, 10, -1), ~[13, 12, 11]);
        assert_eq!(gen(20, 26, 2), ~[20, 22, 24]);
        assert_eq!(gen(36, 30, -2), ~[36, 34, 32]);
        assert_eq!(gen(uint::max_value - 2, uint::max_value, 2),
                   ~[uint::max_value - 2]);
        assert_eq!(gen(uint::max_value - 3, uint::max_value, 2),
                   ~[uint::max_value - 3, uint::max_value - 1]);
        assert_eq!(gen(uint::min_value + 2, uint::min_value, -2),
                   ~[uint::min_value + 2]);
        assert_eq!(gen(uint::min_value + 3, uint::min_value, -2),
                   ~[uint::min_value + 3, uint::min_value + 1]);
    }

    #[test]
    fn test_fibonacci() {
        let it = Fibonacci::new::<uint>();
        let fib = ~[ 1u, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233 ];
        assert_eq!(extvec::from_iter(it.take(fib.len())), fib);
    }

    #[test]
    fn test_sum_uint() {
        assert_eq!(sum_uint(uint_range(0, 4)), 6);
        assert_eq!(sum_uint(uint_range(0, 10)), 45);
        assert_eq!(sum_uint(uint_range(10, 0)), 0);
    }
}