use proconio::input;
use std::collections::HashMap;
use std::collections::BinaryHeap;


fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut map = HashMap::new();
    for _ in 0..n {
        input! {
            (a, b): (usize, usize),
        }
        map.entry(a).or_insert_with(|| vec![]).push(b);
    }
    let mut bh = BinaryHeap::new();
    let mut ans = 0;
    for i in 1..=m {
        if let Some(vec) = map.get(&i) {
            for &v in vec.iter() {
                bh.push(v);
            }
        }
        if let Some(val) = bh.pop() {
            ans += val;
        }
    }
    println!("{}", ans);
}
