use proconio::{fastout, input, marker::Usize1};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        x: [Usize1; q],
    }

    let mut set = HashSet::new();
    let mut s = vec![0];
    let mut a = vec![0; n];

    let mut l_list = vec![usize::MAX; n];

    for i in 0..q {
        if set.contains(&x[i]) {
            set.remove(&x[i]);

            a[x[i]] += s[i] - s[l_list[x[i]]];
            l_list[x[i]] = usize::MAX;
        } else {
            set.insert(x[i]);
            l_list[x[i]] = i;
        }
        let ns = s[i] + set.len();
        s.push(ns);
    }

    for i in (0..n).filter(|&i| l_list[i] != usize::MAX) {
        a[i] += s[q] - s[l_list[i]];
    }

    for i in 0..n {
        print!("{} ", a[i]);
    }
}
