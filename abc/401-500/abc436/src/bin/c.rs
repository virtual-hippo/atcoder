use proconio::{fastout, input};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        _: usize,
        m: usize,
    }

    let mut set = HashSet::new();
    let mut ans = 0;

    for _ in 0..m {
        input! {
            r: usize,
            c: usize,
        }

        let vals = [(r, c), (r + 1, c), (r, c + 1), (r + 1, c + 1)];

        if vals.iter().all(|v| !set.contains(v)) {
            for v in vals {
                set.insert(v);
            }
            ans += 1;
        }
    }

    println!("{}", ans);
}
