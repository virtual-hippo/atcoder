use std::usize;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: [usize; n],
    }
    let mut ans = usize::MAX;
    for bit in 0..(1 << n) {
        let mut a = 0;
        let mut b = 0;
        for i in 0..n {
            if bit & (1 << i) != 0 {
                a += k[i];
            } else {
                b += k[i];
            }
        }
        let now = a.max(b);
        ans = ans.min(now);
    }
    println!("{}", ans);
}
