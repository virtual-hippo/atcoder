use itertools::*;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let m = s.iter().map(|s| s.len()).max().unwrap();
    for i in 0..n {
        let l = (m - s[i].len()) / 2;
        let c: String = repeat_n('.', l).collect();
        println!("{}{}{}", c, s[i], c);
    }
}
