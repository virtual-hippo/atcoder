use itertools::*;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u64,
        m: usize,
        ab: [(u64, u64); m],
    }

    let ab = ab.into_iter().sorted_by_key(|&(a, b)| (a - b, a)).collect::<Vec<_>>();

    let mut now = n;
    let mut ans = 0;
    for i in 0..m {
        let (a, b) = ab[i];
        let c = a - b;
        if now < a {
            continue;
        }

        let x = (now - a) / c + 1;
        now -= c * x;
        ans += x;
    }

    println!("{}", ans);
}
