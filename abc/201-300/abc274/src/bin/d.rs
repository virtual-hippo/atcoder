use std::collections::HashSet;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        x: i64,
        y: i64,
        a: [i64; n],
    }
    let mut dp1 = HashSet::new();
    let mut dp2 = HashSet::new();
    dp1.insert(0);
    dp2.insert(a[0]);
    for i in 1..n {
        if i % 2 == 1 {
            let mut new = HashSet::new();
            for v in dp1.iter() {
                new.insert(*v + a[i]);
                new.insert(*v - a[i]);
            }
            dp1 = new;
        } else {
            let mut new = HashSet::new();
            for v in dp2.iter() {
                new.insert(*v + a[i]);
                new.insert(*v - a[i]);
            }
            dp2 = new;
        }
    }
    if dp1.contains(&y) && dp2.contains(&x) {
        println!("Yes");
    } else {
        println!("No");
    }
}
