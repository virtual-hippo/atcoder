use itertools::*;
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
    }

    let ab = iproduct!(0..n, 0..m).map(|(i, j)| (a[i] + b[j])).collect::<HashSet<usize>>();
    let abc = ab
        .iter()
        .map(|&v| c.iter().map(move |&x| (v + x)))
        .flatten()
        .collect::<HashSet<usize>>();

    for _ in 0..q {
        input! {
            x: usize,
        }

        if abc.contains(&x) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
