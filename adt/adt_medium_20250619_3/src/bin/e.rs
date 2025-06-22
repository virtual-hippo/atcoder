use itertools::*;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }

    let a = a.into_iter().sorted().collect::<Vec<_>>();

    let s = std::iter::once(0_i64)
        .chain(a.iter().scan(0_i64, |sum, &x| {
            *sum = *sum + x;
            Some(*sum)
        }))
        .collect::<Vec<_>>();

    let mut ans = 0;
    for i in 0..n - 1 {
        ans += a[i] * (n - (i + 1)) as i64;
        ans += s[n] - s[i + 1];

        let is_ok = |j: usize| a[j] < 100_000_000 - a[i];
        let j = binary_search((i, n), is_ok).1;

        if i < j && j < n && a[i] + a[j] >= 100_000_000 {
            ans -= (n - j) as i64 * 100_000_000;
        }
    }

    println!("{}", ans);
}

pub fn binary_search<F: Fn(usize) -> bool>(initial_pos: (usize, usize), is_ok: F) -> (usize, usize) {
    let mut left = initial_pos.0;
    let mut right = initial_pos.1;

    while left + 1 < right {
        let mid = (left + right) / 2;
        if is_ok(mid) {
            left = mid;
        } else {
            right = mid;
        }
    }

    (left, right)
}
