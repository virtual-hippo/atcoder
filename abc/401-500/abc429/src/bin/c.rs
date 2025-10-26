use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut map = HashMap::new();
    for i in 0..n {
        *map.entry(a[i]).or_insert(0_u64) += 1;
    }

    let mut ans = 0;
    let n = n as u64;
    for (_, &v) in map.iter() {
        if v >= 2 {
            let a = (v * (v - 1)) / 2;
            ans += a * (n - v);
        }
    }

    println!("{}", ans);
}
