use proconio::{fastout, input};
use std::collections::BTreeSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        mut p: [usize; n],
    }
    if k == 1 {
        println!("0");
        return;
    }

    let mut indexes = vec![0; n];

    for i in 0..n {
        p[i] -= 1;
        indexes[p[i]] = i;
    }

    let mut ans = usize::MAX;
    let mut set = BTreeSet::new();
    for i in 0..n {
        set.insert(indexes[i]);
        if set.len() < k {
            continue;
        }
        ans = ans.min(set.last().unwrap() - set.first().unwrap());
        set.remove(&indexes[i + 1 - k]);
    }
    println!("{}", ans);
}
