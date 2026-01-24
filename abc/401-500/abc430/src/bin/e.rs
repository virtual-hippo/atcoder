use ac_library::*;
use proconio::{fastout, input};

// Z-algorithm
fn solve() {
    input! {
        a: String,
        b: String,
    }
    let n = b.len();
    let c = format!("{}${}{}", b, a, a);
    let ret = z_algorithm(&c);

    if let Some(i) = (0..n).find(|&i| ret[i + n + 1] == n) {
        println!("{}", i);
    } else {
        println!("-1");
    }
}

#[fastout]
fn main() {
    input! {
        t: usize,
    }
    (0..t).for_each(|_| solve());
}
