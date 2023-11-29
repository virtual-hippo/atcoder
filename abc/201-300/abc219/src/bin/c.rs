// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::BinaryHeap;
// use proconio::marker::Chars;
use proconio::marker::Chars;
use proconio::{fastout, input};
use std::cmp::Ordering;
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        x: Chars,
        n: usize,
        mut s: [Chars; n],
    }
    let mut map = HashMap::new();
    for (i, ch) in x.iter().enumerate() {
        map.insert(*ch, i);
    }
    s.sort_by(|a, b| {
        let len = a.len().min(b.len());
        let mut i = 0;
        while i < len && map[&a[i]].cmp(&map[&b[i]]) == Ordering::Equal {
            i += 1;
        }
        if len == i {
            a.len().cmp(&b.len())
        } else {
            map[&a[i]].cmp(&map[&b[i]])
        }
    });

    for i in 0..n {
        for j in 0..s[i].len() {
            print!("{}", s[i][j]);
        }
        println!();
    }
}
