use itertools::*;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        x: u64,
        y: u64,
        a: [u64; n],
    }
    let a = a.iter().copied().sorted().collect_vec();
    let w = a[0] * y;

    let d = y - x;
    let ans = (1..n)
        .map(|i| {
            let wi = a[i] * y;
            if (wi - w) % d != 0 {
                None
            } else {
                let c = (wi - w) / d;
                if c <= a[i] {
                    Some(a[i] - c)
                } else {
                    None
                }
            }
        })
        .fold(Some(a[0]), |sum, x| {
            if let (Some(sum), Some(x)) = (sum, x) {
                Some(sum + x)
            } else {
                None
            }
        });

    if let Some(ans) = ans {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}
