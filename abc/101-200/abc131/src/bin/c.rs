// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn gcd(x: usize, y: usize) -> usize {
    let mut xy = (x, y);
    while xy.0 >= 1 && xy.1 >= 1 {
        if xy.0 < xy.1 {
            xy.1 %= xy.0;
        } else {
            xy.0 %= xy.1;
        }
    }
    if xy.0 >= 1 {
        xy.0
    } else {
        xy.1
    }
}

fn lcm(x: usize, y: usize) -> usize {
    let d = gcd(x,y);
    x / d * y
}

fn warikirerukazu(x: usize, div1: usize, div2: usize) -> usize {
    x- (x/div1 + x/div2 - x/lcm(div1,div2))
}

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    }
    println!("{}", warikirerukazu(b, c, d) - warikirerukazu(a-1, c, d));
}

