use itertools::*;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let ans = a.iter().filter(|&&x| x % k == 0).map(|&x| x / k).join(" ");

    println!("{}", ans);
}
