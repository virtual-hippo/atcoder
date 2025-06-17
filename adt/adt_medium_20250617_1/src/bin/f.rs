use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: usize,
        ac: [(usize,usize); n],
    }

    let mut map = HashMap::new();

    for i in 0..n {
        let (a, c) = ac[i];

        if let Some(v) = map.get_mut(&c) {
            *v = a.min(*v);
        } else {
            map.insert(c, a);
        }
    }

    let ans = map.values().max().unwrap();
    println!("{}", ans);
}
