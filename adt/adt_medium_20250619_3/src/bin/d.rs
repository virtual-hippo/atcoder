use proconio::{fastout, input, marker::Chars};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    let n = s.len();

    let mut set = HashSet::new();

    for l in 0..n {
        for r in l..n {
            let sub: String = s[l..=r].iter().collect();
            set.insert(sub);
        }
    }

    println!("{}", set.len());
}
