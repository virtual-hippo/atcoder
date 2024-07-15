use proconio::{fastout, input};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
    }
    if a == b {
        println!("{}", -1);
    } else {
        let mut set = HashSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);
        set.remove(&a);
        set.remove(&b);
        for v in set.iter() {
            println!("{}", v)
        }
    }
}
