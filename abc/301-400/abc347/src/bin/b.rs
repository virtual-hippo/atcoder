use proconio::{fastout, input};
use rustc_hash::FxHashSet;

#[fastout]
fn main() {
    input! {
        s: String,
    }
    let n = s.len();
    let mut set = FxHashSet::default();
    for i in 0..n {
        for j in i + 1..n + 1 {
            set.insert(&s[i..j]);
        }
    }

    let ans = set.len();
    println!("{}", ans);
}
