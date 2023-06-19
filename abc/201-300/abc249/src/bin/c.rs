// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::input;
use proconio::marker::Chars;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        k: usize,
        s: [Chars; n],
    }
    let mut ans = 0;
    for bit in 0..(1<<n) {
        let mut map = HashMap::new();
        for i in 0..n {
            if bit & (1 << i) == 0 {
                for j in 0..s[i].len() {
                    *map.entry(s[i][j]).or_insert(0) += 1;
                }
            }
        }
        let current = map.iter().filter(|(_, v)| **v == k).count();
        ans = std::cmp::max(ans, current);
    }
    println!("{}", ans);
}

