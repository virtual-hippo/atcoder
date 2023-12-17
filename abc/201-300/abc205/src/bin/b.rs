use proconio::{fastout, input};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut set = (1..=n).collect::<HashSet<usize>>();
    for i in 0..n {
        set.remove(&a[i]);
    }
    if set.len() == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
