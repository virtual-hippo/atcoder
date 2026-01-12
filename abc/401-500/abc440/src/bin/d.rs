use itertools::Itertools;
use proconio::{fastout, input};
use superslice::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
    }

    let a = a.iter().copied().sorted().collect_vec();

    for _ in 0..q {
        input! {
            x: usize,
            y: usize,
        }

        let si = a.lower_bound(&x);

        let is_ok = |med: i64| -> bool {
            let med = med as usize;
            (a[med] - x) - (med - si) < y
        };

        let ti = binary_search((si as i64 - 1, n as i64), is_ok).1 as usize;
        let ans = x + (y - 1) + ti.saturating_sub(si);
        println!("{}", ans);
    }
}

pub fn binary_search<F: Fn(i64) -> bool>(initial_pos: (i64, i64), is_ok: F) -> (i64, i64) {
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
