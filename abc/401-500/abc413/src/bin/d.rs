use itertools::*;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        solve();
    }
}

fn solve() {
    input! {
        n: usize,
        a: [i64; n],
    }

    let b = a.iter().sorted_by_key(|v| v.abs()).map(|&v| v).collect::<Vec<_>>();
    if is_ok(&b) {
        println!("Yes");
        return;
    }

    if b[0].abs() == b[n - 1].abs() {
        let pos = b.iter().filter(|&&v| v > 0).count();
        let neg = n - pos;
        if pos.abs_diff(neg) <= 1 {
            println!("Yes");
            return;
        }
    }

    println!("No");
}

fn is_ok(a: &[i64]) -> bool {
    for i in 2..a.len() {
        if a[i] * a[i - 2] != a[i - 1] * a[i - 1] {
            return false;
        }
    }
    true
}
