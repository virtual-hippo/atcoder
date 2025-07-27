use itertools::*;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        b: [usize; m],
    }
    let a = a.into_iter().sorted().collect::<Vec<_>>();
    let b = b.into_iter().sorted().collect::<Vec<_>>();

    let mut ai = 0;
    let mut ans = 0;
    for bv in b.iter() {
        while ai < n && a[ai] < *bv {
            ai += 1;
        }

        if ai == n {
            println!("-1");
            return;
        }
        ans += a[ai];
        ai += 1;
    }

    println!("{}", ans);
}
