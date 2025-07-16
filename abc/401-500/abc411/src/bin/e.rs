use std::vec;

use ac_library::*;
use itertools::*;
use proconio::{fastout, input};

type MInt = ModInt998244353;

#[fastout]
fn main() {
    let m = 6;
    input! {
        n: usize,
        a: [[usize; m]; n],
    }

    let xs = std::iter::once((0, 0))
        .chain(iproduct!(0..n, 0..m).map(|(i, j)| (a[i][j], i)))
        .sorted()
        .collect::<Vec<_>>();

    let mut ans = MInt::new(0);
    let mut counter = vec![0; n];

    let mut prod = MInt::new(1);
    let mut zero = n;

    for ((pre, _), (x, i)) in xs.iter().tuple_windows() {
        let mut p = MInt::new(0);

        if zero == 0 {
            p = prod / MInt::new(m).pow(n as u64);
        }
        ans += (MInt::new(1) - p) * (x - pre);

        if counter[*i] == 0 {
            zero -= 1;
        } else {
            prod /= counter[*i];
        }
        counter[*i] += 1;
        prod *= counter[*i];
    }

    println!("{}", ans);
}
