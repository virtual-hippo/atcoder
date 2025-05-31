use itertools::*;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let c = a.iter().unique().sorted().collect::<Vec<_>>();

    let m = c.len();
    println!("{}", m);
    for i in 0..m {
        print!("{} ", c[i]);
    }
}
