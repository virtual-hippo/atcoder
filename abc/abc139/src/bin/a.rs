// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }
    let mut ans = 0;
    for i in 0..3 {
        if s[i] == t [i] {
            ans +=1 ;
        }
    }
    println!("{}", ans);
}

