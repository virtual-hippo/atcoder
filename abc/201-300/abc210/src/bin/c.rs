// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        c: [usize; n],
    }
    let mut ans = 0;
    let mut map = HashMap::new();
    for i in 0..k {
        *map.entry(c[i]).or_insert(0) += 1;
    }
    ans = ans.max(map.len());
    for i in k..n {
        if let Some(val) = map.get_mut(&c[i - k]) {
            *val -= 1;
            if *val == 0 {
                map.remove(&c[i - k]);
            }
        }
        *map.entry(c[i]).or_insert(0) += 1;
        ans = ans.max(map.len());
    }
    println!("{}", ans);
}
