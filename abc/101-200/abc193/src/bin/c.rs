use proconio::{fastout, input};
use std::collections::*;

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let ng = (2..)
        .take_while(|&a| a * a <= n)
        .fold(HashSet::new(), |mut set, a| {
            (2..).take_while(|&b| a.pow(b) <= n).for_each(|b| {
                set.insert(a.pow(b));
            });
            set
        })
        .len();

    let ans = n - ng;
    println!("{}", ans);
}
