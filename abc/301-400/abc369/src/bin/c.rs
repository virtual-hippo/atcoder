use proconio::{fastout, input};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    if n == 1 {
        println!("1");
        return;
    }
    let mut b = vec![0; n];
    for i in 1..n {
        b[i] = a[i] - a[i - 1];
    }
    let mut set = HashSet::new();
    let mut l = 1;
    for r in 1..n {
        if b[l] != b[r] {
            set.insert((l - 1, r - 1));
            l = r;
        }
    }
    if b[l] == b[n - 1] {
        set.insert((l - 1, n - 1));
    }

    let mut ans = n;
    for (l, r) in set.iter() {
        let v = *r - *l + 1;
        ans += (v * (v - 1)) / 2;
    }
    println!("{}", ans);
}
