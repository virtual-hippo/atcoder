use itertools::Itertools;
use proconio::marker::Chars;
use proconio::{fastout, input};
use std::collections::BTreeSet;

#[fastout]
fn main() {
    input! {
        s: Chars,
        k: usize,
    }
    let mut set = BTreeSet::new();
    for vec in (0..s.len()).permutations(s.len()) {
        let string = vec.iter().map(|i| s[*i]).collect::<String>();
        set.insert(string);
    }
    println!("{}", set.iter().nth(k - 1).unwrap());
}
