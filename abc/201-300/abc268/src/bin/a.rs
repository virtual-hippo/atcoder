use proconio::{fastout, input};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        a: [usize; 5],
    }
    let mut set = HashSet::new();
    for i in 0..5 {
        set.insert(a[i]);
    }
    println!("{}", set.len());
}
