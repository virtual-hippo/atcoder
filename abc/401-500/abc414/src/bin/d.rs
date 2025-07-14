use itertools::*;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        x: [usize; n],
    }

    let x = x.into_iter().sorted().collect::<Vec<usize>>();
    let d = x.iter().tuple_windows().map(|(a, b)| b - a).sorted().collect::<Vec<usize>>();

    let ans = (0..n - m).map(|i| d[i]).sum::<usize>();
    println!("{}", ans);
}
