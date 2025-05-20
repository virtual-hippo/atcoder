use proconio::{fastout, input};
use std::collections::BTreeSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut b = (0..2001).map(|v| std::cmp::Reverse(v)).collect::<BTreeSet<_>>();
    for i in 0..n {
        b.remove(&std::cmp::Reverse(a[i]));
    }

    let std::cmp::Reverse(v) = b.iter().last().unwrap();

    let ans = *v;
    println!("{}", ans);
}
