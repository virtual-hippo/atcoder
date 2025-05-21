use proconio::{fastout, input};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        b: [usize; m],
    }

    let c = {
        let mut c = vec![0; n + m];
        for i in 0..n {
            c[i] = a[i];
        }
        for i in 0..m {
            c[n + i] = b[i];
        }
        c.sort();
        c
    };

    let aa = a.iter().map(|&v| v).collect::<HashSet<_>>();

    for i in 0..n + m - 1 {
        if aa.contains(&c[i]) && aa.contains(&c[i + 1]) {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
