use std::collections::HashSet;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut vec = Vec::with_capacity(n);
    let mut set = HashSet::with_capacity(n);
    for _ in 0..n {
        input! {
            x: usize,
            y: usize,
        }
        vec.push((x, y));
        set.insert((x, y));
    }
    vec.sort();
    let mut ans = 0;

    for i in 0..n - 1 {
        for j in i + 1..n {
            if vec[i].0 < vec[j].0 && vec[i].1 < vec[j].1 {
                if set.contains(&((vec[i].0, vec[j].1))) && set.contains(&((vec[j].0, vec[i].1))) {
                    ans += 1;
                }
            }
        }
    }
    println!("{}", ans);
}
