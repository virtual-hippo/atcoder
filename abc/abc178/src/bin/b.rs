// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
        d: i64,
    }
    let ans ={
        let aa = std::cmp::max(a*c,a*d);
        let bb = std::cmp::max(b*c,b*d);
        std::cmp::max(aa,bb)
    };

    println!("{}", ans);
}

