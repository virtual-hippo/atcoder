// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        n: u64,
    }
    let a: u64 = {
        let mut ret = 1;
        for _ in 0..n {
            ret *= 10;
            ret %= 1_000_000_007;
        }
        ret
    };
    let b = {
        let mut ret = 1;
        for _ in 0..n {
            ret *= 9;
            ret %= 1_000_000_007;
        }
        ret
    };
    let c = {
        let mut ret = 1;
        for _ in 0..n {
            ret *= 8;
            ret %= 1_000_000_007;
        }
        ret
    };
    let ans = (a + 1_000_000_007 + 1_000_000_007 - (b+b-c)) % 1_000_000_007;

    println!("{}", ans);
}
