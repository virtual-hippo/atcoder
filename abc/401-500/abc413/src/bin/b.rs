use proconio::{fastout, input};
use rustc_hash::FxHashSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let mut set = FxHashSet::default();

    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            let t = format!("{}{}", s[i], s[j]);
            set.insert(t);
        }
    }

    let ans = set.len();
    println!("{}", ans);
}
