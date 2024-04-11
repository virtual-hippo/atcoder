use proconio::{fastout, input};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
        m: usize,
        b: [usize; m],
        l: usize,
        c: [usize; l],
        q: usize,
        x: [usize; q],
    }
    let mut set = HashSet::new();
    for i in 0..n {
        for j in 0..m {
            for k in 0..l {
                set.insert(a[i] + b[j] + c[k]);
            }
        }
    }
    for i in 0..q {
        if set.contains(&x[i]) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
